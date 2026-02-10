## 1. Project Root Setup

- [x] 1.1 Create root `Cargo.toml` with workspace configuration and ariadna as member
- [x] 1.2 Create `rust-toolchain.toml` specifying esp channel for Xtensa support
- [x] 1.3 Create `.gitignore` with Rust and ESP-IDF exclusions
- [x] 1.4 Create `README.md` with project overview and setup instructions

## 2. Cargo Configuration

- [x] 2.1 Create `.cargo/config.toml` with ESP32-S3 target and build settings
- [x] 2.2 Configure runner for espflash in cargo config

## 3. Ariadna Crate

- [x] 3.1 Create `ariadna/Cargo.toml` with esp-idf-svc and esp-idf-hal dependencies
- [x] 3.2 Create `ariadna/src/main.rs` with minimal ESP-IDF application entry point
- [x] 3.3 Create `ariadna/sdkconfig.defaults` with base ESP-IDF configuration
- [x] 3.4 Create `ariadna/build.rs` for ESP-IDF build integration

## 4. Local Testing Support

- [x] 4.1 Add conditional compilation gates in main.rs for Xtensa-specific code
- [x] 4.2 Create `ariadna/src/lib.rs` for testable module exports
- [x] 4.3 Add example unit test that runs on host architecture

## 5. Verification

- [x] 5.1 Verify `cargo build -p ariadna --release` compiles for xtensa-esp32s3-espidf
- [x] 5.2 Verify `cargo test` runs tests on host architecture
