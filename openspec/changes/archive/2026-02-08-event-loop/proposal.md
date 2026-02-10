## Why

The ariadna firmware crate (`firmware/ariadna`) currently lacks a central event loop to coordinate system events and enable graceful shutdown. An event-driven architecture is needed to handle asynchronous operations and provide a clean termination mechanism.

## What Changes

- Introduce shaku dependency injection module (`AppModule`) initialized before the main loop
- Add `EventLoopService` interface in `firmware/ariadna/src/services/event_loop_service.rs`
- Create `Event` enum with `Halt` variant for signaling shutdown
- Implement error handling using `thiserror` crate
- Add unit tests for event loop behavior

## Capabilities

### New Capabilities

- `event-loop`: Core event loop service that processes events from an mpsc channel, handles errors gracefully, and terminates on Halt signal

### Modified Capabilities

<!-- No existing capabilities are being modified -->

## Impact

- **Code**: New module at `firmware/ariadna/src/services/event_loop_service.rs`, modifications to `firmware/ariadna/src/main.rs` for shaku initialization
- **Dependencies**: Adds `shaku`, `thiserror` crates to `firmware/ariadna/Cargo.toml`
- **Architecture**: Establishes foundation for event-driven firmware design in the ariadna crate
