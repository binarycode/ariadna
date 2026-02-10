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

### Requirement: Tests exercise host fallback paths

Tests SHALL call functions that contain target-conditional code. The host fallback paths (non-xtensa) will execute during tests.

#### Scenario: Testing target-conditional functions

- **GIVEN** a function with `#[cfg(target_arch = "xtensa")]` and `#[cfg(not(target_arch = "xtensa"))]` branches
- **WHEN** tests call this function
- **THEN** the non-xtensa branch executes on the host
- **AND** tests verify the function's behavior through the host fallback

#### Guidance: Do not skip testing target-conditional code

When writing tests for code that has xtensa-specific behavior (e.g., `halt()` with infinite loop), the test SHOULD call the function normally. The host fallback will execute, allowing verification of the code path. Do NOT write tests that merely instantiate objects without exercising their methods due to assumptions about target execution.

### Requirement: Mock-based service testing

Service tests requiring dependency mocking SHALL use mockall with shaku Component integration to enable DI-based test modules.

#### Scenario: Mock service dependency

- **WHEN** testing a service that depends on another service interface
- **THEN** the test SHALL use a mockall-generated mock implementing the interface
- **AND** the mock SHALL be wired into a test-specific shaku module

### Requirement: Dev dependency for mocking

The ariadna crate SHALL include mockall as a dev-dependency to enable mock generation in tests.

#### Scenario: Mockall available in tests

- **WHEN** running `cargo test -p ariadna`
- **THEN** mockall macros are available for generating mocks
