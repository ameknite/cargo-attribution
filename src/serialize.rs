// SPDX-License-Identifier: MPL-2.0
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{fs::File, io::Write, path::Path};

use serde::Serialize;
use toml_edit::{ArrayOfTables, Document, Item};

use crate::metadata::DependencyData;

#[derive(Debug, Serialize)]
pub struct DependencySerialized<'a> {
    dependencies: &'a [DependencyData],
    #[serde(skip)]
    file_name: String,
}

impl<'a> DependencySerialized<'a> {
    pub fn new(dependencies: &'a [DependencyData], file_name: String) -> Self {
        Self {
            dependencies,
            file_name,
        }
    }

    pub fn to_toml(&self) -> anyhow::Result<String> {
        // Serialize your struct to a TOML string
        let mut toml_str = toml::to_string_pretty(&self)?;

        if self.file_name != "dependencies" {
            // Parse the TOML string into a toml_edit::Document
            let mut doc = toml_str.parse::<Document>()?;

            if let Item::ArrayOfTables(array_of_tables) = &mut doc["dependencies"] {
                // Rename the list
                let mut new_list = ArrayOfTables::new();
                std::mem::swap(array_of_tables, &mut new_list);
                doc[&self.file_name] = Item::ArrayOfTables(new_list)
            }

            // Convert the modified document back to a TOML string
            toml_str = doc.to_string();
        }

        Ok(toml_str)
    }

    pub fn create_toml(&self, output_dir: &Path) -> anyhow::Result<()> {
        let mut file = File::create(output_dir.join(format!("{}.toml", self.file_name)))?;
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
