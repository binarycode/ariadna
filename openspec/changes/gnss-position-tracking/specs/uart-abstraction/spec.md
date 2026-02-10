# UART Abstraction Capability

## ADDED Requirements

### Requirement: UartService defines platform-abstracted Error type

The uart_service module SHALL define an Error enum using thiserror.

#### Scenario: Error type on xtensa target

- **WHEN** a UART operation fails on xtensa architecture
- **THEN** the Error::Internal variant SHALL wrap the underlying EspError

#### Scenario: Error type on non-xtensa target

- **WHEN** a UART operation fails on non-xtensa architecture
- **THEN** the Error::Internal variant SHALL be opaque

#### Scenario: InvalidIndex error variant

- **WHEN** an invalid UART handle index is used
- **THEN** the Error::InvalidIndex variant SHALL wrap the invalid index as u32

### Requirement: UartService provides platform-abstracted UART initialization

The UartService SHALL provide an `init` method that initializes UART communication with specified pins and returns a handle index.

#### Scenario: Initialization on xtensa target

- **WHEN** `init` is called with TX and RX pin numbers on xtensa architecture
- **THEN** the service SHALL configure real UART hardware with the specified pins
- **THEN** the service SHALL retain the created UartDriver instance such that subsequent operations can access it via the returned u32 index
- **THEN** the service SHALL return the u32 index

#### Scenario: Initialization on non-xtensa target

- **WHEN** `init` is called on non-xtensa architecture
- **THEN** the service SHALL return 0 without storing anything

### Requirement: UartService provides read functionality via index

The UartService SHALL provide a `read_line` method that takes an index and reads data from the corresponding UART.

#### Scenario: Reading with valid index on xtensa target

- **WHEN** `read_line` is called with a valid index on xtensa architecture
- **THEN** the service SHALL retrieve the UartDriver at that index
- **THEN** the service SHALL read a line of data from the UART hardware
- **THEN** the service SHALL return the data as a String

#### Scenario: Reading with invalid index

- **WHEN** `read_line` is called with an index that is out of bounds
- **THEN** the service SHALL return uart_service::Error::InvalidIndex wrapping the provided index

#### Scenario: Reading on non-xtensa target

- **WHEN** `read_line` is called on non-xtensa architecture
- **THEN** the service SHALL return uart_service::Error::Internal

### Requirement: UartService is injectable via Shaku

The UartService SHALL implement `shaku::Interface` and be resolvable from AppModule.

#### Scenario: Resolving service from module

- **WHEN** `AppModule::builder().build()` is called
- **THEN** `UartServiceInterface` SHALL be resolvable via `HasComponent::resolve`

### Requirement: UartService is mockable for testing

The UartService SHALL provide a mock implementation for testing gnss_service.

#### Scenario: Mock returns predefined messages then timeout

- **WHEN** a mock UartService is configured with messages
- **THEN** `read_line` SHALL return messages in sequence
- **THEN** `read_line` SHALL return uart_service::Error::Internal after messages are exhausted
