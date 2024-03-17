// SPDX-License-Identifier: MPL-2.0
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use bytes::Bytes;
use color_eyre::Result;
use reqwest::{self, Url};
use std::path::PathBuf;
use tokio::{fs::File, io::AsyncWriteExt, task};

use crate::metadata::DependencyData;

pub struct LicenseData {
    pub name: String,
    pub path: PathBuf,
    pub url: Url,
}

impl LicenseData {
    pub async fn get_license_content(&self) -> Result<Bytes> {
        let content = reqwest::get(self.url.clone()).await?.bytes().await?;
        Ok(content)
    }

    pub async fn create_license_file(&self, content: Bytes) -> Result<()> {
        let mut file = File::create(&self.path).await?;
        file.write_all(&content).await?;
        Ok(())
    }

    pub async fn generate_license(&self) -> Result<()> {
        println!("Downloading {}", self.name);
        let content = self.get_license_content().await?;
        self.create_license_file(content).await?;
        println!("Complete {}", self.name);
        Ok(())
    }
}

pub async fn generate(
    crates_data: &[DependencyData],
    output_dir: PathBuf,
) -> Result<()> {
    let mut licenses = crates_data
        .iter()
        .flat_map(|c| c.licenses.clone())
        .collect::<Vec<_>>();
    licenses.sort_unstable();
    licenses.dedup();

    let licenses_dir = output_dir.join("licenses");
    super::recreate_folder(&licenses_dir)?;

    let mut tasks = Vec::new();
    for license in licenses {
        let license_data = LicenseData {
            path: licenses_dir.join(&license),
            url: Url::parse(&format!(
                "https://raw.githubusercontent.com/spdx/license-list-data/main/text/{license}.txt"
            ))?,
            name: license,
        };

        tasks.push(task::spawn(async move {
            license_data.generate_license().await
        }));
    }

    let mut exceptions = crates_data
        .iter()
        .flat_map(|c| c.exceptions.clone())
        .collect::<Vec<_>>();
    exceptions.sort_unstable();
    exceptions.dedup();

    if !exceptions.is_empty() {
        let exceptions_dir = licenses_dir.join("exceptions");
        super::recreate_folder(&exceptions_dir)?;

        for exception in exceptions {
            let exception_data = LicenseData {
                path: exceptions_dir.join(&exception),
                url: Url::parse(&format!(
                "https://raw.githubusercontent.com/spdx/license-list-data/main/text/{exception}.txt"
            ))?,
                name: exception,
            };

            tasks.push(task::spawn(async move {
                exception_data.generate_license().await
            }));
        }
    }

    for task in tasks {
        task.await??;
    }

    Ok(())
}
