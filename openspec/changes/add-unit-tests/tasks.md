## 1. Dependencies

- [ ] 1.1 Add mockall as dev-dependency in `firmware/ariadna/Cargo.toml`

## 2. Esp32Service Tests

- [ ] 2.1 Add `#[cfg(test)]` module to `esp32_service.rs` with test for `init()` (verify it executes without panic)
- [ ] 2.2 Add test for `halt()` in `esp32_service.rs` that actually calls halt() (it's testable on host in non-xtensa environment)

## 3. MainService Extraction (with channel ownership)

- [ ] 3.1 Create `main_service.rs` with `MainServiceInterface` trait defining `run(&self) -> Sender<Event>` (channel created internally, sender returned)
- [ ] 3.2 Implement `MainService` struct with shaku Component, injecting `Esp32ServiceInterface` and `EventLoopServiceInterface`
- [ ] 3.3 Implement `MainServiceInterface::run()` that creates channel internally, runs init → event_loop.run(rx) → halt orchestration, and returns sender
- [ ] 3.4 Export `MainService` and `MainServiceInterface` from `services/mod.rs`
- [ ] 3.5 Add `MainService` to `AppModule` components
- [ ] 3.6 Update `main()` to simply call MainService::run()

## 4. Mock Infrastructure

- [ ] 4.1 Add `MockEsp32Service` to `esp32_service.rs` (cfg(test) only) with mockall and shaku Component impl, export from module
- [ ] 4.2 Add `MockEventLoopService` to `event_loop_service.rs` (cfg(test) only) with mockall and shaku Component impl, export from module

## 5. MainService Tests (with distinct test cases)

- [ ] 5.1 Add test module to `main_service.rs` with TestModule declaring MainService and mocks
- [ ] 5.2 Add `test_happy_case()` verifying init(), run() and halt() are called in order using mockall Sequence
- [ ] 5.4 Add `test_halt_called_after_run_error()` verifying halt() is called even when run() returns error (cleanup guarantee)
- [ ] 5.5 Add `test_error_propagation_from_event_loop_service()` verifying errors from EventLoopService are propagated

## 6. Verification

- [ ] 6.1 Run `cargo test -p ariadna` and verify all tests pass
- [ ] 6.2 Run `just check` to verify build succeeds
