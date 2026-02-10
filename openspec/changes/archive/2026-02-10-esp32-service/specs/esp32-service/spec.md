## ADDED Requirements

### Requirement: ESP32 initialization

The system SHALL provide an `init()` method that initializes ESP32-specific components.

On xtensa targets, init() SHALL:

- Call `esp_idf_svc::sys::link_patches()` to apply ESP-IDF patches
- Call `esp_idf_svc::log::EspLogger::initialize_default()` to set up logging

On non-xtensa targets, init() SHALL be a no-op.

#### Scenario: Initialization on xtensa

- **WHEN** init() is called on an xtensa target
- **THEN** ESP-IDF link patches are applied AND default logger is initialized

#### Scenario: Initialization on non-xtensa

- **WHEN** init() is called on a non-xtensa target
- **THEN** the method returns immediately without side effects

### Requirement: ESP32 halt

The system SHALL provide a `halt()` method that prevents the application from exiting.

On xtensa targets, halt() SHALL loop indefinitely calling `FreeRtos::delay_ms(u32::MAX)`.

On non-xtensa targets, halt() SHALL return immediately.

#### Scenario: Halt on xtensa

- **WHEN** halt() is called on an xtensa target
- **THEN** the method loops indefinitely with max-duration FreeRTOS delays

#### Scenario: Halt on non-xtensa

- **WHEN** halt() is called on a non-xtensa target
- **THEN** the method returns immediately

### Requirement: Shaku service integration

Esp32Service SHALL be a shaku Component implementing Esp32ServiceInterface.

The service SHALL be registered in AppModule and resolved in main.

#### Scenario: Service resolution

- **WHEN** AppModule is built
- **THEN** Esp32Service can be resolved via HasComponent

### Requirement: Main lifecycle integration

The main function SHALL call `esp32_service.init()` before running the event loop and `esp32_service.halt()` after the event loop completes.

#### Scenario: Correct lifecycle order

- **WHEN** the application starts
- **THEN** init() is called before event_loop_service.run() AND halt() is called after run() returns
