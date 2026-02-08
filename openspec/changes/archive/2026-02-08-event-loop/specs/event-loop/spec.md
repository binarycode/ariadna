## ADDED Requirements

### Requirement: Event loop processes events from channel

The EventLoopService SHALL provide a `run` method that accepts an `std::sync::mpsc::Receiver<Event>` and processes events in a loop.

#### Scenario: Loop receives and processes events

- **WHEN** the `run` method is called with a receiver
- **THEN** the service SHALL block waiting for events from the channel

### Requirement: Halt event terminates the loop

The EventLoopService SHALL terminate the event loop when a `Halt` event is received.

#### Scenario: Receiving Halt event

- **WHEN** an `Event::Halt` is sent through the channel
- **THEN** the `run` method SHALL return and the loop SHALL stop processing

### Requirement: Channel errors are logged and loop terminates

The EventLoopService SHALL log errors when channel receive fails and terminate the loop.

#### Scenario: Receive error

- **THEN** the run method SHALL return and the loop SHALL stop processing

### Requirement: Service is injectable via Shaku

The EventLoopService SHALL implement `shaku::Interface` and be resolvable from `AppModule`.

#### Scenario: Resolving service from module

- **WHEN** `AppModule::builder().build()` is called
- **THEN** `EventLoopServiceInterface` SHALL be resolvable via `HasComponent::resolve`

### Requirement: Error type is co-located with service

The module SHALL define an `Error` enum using `thiserror` for all error variants.

#### Scenario: Error type accessible

- **WHEN** importing from `event_loop_service`
- **THEN** `event_loop_service::Error` SHALL be available with `ReceiveError` variant
