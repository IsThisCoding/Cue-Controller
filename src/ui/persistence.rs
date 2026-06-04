use crate::common::workspace::Workspace;
use std::{
    fs::{self},
    io::ErrorKind,
    path::Path,
};

#[derive(Debug)]
pub enum SaveError {
    AlreadyExists,
    Io(std::io::Error),
    Serialization(serde_json::Error),
}

#[derive(Debug)]
pub enum LoadError {
    NotFound,
    Io(std::io::Error),
    Deserialization(serde_json::Error),
}

pub fn save_workspace(
    workspace: &Workspace,
    filepath: &str,
    overwrite: bool,
) -> Result<(), SaveError> {
    let path = Path::new(&filepath);

    if path.exists() && !overwrite {
        return Err(SaveError::AlreadyExists);
    }

    let json_string = serde_json::to_string_pretty(workspace).map_err(SaveError::Serialization)?;
    let tmp_path = format!("{}.tmp", filepath);
    fs::write(&tmp_path, json_string).map_err(SaveError::Io)?;
    fs::rename(&tmp_path, filepath).map_err(SaveError::Io)?;

    Ok(())
}

pub fn load_workspace(filepath: &str) -> Result<Workspace, LoadError> {
    let path = Path::new(filepath);

    let json_string = fs::read_to_string(path).map_err(|err| match err.kind() {
        ErrorKind::NotFound => LoadError::NotFound,
        _ => LoadError::Io(err),
    })?;

    let workspace: Workspace =
        serde_json::from_str(&json_string).map_err(LoadError::Deserialization)?;
    Ok(workspace)
}
