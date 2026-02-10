## 1. Core Types

- [ ] 1.1 Add State struct to crate root with longitude: Option<f64> and latitude: Option<f64>
- [ ] 1.2 Add GnssFix { longitude: f64, latitude: f64 } variant to Event enum
- [ ] 1.3 Add IntrospectState variant to Event enum

## 2. UART Service

- [ ] 2.1 Create uart_service.rs with Error enum (Internal variant, wraps EspError on xtensa, opaque on non-xtensa)
- [ ] 2.2 Implement UartServiceInterface trait with init(tx_pin: u32, rx_pin: u32) -> u32 and read_line(index: u32) -> Result<String, Error>
- [ ] 2.3 Implement UartService struct with internal Vec<UartDriver> storage on xtensa
- [ ] 2.4 Implement init() - configure UART hardware on xtensa, return 0 on non-xtensa
- [ ] 2.5 Implement read_line() - read from UartDriver on xtensa, return Error::Internal on non-xtensa
- [ ] 2.6 Add UartService to AppModule
- [ ] 2.7 Add mock implementation for UartService with configurable message sequence

## 3. Introspect State Service

- [ ] 3.1 Create introspect_state_service.rs with IntrospectStateServiceInterface trait
- [ ] 3.2 Implement introspect(&self, state: &State) as no-op
- [ ] 3.3 Add IntrospectStateService to AppModule
- [ ] 3.4 Add mock implementation for IntrospectStateService

## 4. Event Loop Updates

- [ ] 4.1 Add IntrospectStateService dependency to EventLoopService
- [ ] 4.2 Initialize State at start of run() loop
- [ ] 4.3 Handle GnssFix event - update State longitude and latitude
- [ ] 4.4 Handle IntrospectState event - call introspect_state_service.introspect(&state)
- [ ] 4.5 Add tests for GnssFix event updating state (using mock IntrospectStateService)

## 5. GNSS Service

- [ ] 5.1 Create gnss_service.rs with GnssServiceInterface trait
- [ ] 5.2 Add UartService dependency to GnssService
- [ ] 5.3 Implement init(tx: Sender<Event>) -> JoinHandle - call uart_service.init(43, 44), spawn listener thread
- [ ] 5.4 Implement listener loop - read_line, parse NMEA with nmea crate, emit GnssFix events
- [ ] 5.5 Handle NMEA parse errors - log and continue
- [ ] 5.6 Handle uart_service::Error::Internal - exit listener thread
- [ ] 5.7 Add GnssService to AppModule
- [ ] 5.8 Add tests using mock UartService - verify NMEA parsing and event emission

## 6. Main Service Updates

- [ ] 6.1 Add GnssService dependency to MainService
- [ ] 6.2 Call gnss_service.init(tx) before event_loop_service.run(rx)
- [ ] 6.3 Update main_service tests with mock GnssService

## 7. Dependencies

- [ ] 7.1 Add nmea crate to Cargo.toml
