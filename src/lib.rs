#[cfg(feature = "python")]
use pyo3::prelude::*;
mod lcdm;

#[cfg(feature = "python")]
#[pyfunction]
pub fn dh_over_rs(z: f64, h0rd: f64, omegam: f64) -> PyResult<f64> {
    Ok(lcdm::dh_over_rs(z, h0rd, omegam))
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn dm_over_rs(z: f64, h0rd: f64, omegam: f64) -> PyResult<f64> {
    Ok(lcdm::dm_over_rs(z, h0rd, omegam))
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn dv_over_rs(z: f64, h0rd: f64, omegam: f64) -> PyResult<f64> {
    Ok(lcdm::dv_over_rs(z, h0rd, omegam))
}

#[cfg(feature = "python")]
#[pyclass(name = "Likelihood")]
struct PyLikelihood {
    inner: lcdm::Likelihood,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyLikelihood {
    #[new]
    fn new(data_file: &str, cov_file: &str) -> PyResult<Self> {
        Ok(Self { inner: lcdm::Likelihood::from_files(data_file, cov_file), })
    }

    fn __call__(&self, h0rd: f64, omegam: f64) -> PyResult<f64> {
        Ok(self.inner.logl(h0rd, omegam))
    }
}

#[cfg(feature = "python")]
#[pymodule]
fn resi(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(dh_over_rs, m)?)?;
    m.add_function(wrap_pyfunction!(dm_over_rs, m)?)?;
    m.add_function(wrap_pyfunction!(dv_over_rs, m)?)?;
    m.add_class::<PyLikelihood>()?;

    Ok(())
}

type Likelihood = lcdm::Likelihood;

// export likelihood to C++ interface, leave out the distances
#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type Likelihood;
        fn create_likelihood(data_file: &str, cov_file: &str) -> Box<Likelihood>;
        fn logl(likelihood: &Likelihood, h0rd: f64, omegam: f64) -> f64;
    }
}

pub fn create_likelihood(data_file: &str, cov_file: &str) -> Box<lcdm::Likelihood> {
    Box::new(lcdm::Likelihood::from_files(data_file, cov_file))
}

pub fn logl(likelihood: &lcdm::Likelihood, h0rd: f64, omegam: f64) -> f64 {
    likelihood.logl(h0rd, omegam)
}
