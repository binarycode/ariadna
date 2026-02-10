## Why

The application needs proper ESP32 initialization and shutdown handling. Currently there's no centralized service managing ESP32-specific setup (link patches, logger initialization) and graceful halt behavior with FreeRTOS delays.

## What Changes

- Add `Esp32Service` as a shaku-managed service
- Provide `init()` method for ESP32 initialization (link patches, logger setup)
- Provide `halt()` method for graceful shutdown with FreeRTOS delay loop
- Use conditional compilation (`target_arch = "xtensa"`) for real ESP32 code vs no-op stubs
- Integrate service lifecycle into main: `init()` before event loop, `halt()` after

## Capabilities

### New Capabilities

- `esp32-service`: Shaku service providing ESP32 initialization and halt functionality with conditional compilation for xtensa/non-xtensa targets

### Modified Capabilities

<!-- None - this is a new capability -->

## Impact

- New service module in the codebase
- Main entry point modified to call `init()` and `halt()`
- Dependencies: `esp_idf_svc` (already present), `shaku`
