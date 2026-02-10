## 1. Dependencies

- [x] 1.1 Add `shaku` crate to `firmware/ariadna/Cargo.toml`
- [x] 1.2 Add `thiserror` crate to `firmware/ariadna/Cargo.toml`

## 2. Module Structure

- [x] 2.1 Create `firmware/ariadna/src/services/` directory
- [x] 2.2 Create `firmware/ariadna/src/services/mod.rs` with module exports
- [x] 2.3 Create `firmware/ariadna/src/services/event_loop_service.rs`

## 3. Core Types

- [x] 3.1 Define `Event` enum with `Halt` variant
- [x] 3.2 Define `Error` enum using thiserror with `ReceiveError` variant

## 4. Service Implementation

- [x] 4.1 Define `EventLoopServiceInterface` trait with `run(&self, rx: Receiver<Event>)` method
- [x] 4.2 Implement `EventLoopService` struct with shaku component derive
- [x] 4.3 Implement event loop logic: block on recv, handle Halt, log errors

## 5. Shaku Module

- [x] 5.1 Create `firmware/ariadna/src/app_module.rs` with `AppModule` definition
- [x] 5.2 Register `EventLoopService` as component in `AppModule`

## 6. Main Integration

- [x] 6.1 Update `firmware/ariadna/src/main.rs` to import services module
- [x] 6.2 Initialize `AppModule` before the event loop
- [x] 6.3 Resolve `EventLoopServiceInterface` and call `run()`

## 7. Tests

- [x] 7.1 Add unit test: sending `Halt` event terminates the loop
- [x] 7.2 Verify tests pass with `cargo test -p ariadna`
