// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::path::PathBuf;

use cargo_attribution::{
    licenses,
    metadata::{self},
    serialize::SerializeFile,
};
use cargo_metadata::{CargoOpt, MetadataCommand};
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let Args {
        manifest_path,
        current_dir,
        all_features,
        no_default_features,
        features,
        output_dir,
        filter_platform,
        only_normal_dependencies,
    } = Args::parse();

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
        mc.other_options([format!("--filter-platform {platform}")]);
    }

    let metadata = mc.exec()?;
    let dependencies_data = metadata::get_data(metadata, only_normal_dependencies)?;

    cargo_attribution::create_output_dir(&output_dir)?;

    let serialize_file = SerializeFile::new(&dependencies_data);
    serialize_file.create_toml(&output_dir)?;

    licenses::generate_licenses(&dependencies_data, output_dir).await?;

    Ok(())
}

#[derive(Parser)]
#[command(author, version, about, name = "cargo attribution")]
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
