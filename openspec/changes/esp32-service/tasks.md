## 1. Service Implementation

- [x] 1.1 Create `esp32_service.rs` with `Esp32ServiceInterface` trait defining `init()` and `halt()` methods
- [x] 1.2 Implement `Esp32Service` struct with `#[derive(shaku::Component)]`
- [x] 1.3 Add xtensa-conditional `init()` with `link_patches()` and `EspLogger::initialize_default()`
- [x] 1.4 Add xtensa-conditional `halt()` with `FreeRtos::delay_ms(u32::MAX)` loop
- [x] 1.5 Add non-xtensa no-op implementations for both methods

## 2. Module Integration

- [x] 2.1 Export `Esp32Service` and `Esp32ServiceInterface` in `services/mod.rs`
- [x] 2.2 Register `Esp32Service` component in `AppModule`

## 3. Main Integration

- [x] 3.1 Resolve `Esp32ServiceInterface` in main
- [x] 3.2 Call `init()` before event loop
- [x] 3.3 Call `halt()` after event loop

## 4. Verification

- [x] 4.1 Run `just firmware ariadna check` to verify compilation
- [x] 4.2 Run `just firmware ariadna build` to verify xtensa build
