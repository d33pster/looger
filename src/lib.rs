mod loglib_;

use pyo3::prelude::*;

#[pymodule]
#[pyo3(name = "loglib")]
fn loglib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<loglib_::Logger>()?;
    Ok(())
}