## Why

The firmware needs to interact with the ILI9341 display for rendering maps, icons, and text. Currently, display setup is only in the PoC (firmware/poc/src/main.rs) and not integrated into the main firmware's DI architecture. A display service will abstract hardware setup, enable testability via mocks, and provide a clean interface for rendering.

## What Changes

- Add new `utils/display_service/` following the standard service pattern
- Implement `Interface` trait with methods for clearing, drawing images, and rendering text
- Real implementation uses `mipidsi` crate with ILI9341 driver (xtensa-only)
- No-op implementation for non-xtensa targets
- Mock implementation for unit tests
- Wire up SPI pins in main.rs similar to GNSS UART pattern

## Capabilities

### New Capabilities

- `display-service`: DI service abstracting ILI9341 display with clear, draw_text, and size methods

### Modified Capabilities

<!-- None - this is a new capability -->

## Impact

- **New files**: `utils/display_service/{mod,interface,error,service,no_op_service,mock_service}.rs`
- **Modified files**:
  - `firmware/ariadna/Cargo.toml` (add mipidsi, embedded-graphics dependencies)
  - `firmware/ariadna/src/main.rs` (add display service to module, wire up SPI pins)
- **Dependencies**: Add `mipidsi`, `embedded-graphics` (xtensa-only in Cargo.toml)
- **Pins used**: SPI2, SCLK=GPIO10, MOSI=GPIO11, CS=GPIO14, DC=GPIO12, RST=GPIO13
