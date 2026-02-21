## 1. Dependencies

- [x] 1.1 Add mipidsi, embedded-graphics dependencies to Cargo.toml (xtensa-only)

## 2. Service Structure

- [x] 2.1 Create utils/display_service/ directory structure
- [x] 2.2 Create mod.rs with conditional re-exports
- [x] 2.3 Create color.rs with Color enum and to_rgb565() method
- [x] 2.4 Create error.rs with Error enum (NotInitialized, ClearFailed, DrawTextFailed)
- [x] 2.5 Create interface.rs with Interface trait (clear, draw_text, size)

## 3. Service Implementations

- [x] 3.1 Create service.rs with real ILI9341 implementation (xtensa-only)
- [x] 3.2 Implement build_parameters() for SPI/pin initialization
- [x] 3.3 Create no_op_service.rs for non-xtensa targets
- [x] 3.4 Create mock_service.rs using mockall for testing

## 4. Integration

- [x] 4.1 Add display_service module to utils/mod.rs
- [x] 4.2 Wire up display service in main.rs with SPI pins (GPIO10-14)

## 5. Verification

- [x] 5.1 Run firmware check (just firmware::ariadna::check)
- [x] 5.2 Run tests to verify mock implementation works
