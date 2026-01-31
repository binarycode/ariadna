## ADDED Requirements

### Requirement: ESP32-S3 target configuration

The project SHALL include `.cargo/config.toml` with Xtensa ESP32-S3 as the default target for the ariadna crate.

#### Scenario: Correct target architecture

- **WHEN** running `cargo build -p ariadna --release`
- **THEN** the build targets `xtensa-esp32s3-espidf`

### Requirement: Binary output compatible with espflash

The ariadna build SHALL produce a binary that can be flashed using `espflash`.

#### Scenario: Flash to device

- **WHEN** running `espflash flash target/xtensa-esp32s3-espidf/release/ariadna`
- **THEN** the binary uploads successfully to connected ESP32-S3 device

### Requirement: Rust toolchain specification

The project SHALL include `rust-toolchain.toml` specifying the esp channel for Xtensa support.

#### Scenario: Correct toolchain selected

- **WHEN** running `rustup show` in the project directory
- **THEN** the active toolchain is the esp toolchain with Xtensa support

### Requirement: ESP-IDF configuration

The ariadna crate SHALL include base ESP-IDF configuration via `sdkconfig.defaults`.

#### Scenario: ESP-IDF settings applied

- **WHEN** building the ariadna crate
- **THEN** ESP-IDF uses settings from `sdkconfig.defaults`
