## Context

The firmware runs on ESP32-S3 with a GNSS module connected via UART on GPIO43 (TX) and GPIO44 (RX). The existing architecture uses shaku for dependency injection, an event loop processing events from mpsc channels, and platform-conditional compilation (`#[cfg(target_arch = "xtensa")]`) for hardware-specific code.

## Goals / Non-Goals

**Goals:**

- Track GNSS position with longitude/latitude available in event loop state
- Parse standard NMEA sentences from GNSS module
- Abstract UART for testability (real hardware on xtensa, mock elsewhere)
- Enable testing of GNSS parsing logic without hardware

**Non-Goals:**

- Supporting multiple GNSS modules simultaneously
- Altitude, speed, or other GNSS data beyond lat/lon
- Persistent storage of position history

## Decisions

### Decision: Separate uart_service for hardware abstraction

**Choice**: Create dedicated uart_service with platform-conditional implementation.

**Rationale**: Follows esp32_service pattern already in codebase. Keeps gnss_service focused on NMEA parsing. Enables mock UART in tests.

**Alternatives**:

- Inline UART in gnss_service with `#[cfg]` blocks → harder to test, violates SRP

### Decision: Position state as Option<f64> fields

**Choice**: Store `longitude: Option<f64>` and `latitude: Option<f64>` separately.

**Rationale**: Matches GNSS reality where fix may not be available. Simple to check and use. f64 provides sufficient precision for coordinates.

**Alternatives**:

- Wrap in `Option<Position>` struct → extra indirection, both coords always come together anyway
- Use fixed-point → unnecessary complexity for this use case

### Decision: gnss_service spawns thread, returns JoinHandle

**Choice**: `init()` takes channel tx, spawns listener thread, returns `JoinHandle`.

**Rationale**: Allows tests to wait for thread completion. Follows existing pattern where services manage their lifecycle.

**Alternatives**:

- Return nothing → tests can't synchronize with thread completion

### Decision: Test gnss_service via mock uart failures

**Choice**: Tests mock UART to return messages then fail, causing thread to exit.

**Rationale**: Provides clean test termination. Tests can verify expected events were emitted before failure.

**Alternatives**:

- Timeout-based tests → flaky, slower
- Poison pill message → complicates UART interface

### Decision: IntrospectStateService for test-time state verification

**Choice**: Create dedicated no-op service that receives State, mockable in tests.

**Rationale**: Event loop state is internal. Tests need to verify state updates after GnssFix events. IntrospectState event triggers the service, allowing mock to capture and assert state.

**Alternatives**:

- Expose state publicly → breaks encapsulation
- Return state from run() → changes interface, complicates production code

### Decision: main_service initializes gnss_service

**Choice**: main_service calls gnss_service.init(tx) before starting event loop.

**Rationale**: Follows existing pattern where main_service orchestrates service initialization. The tx channel sender is created in main_service and passed to gnss_service.

**Alternatives**:

- Initialize GNSS elsewhere → breaks single point of service orchestration

## Risks / Trade-offs

**[Risk]** UART buffer overflow if NMEA parsing is slow → Use separate thread for reading, process messages as they arrive

**[Risk]** GNSS module sends malformed data → Log parse errors, skip invalid sentences, continue listening

**[Trade-off]** uart_service abstraction adds indirection → Acceptable for testability gain
