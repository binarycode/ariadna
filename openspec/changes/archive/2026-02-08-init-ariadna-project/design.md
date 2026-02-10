## Context

Starting a new embedded Rust project for a GPS navigator. The hardware is LilyGO T-Display S3 Pro (ESP32-S3 with display) paired with Beitian BE-880 GPS module. The project needs to support multiple programs: the main embedded firmware plus host-side utilities (map tile generators, etc.).

ESP32-S3 uses Xtensa architecture which requires the esp-rs toolchain. The T-Display S3 Pro has an ST7789 display and USB-C for flashing/debugging.

## Goals / Non-Goals

**Goals:**

- Cargo workspace supporting embedded and host crates side-by-side
- `ariadna` crate targeting ESP32-S3 with proper binary output for flashing
- Unit tests runnable on host machine without requiring ESP32 hardware
- Clear separation between target-specific and portable code

**Non-Goals:**

- Implementing GPS parsing, display drivers, or navigator logic (future work)
- Creating utility crates (map generators come later)
- CI/CD setup
- OTA update mechanism

## Decisions

### 1. Use `esp-idf-hal` (std) over bare-metal `esp-hal` (no_std)

**Decision:** Use the ESP-IDF framework with std support.

**Rationale:** The T-Display S3 Pro has sufficient resources (16MB flash, 8MB PSRAM). ESP-IDF provides proven drivers, WiFi/BLE stack, and better debugging. The std environment simplifies development and allows richer abstractions.

**Alternative considered:** Bare-metal no_std with esp-hal. Rejected because it adds complexity without clear benefit for this use case.

### 2. Workspace structure with target-aware crates

**Decision:**

```
/
├── Cargo.toml          # workspace root
├── ariadna/            # ESP32-S3 firmware (xtensa target)
│   ├── Cargo.toml
│   └── src/
└── .cargo/
    └── config.toml     # ESP32-S3 target config
```

**Rationale:** Keeps workspace root clean. Future utilities can be added as sibling crates. The `.cargo/config.toml` at root applies ESP32 settings only when building ariadna.

### 3. Use `espflash` for binary output

**Decision:** Configure build to produce binaries compatible with `espflash` tool.

**Rationale:** espflash is the standard tool in esp-rs ecosystem. Handles partition tables, bootloader, and direct USB flashing.

### 4. Test architecture via conditional compilation

**Decision:** Use `#[cfg(target_arch = "xtensa")]` to gate hardware-specific code, allowing business logic tests on host.

**Rationale:** Running tests on actual hardware is slow and requires physical device. Core logic should be testable on any machine.

## Risks / Trade-offs

**[ESP-IDF version lock-in]** → Pin specific esp-idf version in sdkconfig. Document version in README.

**[Toolchain setup complexity]** → Require espup installation. Document setup steps clearly.

**[Large binary size with std]** → Acceptable trade-off for development velocity. Can optimize later if needed.
