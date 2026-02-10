# Introspect State Capability

## ADDED Requirements

### Requirement: IntrospectStateService provides state inspection

The IntrospectStateService SHALL provide an `introspect` method that receives the current State.

#### Scenario: Production behavior

- **WHEN** `introspect` is called with a State reference
- **THEN** the service SHALL be a no-op

### Requirement: IntrospectStateService is injectable via Shaku

The IntrospectStateService SHALL implement `shaku::Interface` and be resolvable from AppModule.

#### Scenario: Resolving service from module

- **WHEN** `AppModule::builder().build()` is called
- **THEN** `IntrospectStateServiceInterface` SHALL be resolvable via `HasComponent::resolve`

### Requirement: IntrospectStateService is mockable for testing

The IntrospectStateService SHALL provide a mock implementation for testing event loop state updates.

#### Scenario: Test behavior with mock

- **WHEN** a mock IntrospectStateService is injected in tests
- **THEN** tests can capture and verify the State passed to `introspect`
