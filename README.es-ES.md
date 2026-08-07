

# cargo-attribution

[![License](https://img.shields.io/badge/license-MPL2.0-blue.svg)](https://www.mozilla.org/en-US/MPL/2.0/)
[![Crates.io](https://img.shields.io/crates/v/cargo-attribution.svg)](https://crates.io/crates/cargo-attribution)
![Minimum Supported Rust Version](https://img.shields.io/badge/rustc-1.81.0+-red)
[![CI](https://github.com/ameknite/cargo-attribution/workflows/CI/badge.svg)](https://github.com/ameknite/cargo-attribution/actions?workflow=CI)

```sh
Un subcomando de cargo para dar crédito a tus dependencias

Uso: cargo attribution [OPCIONES]

Opciones:
      --manifest-path <MANIFEST_PATH>
          Ruta al Cargo.toml, [predeterminado: ./Cargo.toml]
      --current-dir <CURRENT_DIR>
          Directorio del proceso de cargo, [predeterminado: .]
      --output-dir <OUTPUT_DIR>
          Directorio de los archivos de salida, [predeterminado: ./attribution]
  -d, --dependencies-name <DEPENDENCIES_NAME>
          Nombre del archivo de dependencias [predeterminado: dependencies]
      --self-name <SELF_NAME>
          Nombre del archivo self [predeterminado: self]
      --all-features
          Activar todas las características disponibles
      --no-default-features
          Desactivar las características predeterminadas
      --features <FEATURES>
          Seleccionar características para activar, p. ej. f1,f2,f3
      --filter-platform <FILTER_PLATFORM>
          Filtrar por triplete objetivo, p. ej. "wasm32-unknown-unknown"
      --only-normal-dependencies
          Evitar dependencias de desarrollo, compilación y desconocidas
  -h, --help
          Imprimir ayuda
  -V, --version
          Imprimir versión
```

## Instalación

### Cargo [install](https://doc.rust-lang.org/cargo/commands/cargo-install.html)

Compila el crate tú mismo con:

```rust
cargo install cargo-attribution
```

### Cargo [binstall](https://github.com/cargo-bins/cargo-binstall)

Instala una versión binaria:

```rust
cargo binstall cargo-attribution
```

## Características principales

- Descarga una versión general de las licencias utilizadas por tus dependencias. Se descargan desde la lista de licencias SPDX: <https://github.com/spdx/license-list-data>

- Crea un archivo `dependencies.toml` que contiene los metadatos de las dependencias del proyecto, y un archivo `self.toml` que incluye los metadatos del propio proyecto, incluyendo los avisos de derechos de autor.

Ejemplo de una dependencia:

```toml
[[dependencies]]
name = "wasi"
version = "0.11.0+wasi-snapshot-preview1"
description = "Experimental WASI API bindings for Rust"
license = "Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT"
authors = ["The Cranelift Project Developers"]
repository = "https://github.com/bytecodealliance/wasi"
```

Puedes consultar el archivo [dependencies.toml](./attribution/dependencies.toml), [self.toml](./attribution/self.toml) y [licenses](./attribution/licenses/) generados para este proyecto.

## Propósito

Este comando te permite cumplir con licencias que requieren conservar los avisos de licencia y derechos de autor, como la licencia MIT, sin necesidad de incluir la misma licencia repetidamente.

Extracto de la licencia MIT:

> El aviso de derechos de autor anterior y este aviso de permiso deben incluirse en todas las copias o porciones sustanciales del Software.

## LICENCIA

cargo-attribution se proporciona bajo la licencia MPL v2.0. Consulta el archivo [LICENSE](./LICENSE) para más detalles.
