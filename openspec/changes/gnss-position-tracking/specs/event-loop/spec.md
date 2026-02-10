# Event Loop Capability (Delta)

## ADDED Requirements

### Requirement: State struct defined at crate root

The `ariadna` binary crate (firmware/ariadna) SHALL define a `State` struct at the root level (in `src/main.rs` or re-exported from a module via `src/main.rs`) containing GNSS position fields. This struct SHALL be accessible via `crate::State` for use in tests and other modules within the `ariadna` crate.

#### Scenario: State structure

- **WHEN** importing State from the ariadna crate root via `use crate::State;`
- **THEN** State SHALL have a `longitude` field of type `Option<f64>`
- **THEN** State SHALL have a `latitude` field of type `Option<f64>`

### Requirement: Event loop maintains State

The EventLoopService SHALL maintain a State instance throughout the event loop.

#### Scenario: Initial state has no position

- **WHEN** the event loop starts
- **THEN** the State longitude SHALL be None
- **THEN** the State latitude SHALL be None

### Requirement: GnssFix event updates position state

The EventLoopService SHALL update State when a GnssFix event is received.

#### Scenario: Receiving GnssFix event with valid coordinates

- **WHEN** an `Event::GnssFix { longitude, latitude }` is received
- **THEN** the State longitude SHALL be updated with the new value
- **THEN** the State latitude SHALL be updated with the new value
- **THEN** the loop SHALL continue processing events

### Requirement: GnssFix event type exists

The Event enum SHALL include a GnssFix variant for position updates.

#### Scenario: GnssFix event structure

- **WHEN** creating a GnssFix event
- **THEN** the event SHALL contain longitude as f64
- **THEN** the event SHALL contain latitude as f64

### Requirement: IntrospectState event triggers state introspection

The EventLoopService SHALL call IntrospectStateService when an IntrospectState event is received.

#### Scenario: Receiving IntrospectState event

- **WHEN** an `Event::IntrospectState` is received
- **THEN** the service SHALL call `introspect_state_service.introspect(&state)`
- **THEN** the loop SHALL continue processing events
