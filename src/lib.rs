use pyo3::prelude::*;
use pyo3::types::PyList;
use regex::Regex;
use std::borrow::Cow;
use lazy_static::lazy_static;

#[pyfunction]
fn print_strings(input_list: &PyList) -> PyResult<()> {
    for item in input_list.iter() {
        let input: String = item.extract()?;
        eprint!("{}", input);
    }

    Ok(())
}

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"^([+\- ]).*$").unwrap();
}

// Colour strings using regex
#[pyfunction]
fn colour_strings_regex(input_list: &PyList) -> PyResult<()> {
    let green_ansi = "\x1b[32m"; // ANSI code for green
    let red_ansi = "\x1b[31m"; // ANSI code for red
    let white_ansi = "\x1b[37m"; // ANSI code for white

    for item in input_list.iter() {
        let input: String = item.extract()?;
        let input_cow = Cow::Borrowed(input.as_str());
        let mut last_match = 0;

        if REGEX.is_match(&input) {
            let matches = REGEX.find_iter(&input);

            for mat in matches {
                let (begin, end) = (mat.start(), mat.end());
                eprint!("{}", &input[last_match..begin]);

                let prefix = &input[begin..begin+1];
                let line = &input[begin..end];
                match prefix {
                    "+" => {
                        eprint!("{}", green_ansi);
                        eprint!("{}", line);
                        eprint!("{}", white_ansi);
                    },
                    "-" => {
                        eprint!("{}", red_ansi);
                        eprint!("{}", line);
                        eprint!("{}", white_ansi);
                    },
                    " " => {
                        eprint!("{}", white_ansi);
                        eprint!("{}", line);
                        eprint!("{}", white_ansi);
                    },
                    _ => eprint!("{}", line),
                };

                last_match = end;
            }

            eprint!("{}", &input[last_match..]);
        } else {
            eprint!("{}", input_cow.into_owned());
        }
    }

    Ok(())
}

// Colour strings without using regex
#[pyfunction]
fn colour_strings(input_list: &PyList) -> PyResult<()> {
    let green_ansi = "\x1b[32m"; // ANSI code for green
    let red_ansi = "\x1b[31m"; // ANSI code for red
    let white_ansi = "\x1b[37m"; // ANSI code for white

    for item in input_list.iter() {
        let input: String = item.extract()?;
        
        for line in input.lines() {
            match line.chars().next() {
                Some('+') => {
                    eprint!("{}{}{}", green_ansi, line, white_ansi);
                },
                Some('-') => {
                    eprint!("{}{}{}", red_ansi, line, white_ansi);
                },
                Some(' ') => {
                    eprint!("{}{}{}", white_ansi, line, white_ansi);
                },
                _ => eprint!("{}", line),
            }
        }
    }

    Ok(())
}

#[pymodule]
fn fastrs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(print_strings, m)?)?;
    m.add_function(wrap_pyfunction!(colour_strings_regex, m)?)?;
    m.add_function(wrap_pyfunction!(colour_strings, m)?)?;
    Ok(())
}
