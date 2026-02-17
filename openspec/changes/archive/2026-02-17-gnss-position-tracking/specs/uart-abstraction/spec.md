# UART Abstraction Capability

## ADDED Requirements

### Requirement: UartService defines platform-specific Error type

The `gnss::uart_service` module SHALL define an Error enum using thiserror with platform-specific variants.

#### Scenario: Error variants on xtensa target

- **WHEN** compiling for xtensa architecture
- **THEN** the Error enum SHALL include:
  - `PoisonedMutex` - for mutex lock failures
  - `UartInitializationFailure(EspError)` - wrapping ESP-IDF errors
  - `NotInitialized` - when UART is not set up
  - `ReadFailure(std::io::Error)` - wrapping I/O errors

#### Scenario: Error variants on non-xtensa target

- **WHEN** compiling for non-xtensa architecture
- **THEN** the Error enum SHALL include:
  - `NotInitialized` - when UART is not set up

### Requirement: UartService provides line-based UART reading

The UartService SHALL provide a `read_line` method that reads a line from the UART.

#### Scenario: Reading on xtensa target

- **WHEN** `read_line` is called on xtensa architecture with initialized UART
- **THEN** the service SHALL read a line from the UART hardware
- **THEN** the service SHALL return the data as `Result<String, Error>`

#### Scenario: Reading on non-xtensa target

- **WHEN** `read_line` is called on non-xtensa architecture
- **THEN** the service SHALL return `Error::NotInitialized`

### Requirement: UartService configuration via build_parameters

The UartService on xtensa SHALL be configured at module build time via `build_parameters`.

#### Scenario: UART hardware initialization

- **WHEN** `Service::build_parameters(uart, tx_pin, rx_pin)` is called
- **THEN** the service SHALL configure UART1 with 9600 baud rate
- **THEN** the service SHALL use GPIO43 for TX and GPIO44 for RX
- **THEN** the service SHALL create a `BufReader` wrapping the UART driver
- **THEN** the returned parameters SHALL be usable with shaku module builder

### Requirement: UartService uses platform-conditional compilation

The UartService SHALL use conditional compilation to provide different implementations per platform.

#### Scenario: Xtensa target compilation

- **WHEN** compiling for xtensa architecture
- **THEN** the `service.rs` implementation SHALL be used
- **THEN** the implementation SHALL use esp_idf_hal for UART access

#### Scenario: Non-xtensa target compilation

- **WHEN** compiling for non-xtensa architecture
- **THEN** the `no_op_service.rs` implementation SHALL be used
- **THEN** `read_line` SHALL always return `Error::NotInitialized`

### Requirement: UartService is injectable via Shaku

The UartService SHALL implement `shaku::Interface` and be injectable.

#### Scenario: Service injection

- **WHEN** the module is built
- **THEN** `gnss::uart_service::Interface` SHALL be resolvable

### Requirement: UartService is mockable for testing

The UartService SHALL provide a mock implementation using mockall.

#### Scenario: Mock usage in tests

- **WHEN** a mock UartService is configured in tests
- **THEN** `read_line` behavior SHALL be configurable via mockall expectations
- **THEN** tests can sequence responses to simulate UART data flow
