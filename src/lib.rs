use std::{fs, path::Path};

pub mod licenses;
pub mod metadata;
pub mod serialize;

pub fn create_output_dir(path: &Path) -> anyhow::Result<()> {
    if path.try_exists()? {
        fs::remove_dir_all(path)?;
    }

    fs::create_dir(path)?;

    Ok(())
}
