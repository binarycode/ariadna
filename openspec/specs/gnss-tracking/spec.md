# GNSS Tracking Capability

## Purpose

Provides GNSS position tracking via UART-connected GNSS module, parsing NMEA sentences and emitting position events.

## Requirements

### Requirement: MonitorService spawns GNSS listener thread

The `gnss::monitor_service::Service` SHALL provide an `execute` method that spawns a thread to listen for GNSS data via UART.

#### Scenario: Successful execution

- **WHEN** `execute` is called with a channel sender
- **THEN** the service SHALL spawn a thread that reads from uart_service
- **THEN** the service SHALL return a `JoinHandle<()>` for the spawned thread

### Requirement: MonitorService parses NMEA GGA sentences

The MonitorService SHALL parse received UART data as NMEA sentences using the nmea crate, extracting position from GGA sentences.

#### Scenario: Valid GGA sentence with coordinates

- **WHEN** a valid NMEA GGA sentence with latitude and longitude is received
- **THEN** the service SHALL parse latitude and longitude from the sentence
- **THEN** the service SHALL emit a `GnssFix { latitude, longitude }` event via the channel

#### Scenario: Valid GGA sentence without coordinates

- **WHEN** a valid NMEA GGA sentence is received but latitude or longitude is missing
- **THEN** the service SHALL log a warning
- **THEN** the service SHALL continue listening for more data

#### Scenario: Non-GGA NMEA sentence received

- **WHEN** a valid NMEA sentence other than GGA is received
- **THEN** the service SHALL ignore it
- **THEN** the service SHALL continue listening for more data

#### Scenario: Invalid NMEA data received

- **WHEN** malformed or unparseable data is received from UART
- **THEN** the service SHALL log a warning
- **THEN** the service SHALL continue listening for more data

### Requirement: MonitorService handles UART errors

The MonitorService SHALL terminate the listener thread when UART read returns an error.

#### Scenario: UART read error

- **WHEN** uart_service.read_line returns an error
- **THEN** the service SHALL log the error
- **THEN** the listener thread SHALL exit
- **THEN** the thread JoinHandle SHALL become joinable

### Requirement: MonitorService handles channel send failures

The MonitorService SHALL terminate when the event channel is closed.

#### Scenario: Channel closed

- **WHEN** sending a GnssFix event to the channel fails
- **THEN** the service SHALL log the error
- **THEN** the listener thread SHALL exit

### Requirement: MonitorService is injectable via Shaku

The MonitorService SHALL implement `shaku::Interface` and be injectable with UartService dependency.

#### Scenario: Service dependency injection

- **WHEN** the module is built with MonitorService
- **THEN** `gnss::monitor_service::Interface` SHALL be resolvable
- **THEN** the service SHALL have `gnss::uart_service::Interface` injected

### Requirement: MainService initializes MonitorService

The MainService SHALL call `gnss_monitor_service.execute()` with the channel transmitter before starting the event loop.

#### Scenario: GNSS monitoring initialization in main service

- **WHEN** MainService.execute() is called
- **THEN** the service SHALL call gnss_monitor_service.execute(tx) with the channel transmitter
- **THEN** the service SHALL start the event loop with the channel receiver
