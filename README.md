# cargo-attribution

[![License](https://img.shields.io/badge/license-MPL2.0-blue.svg)](https://www.mozilla.org/en-US/MPL/2.0/)
[![Crates.io](https://img.shields.io/crates/v/cargo-attribution.svg)](https://crates.io/crates/cargo-attribution)
![Minimum Supported Rust Version](https://img.shields.io/badge/rustc-1.74.1+-red)
[![CI](https://github.com/ameknite/cargo-attribution/workflows/CI/badge.svg)](https://github.com/ameknite/cargo-attribution/actions?workflow=CI)

A cargo subcommand to give credit to your dependencies

## Install and Usage

Install cargo-attribution with: `cargo install cargo-attribution`.

Run it in your project directory with: `cargo attribution`.

```sh
A cargo subcommand to give credit to your dependencies

Usage: cargo attribution [OPTIONS]

Options:
      --manifest-path <MANIFEST_PATH>
          Path to the Cargo.toml, [default: ./Cargo.toml]
      --current-dir <CURRENT_DIR>
          Directory of the cargo process, [default: .]
      --output-dir <OUTPUT_DIR>
          Directory of the output files, [default: ./attribution]
  -d, --dependencies-name <DEPENDENCIES_NAME>
          Dependencies file name [default: dependencies]
      --self-name <SELF_NAME>
          Self file name [default: self]
      --all-features
          Activate all available features
      --no-default-features
          Deactivate default features
      --features <FEATURES>
          Select features to activate, e.g. f1,f2,f3
      --filter-platform <FILTER_PLATFORM>
          Filter by target triple, e.g., "wasm32-unknown-unknown"
      --only-normal-dependencies
          Avoid dev, build, and unknown dependencies
  -h, --help
          Print help
  -V, --version
          Print version
```

## Main Features

- Download a general version of the licenses used by your dependencies. They are downloaded from the spdx license-list-data: <https://github.com/spdx/license-list-data>

- Create a `dependencies.toml` file that contains metadata for the project's dependencies, and a `self.toml` file that includes metadata of the project itself, including copyright notices.

Example of a dependency:

```toml
[[dependencies]]
name = "wasm-bindgen"
version = "0.2.91"
description = """
Easy support for interacting between JS and Rust.
"""
license = "MIT OR Apache-2.0"
notices = ["Copyright (c) 2014 Alex Crichton"]
authors = ["The wasm-bindgen Developers"]
repository = "https://github.com/rustwasm/wasm-bindgen"
homepage = "https://rustwasm.github.io/"
```

You can check the [dependencies.toml](./attribution/dependencies.toml) file, [self.toml](./attribution/self.toml) and [licenses](./attribution/licenses/) generated for this project.

## Purpose

This command allows you to comply with licenses that require you to retain the license and copyright notices, such as the MIT license, without the need to include the same license repeatedly.

MIT license extract:

> The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

## LICENSE

cargo-attribution is provided under the MPL v2.0 license. Refer to the [LICENSE](./LICENSE) file for more details.
