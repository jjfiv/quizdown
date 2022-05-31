use pyo3::exceptions::PyValueError;
use pyo3::wrap_pyfunction;
use quizdown_lib as qd;

use pyo3::prelude::*;
use pyo3::PyErr;

#[pyfunction]
pub fn default_config() -> PyResult<String> {
    Ok(serde_json::to_string(&qd::Config::default()).unwrap())
}

#[pyfunction]
pub fn available_themes() -> PyResult<Vec<String>> {
    Ok(qd::list_themes())
}

fn stringify_err<E: std::error::Error>(context: &str, e: E) -> PyErr {
    PyValueError::new_err(format!("{}: {:?}", context, e))
}

#[pyfunction]
pub fn try_parse_quizdown(text: &str, name: &str, format: &str, config: &str) -> PyResult<String> {
    let format: qd::OutputFormat =
        serde_json::from_str(format).map_err(|e| stringify_err("format invalid", e))?;
    let config: qd::Config =
        serde_json::from_str(config).map_err(|e| stringify_err("config invalid", e))?;
    let parsed = qd::process_questions_str(text, Some(config))
        .map_err(|e| stringify_err("Parsing Error", e))?;
    Ok(format
        .render(name, &parsed)
        .map_err(|e| stringify_err("Rendering Error", e))?)
}

#[pymodule]
pub fn quizdown(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(available_themes, m)?)?;
    m.add_function(wrap_pyfunction!(default_config, m)?)?;
    m.add_function(wrap_pyfunction!(try_parse_quizdown, m)?)?;
    Ok(())
}
