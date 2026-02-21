## Context

The firmware uses shaku for dependency injection. Services follow a standard pattern with `interface.rs`, `service.rs`, `no_op_service.rs`, `mock_service.rs`, and `error.rs`. The PoC at `firmware/poc/src/main.rs` demonstrates ILI9341 setup using mipidsi crate over SPI.

Current state: Display code exists only in PoC, not integrated with main firmware DI architecture.

## Goals / Non-Goals

**Goals:**

- Abstract display hardware behind a testable DI service
- Support clearing screen, and rendering text
- Follow existing service patterns (gnss/uart_service as reference)
- Enable unit testing with mock implementation

**Non-Goals:**

- Alpha blending / transparency (opaque overwrite only)
- Framebuffer / double-buffering (direct writes to display)
- Touch input handling
- Multiple display support

## Decisions

### 1. Service location: `utils/display_service/`

**Rationale**: Display is a utility peripheral, similar to log_service. Not core business logic, not GNSS-specific.

**Alternatives considered**:

- `core/` - rejected, display is not core business logic
- `display/display_service/` - rejected, unnecessary nesting for single service

### 2. Color type in separate file

Color type defined in `color.rs`:

```rust
#[derive(Debug, Clone, Copy)]
pub enum Color {
    Black,
    White,
    Red,
    Green,
    Blue,
}

impl Color {
    #[cfg(target_arch = "xtensa")]
    pub fn to_rgb565(self) -> Rgb565 { ... }
}
```

**Rationale**: Separate color definition from interface allows future expansion. `to_rgb565()` method only on xtensa target where Rgb565 is available.

### 3. Interface design: High-level methods with Color type

```rust
pub trait Interface: shaku::Interface {
    fn clear(&self, color: Color) -> Result<(), Error>;
    fn draw_text(&self, text: &str, x: i32, y: i32, color: Color, background_color: Option<Color>) -> Result<(), Error>;
    fn size(&self) -> (u32, u32);
}
```

**Rationale**:

- `DrawTarget` trait is not dyn-compatible, can't use with shaku's trait objects
- Color type abstracts RGB565 conversion, keeps interface clean
- Keep interface simple: clear, draw_text, and size

**Alternatives considered**:

- Expose `DrawTarget` - rejected, not dyn-compatible
- Rgb565 directly - rejected, needs abstraction in Color type

### 4. Error handling with specific variants

Error enum defined in `error.rs`:

```rust
#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum Error {
    #[error("Display service not initialized")]
    NotInitialized,
    #[error("Failed to clear display: {0}")]
    ClearFailed(#[source] Box<dyn std::error::Error>),
    #[error("Failed to draw text: {0}")]
    DrawTextFailed(#[source] Box<dyn std::error::Error>),
}
```

**Rationale**: Specific error variants for each operation. Each variant wraps the underlying error (from mipidsi or embedded-graphics), not arbitrary strings. Easier for callers to handle specific failure modes while preserving root cause information.

### 5. Direct writes (no buffering)

**Rationale**: ILI9341 has internal GRAM. Each draw goes directly to display. No RAM buffer needed on ESP32 side. Acceptable for opaque overwrite compositing.

**Trade-off**: Sequential draws visible (no atomic frame updates). Acceptable per requirements.

### 6. Text rendering: embedded-graphics MonoFont

**Rationale**: embedded-graphics provides built-in mono fonts (e.g., `FONT_6X10`). Simple, no external font loading needed.

**Decision**: Start with a single built-in font. Font selection can be added later if needed.

### 7. Service initialization via build_parameters()

**Rationale**: Unwrap on initialization failure (like uart_service does). If display hardware fails to initialize, it's a fatal configuration error, not a recoverable runtime error.

**Decision**:

- Service struct property (xtensa target):
  ```rust
  #[shaku(default)]
  display: Mutex<Option<Display<'static>>>,
  ```
  where `Display` is:
  ```rust
  type Display<'a> = mipidsi::Display<
      SPIInterface<
          SpiDeviceDriver<'a, esp_idf_hal::spi::SpiDriver<'a>>,
          PinDriver<'a, esp_idf_hal::gpio::Gpio12, esp_idf_hal::gpio::Output>
      >,
      mipidsi::models::ILI9341Rgb565,
      PinDriver<'a, esp_idf_hal::gpio::Gpio13, esp_idf_hal::gpio::Output>
  >;
  ```
- `#[shaku(default)]` initializes field to `Mutex::new(None)`
- `build_parameters()` returns `ServiceParameters { display: Mutex::new(Some(display_instance)) }`
- `build_parameters()` unwraps on Display initialization failure (fatal config error)
- `build_parameters()` does NOT clear the display (caller responsibility)

### 8. Pin configuration via build_parameters()

Following gnss/uart_service pattern:

```rust
impl Service {
    pub fn build_parameters(
        spi: SPI2,
        sclk: Gpio10,
        mosi: Gpio11,
        dc: Gpio12,
        rst: Gpio13,
        cs: Gpio14,
    ) -> ServiceParameters { ... }
}
```

**Rationale**: Consistent with existing UART pattern. Peripherals passed from main.rs, service owns initialization. Unwraps on failure (fatal configuration error).

## Risks / Trade-offs

**[Risk] Display initialization failure** → Unwrap in build_parameters, crash at startup. Correct behavior for hardware configuration errors.

**[Risk] Text rendering performance** → Direct SPI writes per character may be slow for large text. Mitigation: acceptable for status displays; optimize later if needed.

**[Trade-off] No buffering** → Can't do atomic frame updates, may see partial draws. Acceptable per requirements.

**[Trade-off] Single font** → Less flexibility. Can extend interface later with font parameter.
