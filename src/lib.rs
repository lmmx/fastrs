use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::exceptions;

#[pyfunction]
fn join_strings(py: Python, string_list: &PyList, separator: Option<String>) -> PyResult<String> {
    let sep = separator.unwrap_or_else(|| "".to_string());
    let mut strings = Vec::new();
    
    for item in string_list.iter() {
        match item.extract::<String>() {
            Ok(s) => strings.push(s),
            Err(_) => return Err(exceptions::PyValueError::new_err("List items must be strings")),
        }
    }

    Ok(strings.join(&sep))
}

#[pymodule]
fn hellorust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(join_strings, m)?)?;
    Ok(())
}
