# Project Structure Capability

## Purpose

Defines the workspace organization and crate layout for the Ariadna ESP32-S3 firmware project.

## Requirements

### Requirement: Cargo workspace configuration

The repository SHALL have a root `Cargo.toml` that defines a Cargo workspace with member crates.

#### Scenario: Workspace includes ariadna crate

- **WHEN** running `cargo metadata` from repository root
- **THEN** the workspace members list includes "firmware/ariadna"

### Requirement: Ariadna crate exists as workspace member

The workspace SHALL contain an `firmware/ariadna` directory with its own `Cargo.toml` configured as an ESP-IDF application.

#### Scenario: Ariadna crate is valid

- **WHEN** examining `firmware/ariadna/Cargo.toml`
- **THEN** it defines a binary crate with esp-idf-svc dependency

### Requirement: Future utilities supported

The workspace structure SHALL allow adding additional host-side utility crates without modifying existing crate configurations.

#### Scenario: Adding new utility crate

- **WHEN** creating a new crate directory and adding it to workspace members
- **THEN** both the new crate and ariadna can be built independently
