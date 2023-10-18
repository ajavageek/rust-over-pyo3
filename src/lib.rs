use num_complex::Complex;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn rust_over_pyo3(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(compute, m)?)?;
    Ok(())
}

#[pyfunction]
fn compute(command: &str, a: Complex<f64>, b: Complex<f64>) -> PyResult<Complex<f64>> {
    match command {
        "add" => Ok(a + b),
        "sub" => Ok(a - b),
        "mul" => Ok(a * b),
        _ => Err(PyValueError::new_err("Unknown command")),
    }
}
