use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::exceptions;

#[pyfunction]
fn join_strings(_py: Python, string_list: &PyList, separator: Option<String>) -> PyResult<String> {
    let sep = separator.unwrap_or_else(|| "".to_string());
    // Preallocate memory for the final string: calculate total length of all strings combined
    let number_of_separators = string_list.len().saturating_sub(1);
    let mut total_length = sep.len() * number_of_separators;

    for item in string_list.iter() {
        match item.extract::<String>() {
            Ok(s) => total_length += s.len(),
            Err(_) => return Err(exceptions::PyValueError::new_err("List items must be strings")),
        }
    }

    // Then create a new string with this capacity (avoids reallocating memory while appending)
    let mut result = String::with_capacity(total_length);

    for (i, item) in string_list.iter().enumerate() {
        let string = item.extract::<String>().unwrap(); // Safe to unwrap because we've already checked
        result.push_str(&string);

        // Add separator only if it's not the last element
        if i < number_of_separators {
            result.push_str(&sep);
        }
    }

    Ok(result)
}

#[pymodule]
fn fastrs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(join_strings, m)?)?;
    Ok(())
}
