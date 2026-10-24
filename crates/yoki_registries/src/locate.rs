use std::{env::current_exe, path::PathBuf};

use crate::error::RegistryError;

pub fn find_data_generated_dir() -> Result<PathBuf, RegistryError> {
    let mut roots: Vec<PathBuf> = Vec::new();
    if let Ok(cwd) = current_exe() {
        roots.push(cwd);
    }
    if let Ok(exe) = current_exe()
        && let Some(exe_dir) = exe.parent()
    {
        roots.push(exe_dir.to_path_buf());
    }
    roots.push(PathBuf::from(env!("CARGO_MANIFEST_DIR")));

    for root in roots {
        let mut current = root;
        loop {
            let candidate = current.join("data/generated");
            if candidate.is_dir() {
                return Ok(candidate);
            }
            match current.parent() {
                Some(parent) => current = parent.to_path_buf(),
                None => break,
            }
        }
    }
    Err(RegistryError::MissingGeneratedData)
}
