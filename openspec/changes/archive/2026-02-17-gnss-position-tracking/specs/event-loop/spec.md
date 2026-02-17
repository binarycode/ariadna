# Event Loop Capability (Delta)

## ADDED Requirements

### Requirement: State struct defined in core module

The `ariadna` binary crate SHALL define a `State` struct in the `core` module containing GNSS position fields.

#### Scenario: State structure

- **WHEN** importing State via `crate::core::State`
- **THEN** State SHALL have a `latitude` field of type `Option<f64>`
- **THEN** State SHALL have a `longitude` field of type `Option<f64>`
- **THEN** State SHALL derive `Debug`

### Requirement: Event loop maintains State

The EventLoopService SHALL maintain a State instance throughout the event loop.

#### Scenario: Initial state has no position

- **WHEN** the `execute` method starts
- **THEN** the State latitude SHALL be None
- **THEN** the State longitude SHALL be None

### Requirement: GnssFix event updates position state

The EventLoopService SHALL update State when a GnssFix event is received.

#### Scenario: Receiving GnssFix event with coordinates

- **WHEN** an `Event::GnssFix { latitude, longitude }` is received
- **THEN** the State latitude SHALL be updated to `Some(latitude)`
- **THEN** the State longitude SHALL be updated to `Some(longitude)`
- **THEN** the loop SHALL continue processing events

### Requirement: GnssFix event type exists

The `core::Event` enum SHALL include a GnssFix variant for position updates.

#### Scenario: GnssFix event structure

- **WHEN** creating a GnssFix event
- **THEN** the event SHALL be `Event::GnssFix { latitude: f64, longitude: f64 }`

### Requirement: IntrospectState event triggers state introspection

The EventLoopService SHALL call IntrospectStateService when an IntrospectState event is received.

#### Scenario: Receiving IntrospectState event

- **WHEN** an `Event::IntrospectState` is received
- **THEN** the service SHALL call `utils_introspect_state_service.execute(&state)`
- **THEN** the loop SHALL continue processing events

### Requirement: IntrospectState event type exists

The `core::Event` enum SHALL include an IntrospectState variant.

#### Scenario: IntrospectState event structure

- **WHEN** creating an IntrospectState event
- **THEN** the event SHALL be `Event::IntrospectState` (unit variant)
