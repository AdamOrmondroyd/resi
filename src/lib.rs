use pyo3::prelude::*;
mod lcdm;


#[pyfunction]
pub fn dh_over_rs(z: f64, h0rd: f64, omegam: f64) -> PyResult<f64> {
    Ok(lcdm::dh_over_rs(z, h0rd, omegam))
}

#[pyfunction]
pub fn dm_over_rs(z: f64, h0rd: f64, omegam: f64) -> PyResult<f64> {
    Ok(lcdm::dm_over_rs(z, h0rd, omegam))
}

#[pymodule]
fn deri(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(dh_over_rs, m)?)?;
    m.add_function(wrap_pyfunction!(dm_over_rs, m)?)?;

    Ok(())
}
