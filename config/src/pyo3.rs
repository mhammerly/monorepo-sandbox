use pyo3::prelude::*;

#[pyfunction]
pub fn container_repo() -> Option<String> {
    rust_config::container_repo()
}

#[pyfunction]
pub fn gcp_project_id() -> Option<String> {
    rust_config::gcp_project_id()
}

#[pyfunction]
pub fn gcp_topic_id() -> Option<String> {
    rust_config::gcp_topic_id()
}

#[pyfunction]
pub fn gcp_schema_id() -> Option<String> {
    rust_config::gcp_schema_id()
}

#[pyfunction]
pub fn gcp_subscription_id() -> Option<String> {
    rust_config::gcp_subscription_id()
}

#[pyfunction]
pub fn using_pubsub_emulator() -> bool {
    rust_config::using_pubsub_emulator()
}

#[pymodule]
pub fn py_config(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(container_repo, m)?)?;
    m.add_function(wrap_pyfunction!(gcp_project_id, m)?)?;
    m.add_function(wrap_pyfunction!(gcp_topic_id, m)?)?;
    m.add_function(wrap_pyfunction!(gcp_schema_id, m)?)?;
    m.add_function(wrap_pyfunction!(gcp_subscription_id, m)?)?;
    m.add_function(wrap_pyfunction!(using_pubsub_emulator, m)?)?;
    Ok(())
}
