use pyo3::prelude::*;
use lichen_core::extract_links as extract_links_core;

#[pyfunction]
fn extract_links(html: &str, base_url: &str) -> PyResult<Vec<String>> {
    extract_links_core(html, base_url).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e))
    })
}

#[pymodule]
fn lichenpy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(extract_links, m)?)?;
    Ok(())
}
