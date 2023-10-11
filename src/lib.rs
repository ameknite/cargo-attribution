// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{fs, path::Path};

pub mod licenses;
pub mod metadata;
pub mod serialize;

pub fn recreate_folder(path: &Path) -> anyhow::Result<()> {
    if path.try_exists()? {
        fs::remove_dir_all(path)?;
    }

    fs::create_dir(path)?;

    Ok(())
}

pub fn create_folder(path: &Path) -> anyhow::Result<()> {
    if path.try_exists()? {
        return Ok(());
    }

    fs::create_dir(path)?;
    Ok(())
}
