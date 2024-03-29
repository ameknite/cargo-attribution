// SPDX-License-Identifier: MPL-2.0
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::fs;

use cargo_metadata::{DependencyKind, Metadata, Package};
use color_eyre::Result;
use regex::Regex;
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
    pub fn get_license_notices(&mut self, package: &Package) {
        let re = Regex::new(r"\d{4,}").unwrap();

        let Some(parent) = package.manifest_path.parent() else {
            return;
        };
        let Ok(entry) = fs::read_dir(parent) else {
            return;
        };
        for entry in entry {
            let Ok(entry) = entry else {
                return;
            };
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
                    line.contains("copyright")
                        && (line.contains("(c)")
                            || line.contains('©')
                            || re.is_match(&line))
                })
                .map(str::trim)
            {
                self.notices.push(notice.to_string());
            }
        }

        self.notices.sort_unstable();
        self.notices.dedup();
    }

    pub fn get_parse_licenses(&mut self) {
        let Some(license_metadata) = self.license.as_ref() else {
            return;
        };

        let expression = if let Ok(spdx) = Expression::parse(license_metadata) {
            spdx
        } else {
            let license = license_metadata
                .split('/')
                .map(str::trim)
                .collect::<Vec<_>>()
                .join(" OR ");
            Expression::parse(&license).unwrap()
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
    }
}

pub fn get_data(
    metadata: &Metadata,
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

        dependency.get_license_notices(package);

        dependency.get_parse_licenses();

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
