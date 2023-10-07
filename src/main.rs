// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::path::PathBuf;

use argh::FromArgs;
use cargo_attribution::{
    licenses,
    metadata::{self},
    serialize::SerializeFile,
};
use cargo_metadata::{CargoOpt, MetadataCommand};

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
    } = argh::from_env();

    let mut mc = MetadataCommand::new();
    mc.manifest_path(manifest_path);
    mc.current_dir(current_dir);

    if all_features {
        mc.features(CargoOpt::AllFeatures);
    }

    if no_default_features {
        mc.features(CargoOpt::NoDefaultFeatures);
    }

    if !features.is_empty() {
        mc.features(CargoOpt::SomeFeatures(
            features.split_whitespace().map(|f| f.to_owned()).collect(),
        ));
    }

    if !filter_platform.is_empty() {
        mc.other_options([format!("--filter-platform {filter_platform}")]);
    }

    let metadata = mc.exec()?;
    let dependencies_data = metadata::get_data(metadata, only_normal_dependencies)?;

    cargo_attribution::create_output_dir(&output_dir)?;

    let serialize_file = SerializeFile::new(&dependencies_data);
    serialize_file.create_toml(&output_dir)?;

    licenses::generate_licenses(&dependencies_data, output_dir).await?;

    Ok(())
}

#[derive(FromArgs)]
/// A cargo subcommand to give credit to your dependencies
struct Args {
    /// path to the Cargo.toml,
    /// default to: ./Cargo.toml
    #[argh(option, default = "PathBuf::from(\"./Cargo.toml\")")]
    manifest_path: PathBuf,

    /// directory of the cargo process,
    /// default to current dir: .
    #[argh(option, default = "PathBuf::from(\".\")")]
    current_dir: PathBuf,

    /// directory of the output files,
    /// default to: ./attribution
    #[argh(option, default = "PathBuf::from(\"./attribution\")")]
    output_dir: PathBuf,

    /// activate all available features
    #[argh(switch)]
    all_features: bool,

    /// deactivate default features
    #[argh(switch)]
    no_default_features: bool,

    /// select features to activate,
    /// e.g. "f1 f2 f3"
    #[argh(option, default = "String::new()")]
    features: String,

    /// filter by target triple,
    /// e.g. "wasm32-unknown-unknown"
    #[argh(option, default = "String::new()")]
    filter_platform: String,

    /// avoid dev, build and unknown dependencies
    #[argh(switch)]
    only_normal_dependencies: bool,
}
