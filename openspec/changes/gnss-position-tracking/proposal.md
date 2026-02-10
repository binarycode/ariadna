## Why

The firmware needs to track device position via GNSS for location-aware functionality. GPS coordinates from a GNSS module connected via UART must be received, parsed, and made available to the event loop for use by other services.

## What Changes

- Add position state (longitude/latitude) to event_loop_service that tracks current GNSS fix
- Introduce new `GnssFix` event type for position updates
- Create gnss_service that connects to GNSS module via UART, parses NMEA sentences, and emits fix events
- Create uart_service to abstract UART communication with platform-specific implementations (real on xtensa, no-op elsewhere)

## Capabilities

### New Capabilities

- `gnss-tracking`: GNSS position tracking via UART, NMEA parsing, and event emission
- `uart-abstraction`: Platform-abstracted UART communication layer
- `introspect-state`: No-op service for test-time state verification

### Modified Capabilities

- `event-loop`: Add State struct, GnssFix and IntrospectState event handling

## Impact

- **Code**: New `gnss_service.rs`, `uart_service.rs`, `introspect_state_service.rs` in services; modified `event.rs`, `event_loop_service.rs`, `main_service.rs`
- **Dependencies**: `nmea` crate for parsing NMEA sentences
- **Hardware**: UART connection on GPIO43 (TX) and GPIO44 (RX) for GNSS module
