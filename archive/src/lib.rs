use web_archive::blocking;

pub async fn async_archive(url: &str) -> String {
    let archived_page = web_archive::archive(url, Default::default()).await.unwrap();

    archived_page.embed_resources()
}

pub fn archive(url: &str) -> String {
    let archived_page = blocking::archive(url, Default::default()).unwrap();

    archived_page.embed_resources()
}

#[cfg(feature = "pyo3")]
mod pyo3 {
    use pyo3::prelude::*;

    #[pyfunction]
    pub fn archive(url: &str) -> PyResult<String> {
        Ok(super::archive(url))
    }

    #[pymodule]
    pub fn pyo3_archive(m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add_function(wrap_pyfunction!(archive, m)?)?;
        Ok(())
    }
}
