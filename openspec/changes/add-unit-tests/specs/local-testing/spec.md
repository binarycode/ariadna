# Local Testing Capability (Delta)

## ADDED Requirements

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
