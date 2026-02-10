# GNSS Tracking Capability

## ADDED Requirements

### Requirement: GnssService initializes UART and spawns listener thread

The GnssService SHALL provide an `init` method that accepts a channel transmitter and establishes UART communication for GNSS data via uart_service.

#### Scenario: Successful initialization

- **WHEN** `init` is called with a channel tx
- **THEN** the service SHALL call uart_service.init with TX pin 43 and RX pin 44
- **THEN** the service SHALL store the returned UART index
- **THEN** the service SHALL spawn a thread to listen for UART data
- **THEN** the service SHALL return the thread JoinHandle

### Requirement: GnssService parses NMEA sentences

The GnssService SHALL parse received UART data as NMEA sentences using the nmea crate.

#### Scenario: Valid GGA sentence received

- **WHEN** a valid NMEA GGA sentence is received from UART
- **THEN** the service SHALL parse latitude and longitude from the sentence
- **THEN** the service SHALL emit a GnssFix event with the parsed coordinates

#### Scenario: Invalid NMEA data received

- **WHEN** malformed or unparseable data is received from UART
- **THEN** the service SHALL log the error
- **THEN** the service SHALL continue listening for more data

### Requirement: GnssService handles UART errors

The GnssService SHALL terminate the listener thread when UART read returns an error.

#### Scenario: UART error

- **WHEN** uart_service.read_line returns uart_service::Error::Internal
- **THEN** the listener thread SHALL exit
- **THEN** the thread JoinHandle SHALL become joinable

### Requirement: GnssService is injectable via Shaku

The GnssService SHALL implement `shaku::Interface` and be resolvable from AppModule.

#### Scenario: Resolving service from module

- **WHEN** `AppModule::builder().build()` is called
- **THEN** `GnssServiceInterface` SHALL be resolvable via `HasComponent::resolve`

### Requirement: MainService initializes GnssService

The MainService SHALL call gnss_service.init() with the channel transmitter before starting the event loop.

#### Scenario: GNSS initialization in main service

- **WHEN** MainService.run() is called
- **THEN** the service SHALL call gnss_service.init(tx) with the channel transmitter
- **THEN** the service SHALL start the event loop with the channel receiver
