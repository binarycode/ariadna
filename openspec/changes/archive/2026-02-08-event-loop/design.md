## Context

The ariadna firmware (`firmware/ariadna`) currently has a simple delay-based loop in `main.rs`. To support graceful shutdown and future event-driven features, we need a proper event loop architecture with dependency injection.

Current state:

- Simple `loop { FreeRtos::delay_ms(1000) }` in main
- No dependency injection framework
- No event handling mechanism

## Goals / Non-Goals

**Goals:**

- Implement event loop that processes events from an mpsc channel
- Use shaku for dependency injection with `AppModule`
- Handle `Halt` event to gracefully terminate the loop
- Log errors and terminate the loop on receive errors
- Provide unit-testable service interface

**Non-Goals:**

- Async/await event loop (using sync mpsc for simplicity)
- Multiple event types beyond `Halt` (future work)
- Integration with ESP-IDF event system (separate concern)

## Decisions

### 1. Shaku for Dependency Injection

**Choice**: Use `shaku` crate with `#[shaku(interface = EventLoopServiceInterface)]`

**Rationale**: Shaku provides compile-time verified DI, works well with trait objects, and is lightweight for embedded use.

**Alternatives considered**:

- Manual DI: More boilerplate, harder to test
- `inject`: Less mature, smaller community

### 2. Sync mpsc Channel

**Choice**: Use `std::sync::mpsc::channel` for event passing

**Rationale**: Simple, well-understood, works in both ESP32 and test environments. The `Receiver` is passed to `run()` allowing the caller to control the channel.

**Alternatives considered**:

- `crossbeam-channel`: More features but unnecessary dependency
- Async channels: Adds complexity, not needed for current requirements

### 3. Event Enum with thiserror

**Choice**: Define `Event` enum (initially just `Halt`) and use `thiserror` for error types

**Rationale**: Extensible pattern for future events. `thiserror` provides clean error derivation.

```rust
pub enum Event {
    Halt,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("channel receive error: {0}")]
    ReceiveError(#[from] std::sync::mpsc::RecvError),
}
```

The error type is named `Error` and co-located with the service in `event_loop_service.rs`, accessed as `event_loop_service::Error`.

### 4. Service Interface Pattern

**Choice**: `EventLoopServiceInterface` trait with `run(&self, rx: Receiver<Event>)` method

**Rationale**:

- Trait allows mocking in tests
- Passing `rx` as parameter keeps channel ownership flexible
- Service resolved via `HasComponent::resolve(&module)`

## Risks / Trade-offs

**[Blocking receive]** → The loop blocks on `rx.recv()`. For ESP32, this is acceptable as FreeRTOS handles threading. Future async migration possible if needed.

**[Single-threaded design]** → Current design assumes single event producer. If multiple producers needed, `Sender` can be cloned.

**[Test environment differences]** → Code is conditional on `target_arch`. Tests run on host, so service must work without ESP-IDF dependencies.
