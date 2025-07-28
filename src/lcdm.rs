use nalgebra::{DMatrix, DVector};
use std::fs;
// speed of light in km/s
const C: f64 = 299792.458;

// cumulative trapezoidal integration (takes vectors of y and x values)
fn cumulative_trapezoid(y: &[f64], x: &[f64]) -> Vec<f64> {
    let mut integral = vec![0.0; y.len()];
    for i in 1..y.len() {
        integral[i] = integral[i - 1] + 0.5 * (y[i] + y[i - 1]) * (x[i] - x[i - 1]);
    }
    integral
}

fn one_over_h(z: f64, omegam: f64) -> f64 {
    let h = (omegam * (1.0 + z).powi(3) + (1.0 - omegam)).sqrt();
    1.0 / h
}

pub fn dh_over_rs(z: f64, h0rd: f64, omegam: f64) -> f64 {
    C / h0rd * one_over_h(z, omegam)
}

pub fn dm_over_rs(z: f64, h0rd: f64, omegam: f64) -> f64 {
    // dm/rs = ∫dh/rs dz
    // number of points for integration
    let n = 1000;
    // create a vector of z values
    let z_values: Vec<f64> = (0..=n).map(|i| i as f64 * z / n as f64).collect();
    // create a vector of dh/rs values
    let one_over_h_values: Vec<f64> = z_values.iter().map(|&z| one_over_h(z, omegam)).collect();
    // perform cumulative trapezoidal integration
    let integral = cumulative_trapezoid(&one_over_h_values, &z_values);
    // return the last value of the integral, which is dm/rs at z
    integral.last().cloned().unwrap_or(0.0) * C / h0rd
}

pub fn dv_over_rs(z: f64, h0rd: f64, omegam: f64) -> f64 {
    (z * dm_over_rs(z, h0rd, omegam).powi(2) * dh_over_rs(z, h0rd, omegam)).cbrt()
}


fn parse_covariance_matrix(filename: &str) -> DMatrix<f64> {
    let content = fs::read_to_string(filename).expect("Failed to read the covariance matrix file");
    let lines: Vec<&str> = content.lines().collect();
    let n = lines.len();
    let mut data = Vec::new();

    for line in lines {
        let row: Result<Vec<f64>, _> = line.split_whitespace().map(|s| s.parse::<f64>()).collect();
        data.extend(row.expect("Failed to parse a row of the covariance matrix"));
    }
    DMatrix::from_vec(n, n, data)
}

fn parse_desidr2_data(filename: &str) -> Vec<DesiData> {
    let content = fs::read_to_string(filename).expect("Failed to read the DESI DR2 data file");
    let mut data = Vec::new();

    for line in content.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 {
            continue; // Skip lines that do not have enough data
        }
        let z = parts[0].parse::<f64>().expect("Failed to parse redshift");
        let value = parts[1].parse::<f64>().expect("Failed to parse value");
        let quantity = parts[2].to_string();
        data.push(DesiData { z, value, quantity });
    }
    data
}

struct DesiData {
    z: f64,
    value: f64,
    quantity: String,
}

pub
struct Likelihood {
    data: Vec<DesiData>,
    invcov: DMatrix<f64>,
    lognorm: f64,
}

impl Likelihood {
    fn new(data: Vec<DesiData>, cov: DMatrix<f64>) -> Self {
        let log_det_cov = cov.determinant().ln();
        let invcov = cov
            .try_inverse()
            .expect("Covariance matrix is not invertible");
        let n = data.len() as f64;
        let lognorm = -(n * (2.0 * std::f64::consts::PI).ln() + log_det_cov) / 2.0;
        Self { data, invcov, lognorm }
    }

    pub fn from_files(data_file: &str, cov_file: &str) -> Self {
        let data = parse_desidr2_data(data_file);
        let cov = parse_covariance_matrix(cov_file);
        Self::new(data, cov)
    }

    fn compute_residuals(&self, h0rd: f64, omegam: f64) -> DVector<f64> {
        let residuals: Vec<f64> = self
            .data
            .iter()
            .map(|data| {
                let theory = match data.quantity.as_str() {
                    "DH_over_rs" => dh_over_rs(data.z, h0rd, omegam),
                    "DM_over_rs" => dm_over_rs(data.z, h0rd, omegam),
                    "DV_over_rs" => dv_over_rs(data.z, h0rd, omegam),
                    _ => panic!("Unknown quantity: {}", data.quantity),
                };
                theory - data.value
            })
            .collect();
        DVector::from_vec(residuals)
    }

    pub fn logl(&self, h0rd: f64, omegam: f64) -> f64 {
        let residuals = self.compute_residuals(h0rd, omegam);
        let chi2 = residuals.transpose() * &self.invcov * residuals;
        -0.5 * chi2[(0, 0)] + self.lognorm
    }
}
