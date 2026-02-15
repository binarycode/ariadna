## 1. Core Types

- [x] 1.1 Add State struct to crate root with longitude: Option<f64> and latitude: Option<f64>
- [x] 1.2 Add GnssFix { longitude: f64, latitude: f64 } variant to Event enum
- [x] 1.3 Add IntrospectState variant to Event enum

## 2. UART Service

- [x] 2.1 Create uart_service.rs with Error enum (Internal variant, wraps EspError on xtensa, opaque on non-xtensa)
- [x] 2.2 Implement UartServiceInterface trait with init(tx_pin: u32, rx_pin: u32) -> u32 and read_line(index: u32) -> Result<String, Error>
- [x] 2.3 Implement UartService struct with internal Vec<UartDriver> storage on xtensa
- [x] 2.4 Implement init() - configure UART hardware on xtensa, return 0 on non-xtensa
- [x] 2.5 Implement read_line() - read from UartDriver on xtensa, return Error::Internal on non-xtensa
- [x] 2.6 Add UartService to AppModule
- [x] 2.7 Add mock implementation for UartService with configurable message sequence

## 3. Introspect State Service

- [x] 3.1 Create introspect_state_service.rs with IntrospectStateServiceInterface trait
- [x] 3.2 Implement introspect(&self, state: &State) as no-op
- [x] 3.3 Add IntrospectStateService to AppModule
- [x] 3.4 Add mock implementation for IntrospectStateService

## 4. Event Loop Updates

- [x] 4.1 Add IntrospectStateService dependency to EventLoopService
- [x] 4.2 Initialize State at start of run() loop
- [x] 4.3 Handle GnssFix event - update State longitude and latitude
- [x] 4.4 Handle IntrospectState event - call introspect_state_service.introspect(&state)
- [x] 4.5 Add tests for GnssFix event updating state (using mock IntrospectStateService)

## 5. GNSS Service

- [x] 5.1 Create gnss_service.rs with GnssServiceInterface trait
- [x] 5.2 Add UartService dependency to GnssService
- [x] 5.3 Implement init(tx: Sender<Event>) -> JoinHandle - call uart_service.init(43, 44), spawn listener thread
- [x] 5.4 Implement listener loop - read_line, parse NMEA with nmea crate, emit GnssFix events
- [x] 5.5 Handle NMEA parse errors - log and continue
- [x] 5.6 Handle uart_service::Error::Internal - exit listener thread
- [x] 5.7 Add GnssService to AppModule
- [x] 5.8 Add tests using mock UartService - verify NMEA parsing and event emission

## 6. Main Service Updates

- [x] 6.1 Add GnssService dependency to MainService
- [x] 6.2 Call gnss_service.init(tx) before event_loop_service.run(rx)
- [x] 6.3 Update main_service tests with mock GnssService

## 7. Dependencies

- [x] 7.1 Add nmea crate to Cargo.toml
