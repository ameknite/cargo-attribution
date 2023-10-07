# cargo-attribution

[![License](https://img.shields.io/badge/license-MPL2.0-blue.svg)](https://www.mozilla.org/en-US/MPL/2.0/)
[![Crates.io](https://img.shields.io/crates/v/cargo-attribution.svg)](https://crates.io/crates/cargo-attribution)
![Minimum Supported Rust Version](https://img.shields.io/badge/rustc-1.66.1+-red)
[![CI](https://github.com/ameknite/cargo-attribution/workflows/CI/badge.svg)](https://github.com/ameknite/cargo-attribution/actions?workflow=CI)

A cargo subcommand to give credit to your dependencies

## Install and Usage

Install cargo-attribution with: `cargo install cargo-attribution`.

Run it in your project directory with: `cargo attribution`.

```bash
Usage: cargo-attribution [--manifest-path <manifest-path>] [--current-dir <current-dir>] [--output-dir <output-dir>] [--all-features] [--no-default-features] [--features <features>] [--filter-platform <filter-platform>] [--only-normal-dependencies]

A cargo subcommand to give credit to your dependencies

Options:
  --manifest-path   path to the Cargo.toml, default to: ./Cargo.toml
  --current-dir     directory of the cargo process, default to current dir: .
  --output-dir      directory of the output files, default to: ./attribution
  --all-features    activate all available features
  --no-default-features
                    deactivate default features
  --features        select features to activate, e.g. "f1 f2 f3"
  --filter-platform filter by target triple, e.g. "wasm32-unknown-unknown"
  --only-normal-dependencies
                    avoid dev, build and unknown dependencies
  --help            display usage information
```

## Main Features

- Download a general version of the licenses used by your dependencies. They are downloaded from the spdx license-list-data: <https://github.com/spdx/license-list-data>

- Create a "dependencies.toml" file containing the dependencies metadata used in your project. Including copyright notices.

Example of a dependency:

```toml
[[dependency]]
name = "wasm-bindgen"
version = "0.2.87"
description = """
Easy support for interacting between JS and Rust.
"""
license = "MIT/Apache-2.0"
notices = ["Copyright (c) 2014 Alex Crichton"]
authors = ["The wasm-bindgen Developers"]
repository = "https://github.com/rustwasm/wasm-bindgen"
homepage = "https://rustwasm.github.io/"

```

You can check an example of the files generated in the [attribution folder](./attribution/) of this repository.

## Purpose

This command allows you to comply with licenses that require you to retain the license and copyright notices, such as the MIT license, without the need to include the same license repeatedly.

MIT license extract:

> The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.


## LICENSE

cargo-attribution is provided under the MPL v2.0 license. Refer to the [LICENSE](./LICENSE) file for more details.
