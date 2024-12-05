use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use sha1::{Sha1, Digest};
use std::path::Path;
use std::collections::HashSet;
use std::fs;

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_hash, m)?)?;
    m.add_function(wrap_pyfunction!(compare_hash, m)?)?;
    m.add_function(wrap_pyfunction!(get_all_hashes, m)?)?;
    Ok(())
}

#[pyfunction]
fn get_hash(file: &str) -> PyResult<String> {
    let data = fs::read(file).map_err(|e| {
        pyo3::exceptions::PyIOError::new_err(format!("Failed to read file: {}", e))
    })?;
    
    let hash = Sha1::digest(&data);
    Ok(format!("{:x}", hash))
}

#[pyfunction]
fn compare_hash(file: &str, previous_hash: &str) -> PyResult<bool> {
    let current_hash = get_hash(file)?;
    Ok(current_hash == previous_hash)
}

#[pyfunction]
fn get_all_hashes(folder: &str) -> PyResult<HashSet<String>> {
    let path = Path::new(folder);
    if !path.is_dir() {
        return Err(pyo3::exceptions::PyValueError::new_err("Provided path is not a directory"));
    }

    let mut hash_set = HashSet::new();
    for entry in fs::read_dir(path).map_err(|e| {
        pyo3::exceptions::PyIOError::new_err(format!("Failed to read directory: {}", e))
    })? {
        let entry = entry.map_err(|e| {
            pyo3::exceptions::PyIOError::new_err(format!("Failed to access directory: {}", e))
        })?;
        
        if entry.path().is_file() {
            let file_path = entry.path().to_string_lossy().into_owned();
            let file_hash = get_hash(&file_path)?;
            hash_set.insert(file_hash);
        }
    }
    Ok(hash_set)
}
