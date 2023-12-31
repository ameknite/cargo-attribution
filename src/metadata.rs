// SPDX-License-Identifier: MPL-2.0
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::fs;

use cargo_metadata::{DependencyKind, Metadata, Package};
use color_eyre::{eyre::ContextCompat, Result};
use serde::Serialize;
use spdx::Expression;

#[derive(Debug, Serialize, Clone)]
pub struct DependencyData {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub license: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub notices: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub authors: Vec<String>,
    pub repository: Option<String>,
    pub homepage: Option<String>,
    #[serde(skip_serializing)]
    pub licenses: Vec<String>,
    #[serde(skip_serializing)]
    pub exceptions: Vec<String>,
}

impl DependencyData {
    pub fn new(package: &Package) -> Self {
        Self {
            name: package.name.clone(),
            version: package.version.to_string(),
            authors: package.authors.clone(),
            description: package.description.clone(),
            repository: package.repository.clone(),
            homepage: if package.homepage == package.repository {
                None
            } else {
                package.homepage.clone()
            },
            license: package.license.clone(),
            licenses: Vec::new(),
            notices: Vec::new(),
            exceptions: Vec::new(),
        }
    }
}

impl DependencyData {
    pub fn get_license_notices(&mut self, package: &Package) -> Result<()> {
        let parent = package
            .manifest_path
            .parent()
            .wrap_err("Not manifest parent")?;
        for entry in fs::read_dir(parent)? {
            let entry = entry?;
            let file_path = entry.path();

            if file_path.is_dir() {
                continue;
            }

            let Some(file_name) = file_path.file_name() else {
                continue;
            };

            let Some(file_name) = file_name.to_str() else {
                continue;
            };

            if ["license", "copying"]
                .iter()
                .all(|name| !file_name.to_lowercase().contains(name))
            {
                continue;
            }

            let Ok(license) = fs::read_to_string(file_path) else {
                continue;
            };
            for notice in license
                .lines()
                .filter(|line| {
                    let line = line.trim().to_lowercase();
                    line.starts_with("copyright")
                        && (line.contains("(c)")
                            || line.contains('©')
                            || line.chars().any(|c| c.is_ascii_digit()))
                })
                .map(|line| line.trim())
            {
                self.notices.push(notice.to_string());
            }
        }

        self.notices.sort_unstable();
        self.notices.dedup();

        Ok(())
    }

    pub fn get_parse_licenses(&mut self) -> Result<()> {
        let license_metadata = self.license.clone().wrap_err("License not found")?;

        let expression = match Expression::parse(&license_metadata) {
            Ok(spdx) => spdx,
            Err(_) => {
                let license = license_metadata
                    .split('/')
                    .map(|s| s.trim())
                    .collect::<Vec<_>>()
                    .join(" OR ");
                Expression::parse(&license).unwrap()
            }
        };

        for license_req in expression.requirements().map(|x| x.req.clone()) {
            if let Some(license_id) = license_req.license.id() {
                self.licenses.push(license_id.name.to_owned());
            }

            if let Some(exception_id) = license_req.exception {
                self.exceptions.push(exception_id.name.to_owned());
            }
        }

        self.licenses.sort_unstable();
        self.licenses.dedup();

        Ok(())
    }
}

pub fn get_data(
    metadata: Metadata,
    only_normal_dependencies: bool,
) -> Result<(Vec<DependencyData>, Option<DependencyData>)> {
    let mut dependencies = Vec::with_capacity(metadata.packages.len());
    let mut my_crate = None;

    for package in &metadata.packages {
        let mut dependency = DependencyData::new(package);

        if only_normal_dependencies
            && metadata.root_package().iter().any(|root| {
                root.dependencies
                    .iter()
                    .filter(|dep| dep.kind != DependencyKind::Normal)
                    .any(|dep| dep.name == dependency.name)
            })
        {
            continue;
        }

        let Ok(_) = dependency.get_license_notices(package) else {
            continue;
        };

        let Ok(_) = dependency.get_parse_licenses() else {
            continue;
        };

        if metadata
            .root_package()
            .iter()
            .any(|root| root.id == package.id)
        {
            my_crate = Some(dependency);
        } else {
            dependencies.push(dependency);
        }
    }
    Ok((dependencies, my_crate))
}
