use pyo3::prelude::*;
mod lcdm;

use lcdm::Likelihood;

#[pyfunction]
pub fn dh_over_rs(z: f64, h0rd: f64, omegam: f64) -> PyResult<f64> {
    Ok(lcdm::dh_over_rs(z, h0rd, omegam))
}

#[pyfunction]
pub fn dm_over_rs(z: f64, h0rd: f64, omegam: f64) -> PyResult<f64> {
    Ok(lcdm::dm_over_rs(z, h0rd, omegam))
}

#[pyfunction]
pub fn dv_over_rs(z: f64, h0rd: f64, omegam: f64) -> PyResult<f64> {
    Ok(lcdm::dv_over_rs(z, h0rd, omegam))
}

#[pyclass(name = "Likelihood")]
struct PyLikelihood {
    inner: Likelihood,
}

#[pymethods]
impl PyLikelihood {
    #[new]
    fn new(data_file: &str, cov_file: &str) -> PyResult<Self> {
        Ok(Self { inner: Likelihood::from_files(data_file, cov_file), })
    }

    fn __call__(&self, h0rd: f64, omegam: f64) -> PyResult<f64> {
        Ok(self.inner.logl(h0rd, omegam))
    }
}

#[pymodule]
fn deri(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(dh_over_rs, m)?)?;
    m.add_function(wrap_pyfunction!(dm_over_rs, m)?)?;
    m.add_function(wrap_pyfunction!(dv_over_rs, m)?)?;
    m.add_class::<PyLikelihood>()?;

    Ok(())
}
