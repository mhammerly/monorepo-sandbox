use pyo3::prelude::*;

#[pyfunction]
fn foo() -> PyResult<String> {
    Ok("foo and bar and baz and qux".to_string())
}

#[pymodule]
fn return_foo(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(foo, m)?)?;
    Ok(())
}
