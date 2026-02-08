## Why

We're building a GPS navigator device using LilyGO T-Display S3 Pro and Beitian BE-880 GPS module. We need a proper Rust project structure that supports multiple programs in a single repository, with the main embedded application ("ariadna") targeting ESP32-S3, while allowing utilities to run on the host machine.

## What Changes

- Initialize Cargo workspace with multi-crate structure
- Create `ariadna` crate as the main ESP32-S3 embedded application
- Configure ESP-IDF toolchain and build settings for proper binary output
- Set up architecture separation so tests can run locally on host machine
- Add `.cargo/config.toml` with ESP32-S3 target configuration
- Create base project files (README, gitignore, rust-toolchain)

## Capabilities

### New Capabilities

- `project-structure`: Cargo workspace configuration supporting multiple crates (embedded + host utilities)
- `esp32-build`: ESP-IDF build configuration for LilyGO T-Display S3 Pro targeting Xtensa architecture
- `local-testing`: Architecture abstraction allowing unit tests to run on host machine

### Modified Capabilities

<!-- None - this is a fresh project initialization -->

## Impact

- Creates new project structure from scratch
- Requires esp-rs toolchain (espup) to be installed for building
- Development workflow: `cargo build --release` for ESP32 binary, `cargo test` for local tests
- Binary output in format suitable for flashing via `espflash`
