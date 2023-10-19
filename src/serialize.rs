// SPDX-License-Identifier: MPL-2.0
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{fs::File, io::Write, path::Path};

use serde::Serialize;

use crate::metadata::DependencyData;

#[derive(Debug, Serialize)]
pub struct DependencySerialized<'a> {
    #[serde(rename = "crate")]
    dependency: &'a [DependencyData],
}

impl<'a> DependencySerialized<'a> {
    pub fn new(dependencies: &'a [DependencyData]) -> Self {
        Self {
            dependency: dependencies,
        }
    }

    pub fn to_toml(&self) -> Result<String, toml::ser::Error> {
        toml::to_string_pretty(&self)
    }

    pub fn create_toml(&self, output_dir: &Path) -> anyhow::Result<()> {
        let mut file = File::create(output_dir.join("crates.toml"))?;
        file.write_all(self.to_toml()?.as_bytes())?;
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct SelfSerialized<'a> {
    #[serde(rename = "self")]
    self_crate: &'a DependencyData,
}

impl<'a> SelfSerialized<'a> {
    pub fn new(self_crate: &'a DependencyData) -> Self {
        Self { self_crate }
    }

    pub fn to_toml(&self) -> Result<String, toml::ser::Error> {
        toml::to_string_pretty(&self)
    }

    pub fn create_toml(&self, output_dir: &Path) -> anyhow::Result<()> {
        let mut file = File::create(output_dir.join("self.toml"))?;
        file.write_all(self.to_toml()?.as_bytes())?;
        Ok(())
    }
}
