## Why

The ariadna firmware lacks sufficient test coverage. While `event_loop_service` has tests, `esp32_service` has none, and the main function's orchestration logic is untested. Adding tests with proper DI mocking will improve reliability and enable safe refactoring.

## What Changes

- Add tests for `Esp32Service` that verify `init()` and `halt()` behavior on host
- Extract main function logic into a testable `MainService` component (including channel creation)
- Introduce mock-based testing using `mockall` with shaku DI integration
- Add tests for `MainService` that verify correct service call ordering (init → run → halt)

## Capabilities

### New Capabilities

- `unit-testing`: Defines the unit testing approach including mockall integration with shaku DI, test module patterns, and service call verification

### Modified Capabilities

- `local-testing`: Adding requirements for mock-based testing and service-level test patterns

## Impact

- `firmware/ariadna/src/main.rs`: Logic extracted to MainService
- `firmware/ariadna/src/services/esp32_service.rs`: Add test module
- `firmware/ariadna/src/services/mod.rs`: Export MainService
- `firmware/ariadna/src/app_module.rs`: Include MainService component
- `firmware/ariadna/Cargo.toml`: Add mockall dev-dependency
