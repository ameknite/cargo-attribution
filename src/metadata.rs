use std::fs;

use anyhow::Context;
use cargo_metadata::{DependencyKind, Metadata, Package};
use serde::Serialize;
use spdx::Expression;
use walkdir::WalkDir;

#[derive(Debug, Serialize)]
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
        }
    }
}

impl DependencyData {
    pub fn get_license_notices(&mut self, package: &Package) -> anyhow::Result<()> {
        let parent = package
            .manifest_path
            .parent()
            .context("Not manifest parent")?;
        for entry in WalkDir::new(parent) {
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

            if !file_name.to_lowercase().contains("license") {
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
                        && (line.contains("(c)") || line.chars().any(|c| c.is_ascii_digit()))
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

    pub fn get_parse_licenses(&mut self) -> anyhow::Result<()> {
        let license_metadata = self.license.clone().context("License not found")?;

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
                self.licenses.push(exception_id.name.to_owned());
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
) -> anyhow::Result<Vec<DependencyData>> {
    let mut dependencies = Vec::with_capacity(metadata.packages.len());

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

        dependencies.push(dependency);
    }
    Ok(dependencies)
}
