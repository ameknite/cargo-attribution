// SPDX-License-Identifier: MPL-2.0
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{fs::File, io::Write, path::Path};

use color_eyre::eyre::Result;
use serde::Serialize;
use taplo::formatter;
use toml_edit::{ArrayOfTables, DocumentMut, Item, Table};

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

    pub fn to_toml(&self) -> Result<String> {
        // Serialize your struct to a TOML string
        let toml_str = toml::to_string(&self)?;

        if self.file_name == "dependencies" {
            return Ok(format_toml(&toml_str));
        }

        // Parse the TOML string into a toml_edit::DocumentMut
        let mut doc = toml_str.parse::<DocumentMut>()?;

        if let Item::ArrayOfTables(array_of_tables) = &mut doc["dependencies"] {
            // Rename the list
            let mut new_list = ArrayOfTables::new();
            std::mem::swap(array_of_tables, &mut new_list);
            doc[&self.file_name] = Item::ArrayOfTables(new_list)
        }

        // Convert the modified document back to a TOML string
        let toml_str = doc.to_string();
        Ok(format_toml(&toml_str))
    }

    pub fn create_toml(&self, output_dir: &Path) -> Result<()> {
        let mut file = File::create(output_dir.join(format!("{}.toml", self.file_name)))?;
        file.write_all(self.to_toml()?.as_bytes())?;
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct SelfSerialized<'a> {
    #[serde(rename = "self")]
    self_crate: &'a DependencyData,
    #[serde(skip)]
    file_name: String,
}

impl<'a> SelfSerialized<'a> {
    pub fn new(self_crate: &'a DependencyData, file_name: String) -> Self {
        Self {
            self_crate,
            file_name,
        }
    }

    pub fn to_toml(&self) -> Result<String> {
        // Serialize your struct to a TOML string
        let toml_str = toml::to_string_pretty(&self)?;

        if self.file_name == "self" {
            return Ok(format_toml(&toml_str));
        }

        // Parse the TOML string into a toml_edit::DocumentMut
        let mut doc = toml_str.parse::<DocumentMut>()?;

        if let Item::Table(table) = &mut doc["self"] {
            // Rename the table
            let mut new_table = Table::new();
            std::mem::swap(table, &mut new_table);
            doc[&self.file_name] = Item::Table(new_table);
            doc.remove("self");
        }

        // Convert the modified document back to a TOML string
        let toml_str = doc.to_string();
        Ok(format_toml(&toml_str))
    }

    pub fn create_toml(&self, output_dir: &Path) -> Result<()> {
        let mut file = File::create(output_dir.join(format!("{}.toml", self.file_name)))?;
        file.write_all(self.to_toml()?.as_bytes())?;
        Ok(())
    }
}

pub fn format_toml(text: &str) -> String {
    formatter::format(text, formatter::Options::default())
}
