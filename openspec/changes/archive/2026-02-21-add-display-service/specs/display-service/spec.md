## ADDED Requirements

### Requirement: Display service follows DI architecture

The display service SHALL implement `shaku::Interface` and follow the standard service pattern with `mod.rs`, `interface.rs`, `error.rs`, `service.rs`, `no_op_service.rs`, and `mock_service.rs`.

#### Scenario: Service is injectable via shaku

- **WHEN** a component requires display functionality
- **THEN** it SHALL receive the display service via shaku dependency injection

### Requirement: Display service provides screen dimensions

The display service SHALL expose a `size()` method returning the display width and height in pixels.

#### Scenario: Query display dimensions

- **WHEN** caller invokes `size()`
- **THEN** the service SHALL return (240, 320) for ILI9341 in portrait mode

### Requirement: Display service clears screen

The display service SHALL provide a `clear(color)` method to fill the entire screen with a solid color.

#### Scenario: Clear screen with color

- **WHEN** caller invokes `clear(color)`
- **THEN** the entire display SHALL be filled with the specified color

### Requirement: Display service renders text

The display service SHALL provide a `draw_text(text, position, color, background_color)` method to render text at a given position.

#### Scenario: Render text string

- **WHEN** caller invokes `draw_text(text, position, color, background_color)`
- **THEN** the text SHALL be rendered at the specified position with the given color

### Requirement: Platform-specific implementations

The display service SHALL provide different implementations based on target architecture.

#### Scenario: Real implementation on xtensa

- **WHEN** compiled for `target_arch = "xtensa"`
- **THEN** the service SHALL use `mipidsi` crate with ILI9341 driver over SPI

#### Scenario: No-op implementation on other platforms

- **WHEN** compiled for non-xtensa targets
- **THEN** the service SHALL use a no-op implementation that performs no hardware operations

### Requirement: Display service is mockable for testing

The display service SHALL provide a mock implementation using `mockall` for unit testing.

#### Scenario: Mock in unit tests

- **WHEN** running unit tests
- **THEN** the mock service SHALL be available via conditional compilation (`#[cfg(test)]`)
