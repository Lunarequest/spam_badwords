use pyo3::prelude::*;
use std::collections::BTreeSet;

/// checks if word is in wordlist
#[pyfunction]
fn isbad(message: BTreeSet<String>, wordlist: BTreeSet<String>) -> PyResult<bool> {
    if wordlist.is_disjoint(&message) {
        return Ok(false);
    }
    return Ok(true);
}

/// A Python module implemented in Rust.
#[pymodule]
fn spam_badwords(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(isbad, m)?)?;
    Ok(())
}
