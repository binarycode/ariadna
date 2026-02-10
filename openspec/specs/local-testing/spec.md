# Local Testing Capability

## Purpose

Enables host-based testing of firmware code without requiring physical ESP32 hardware.

## Requirements

### Requirement: Host-target test execution

Unit tests for non-hardware-specific code SHALL be runnable on the host machine without ESP32 hardware.

#### Scenario: Run tests on host

- **WHEN** running `cargo test` from repository root
- **THEN** tests execute on the host architecture (not Xtensa)

### Requirement: Target-conditional compilation

Hardware-specific code SHALL be gated with `#[cfg(target_arch = "xtensa")]` to allow compilation on host for testing.

#### Scenario: Code compiles on host

- **WHEN** building test targets on host machine
- **THEN** Xtensa-specific code is excluded via cfg attributes

### Requirement: Separate test configuration

The `.cargo/config.toml` SHALL NOT apply Xtensa target when running `cargo test`, allowing tests to use host architecture.

#### Scenario: Test target differs from build target

- **WHEN** running `cargo test -p ariadna`
- **THEN** tests compile for host architecture, not xtensa-esp32s3-espidf
