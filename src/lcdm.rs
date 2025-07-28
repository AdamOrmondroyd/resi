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
