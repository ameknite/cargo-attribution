// SPDX-License-Identifier: MPL-2.0
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::path::PathBuf;

use cargo_attribution::{
    licenses,
    metadata::{self},
    serialize::{DependencySerialized, SelfSerialized},
};
use cargo_metadata::{CargoOpt, MetadataCommand};
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let CargoCli::Attribution(Args {
        manifest_path,
        current_dir,
        all_features,
        no_default_features,
        features,
        output_dir,
        filter_platform,
        only_normal_dependencies,
        dependencies_name,
    }) = CargoCli::parse();

    let mut mc = MetadataCommand::new();
    mc.manifest_path(manifest_path);
    mc.current_dir(current_dir);

    if all_features {
        mc.features(CargoOpt::AllFeatures);
    }

    if no_default_features {
        mc.features(CargoOpt::NoDefaultFeatures);
    }

    if let Some(features) = features {
        mc.features(CargoOpt::SomeFeatures(features));
    }

    if let Some(platform) = filter_platform {
        mc.other_options(["--filter-platform".to_owned(), platform]);
    }

    let metadata = mc.exec()?;
    println!("Extracting Metadata");
    let (mut dependencies_data, crate_data) =
        metadata::get_data(metadata, only_normal_dependencies)?;
    println!("Complete Metadata");

    cargo_attribution::create_folder(&output_dir)?;

    let dependencies_file = DependencySerialized::new(&dependencies_data, dependencies_name);
    dependencies_file.create_toml(&output_dir)?;

    if let Some(crate_data) = crate_data {
        let crate_file = SelfSerialized::new(&crate_data);
        crate_file.create_toml(&output_dir)?;
        dependencies_data.push(crate_data);
    }

    licenses::generate_licenses(&dependencies_data, output_dir).await?;
    Ok(())
}

#[derive(clap::Args)]
#[command(author, version, about)]
struct Args {
    /// Path to the Cargo.toml,
    #[arg(long, default_value = "./Cargo.toml")]
    manifest_path: PathBuf,

    /// Directory of the cargo process,
    #[arg(long, default_value = ".")]
    current_dir: PathBuf,

    /// Directory of the output files,
    #[arg(long, default_value = "./attribution")]
    output_dir: PathBuf,

    /// Dependencies file name
    #[arg(short, long, default_value = "dependencies")]
    dependencies_name: String,

    /// Activate all available features
    #[arg(long)]
    all_features: bool,

    /// Deactivate default features
    #[arg(long)]
    no_default_features: bool,

    /// Select features to activate,
    /// e.g. f1,f2,f3
    #[arg(long, value_delimiter = ',')]
    features: Option<Vec<String>>,

    /// Filter by target triple,
    /// e.g., "wasm32-unknown-unknown"
    #[arg(long)]
    filter_platform: Option<String>,

    /// Avoid dev, build, and unknown dependencies
    #[arg(long)]
    only_normal_dependencies: bool,
}

#[derive(Parser)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
enum CargoCli {
    Attribution(Args),
}
