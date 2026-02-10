## Context

The firmware currently lacks ESP32-specific initialization and shutdown handling. The `main.rs` directly builds the shaku module and runs the event loop without proper ESP32 setup (link patches, logger) or graceful halt behavior.

Existing patterns:

- Services use shaku with `trait XxxInterface: shaku::Interface` and `#[derive(shaku::Component)]`
- Conditional compilation uses `#[cfg(target_arch = "xtensa")]` for hardware-specific code
- FreeRTOS delays available via `esp_idf_svc::hal::task::block_on` or `FreeRtos::delay_ms`

## Goals / Non-Goals

**Goals:**

- Centralized ESP32 initialization via shaku service
- Graceful halt with FreeRTOS delay loop on xtensa
- No-op implementations for non-xtensa (host testing)
- Clean integration in main: init() → event loop → halt()

**Non-Goals:**

- Runtime configuration of init/halt behavior
- Error recovery in halt loop

## Decisions

**Decision 1: Separate service vs extending EventLoopService**

- Choice: Create new `Esp32Service`
- Rationale: Single responsibility - ESP32 lifecycle separate from event handling. EventLoopService handles events, Esp32Service handles hardware lifecycle.

**Decision 2: Conditional compilation approach**

- Choice: `#[cfg(target_arch = "xtensa")]` on method bodies, not entire impl blocks
- Rationale: Keeps interface consistent, only implementation differs. Matches existing pattern in codebase.

**Decision 3: halt() loop behavior**

- Choice: Infinite loop with `FreeRtos::delay_ms(u32::MAX)` on xtensa, immediate return on non-xtensa
- Rationale: Prevents ESP32 from exiting main (which would reboot). Host builds can exit normally.

## Risks / Trade-offs

- [Risk] init() called multiple times → No mitigation needed, idempotent operations (logger init handles this)
- [Risk] halt() blocks forever on xtensa → Intentional behavior for ESP32
