# Ariadna

Simple GNSS navigation device based on ESP32-S3.

## Project Structure

- `firmware/ariadna/` - Main ESP32-S3 firmware (Xtensa target)
- `chips/gnss-emulator/` - GNSS emulator (WebAssembly target)

## Commands

### firmware/ariadna

```bash
just firmware::ariadna::check            # Check compilation
just firmware::ariadna::test [TEST_NAME] # Run all tests, or specific test/pattern
```

### chips/gnss-emulator

```bash
just chips::gnss-emulator::check            # Check compilation
just chips::gnss-emulator::test [TEST_NAME] # Run all tests, or specific test/pattern
```

## Code Conventions

### Service Architecture

Each service is a folder named `*_service/` containing:

- `mod.rs` - re-exports everything with `pub use *;`
- `interface.rs` - trait extending `shaku::Interface`
- `service.rs` - implementation with `#[derive(Component)]`
- `error.rs` - error enum using `thiserror`
- `mock_service.rs` - mockall mock for testing (conditionally compiled with `#[cfg(test)]`)

### Naming

- Main struct: `Service` (not `FooService`)
- Trait: `Interface`
- Error: `Error`
- Local crate dependencies: use full path (e.g., `crate::gnss::uart_service::Interface`)
- External crate dependencies: use full path when it prevents confusion (e.g., local `Error` vs external `Error`)

### Derives

Each derive on its own line:

```rust
#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum Error { ... }
```

### Platform-Specific Code

Use conditional compilation for ESP32-specific code:

```rust
#[cfg(target_arch = "xtensa")]
mod service;
#[cfg(not(target_arch = "xtensa"))]
mod no_op_service;
```

### Testing

- Tests in same file as implementation
- Create `TestModule` using `shaku::module!` macro
- Use `MockService` with `with_component_override` for dependency injection
