
use std::{fs, path::PathBuf};

use anyhow::{bail, Result};

pub fn read_source_file(input: &str) -> Result<String> {
    let path = PathBuf::from(input);
    if !path.exists() {
        bail!("Input file does not exist: {:?}", path);
    }

    let content = fs::read_to_string(&path)?;
    Ok(content)
}