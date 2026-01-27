# Hardware Specification

## ADDED Requirements

### Requirement: Bill of Materials

The Ariadna GPS device consists of the following components:

| Component | Model | Purpose | Est. Price |
|-----------|-------|---------|-----------|
| Main Board | Guition JC3248W535 | ESP32-S3 + 3.5" capacitive touch IPS display (320×480) | ~$20-25 |
| GPS Module | Beitian BN-180 | Multi-GNSS receiver (GPS/GLONASS/Galileo/BeiDou/QZSS/SBAS) | ~$6-10 |
| Battery | 18650 Li-Ion + holder | Power source (3.7V nominal) | ~$3-5 |

**Total estimated cost: ~$30-40 USD**

#### Scenario: Component availability

- **WHEN** sourcing components from AliExpress
- **THEN** all components should be readily available from multiple sellers
- **AND** total cost should not exceed $50 USD

---

### Requirement: Main Board Specifications

The Guition JC3248W535 provides the following capabilities:

| Specification | Value |
|--------------|-------|
| MCU | ESP32-S3-WROOM-1 (dual-core @ 240MHz) |
| Flash | 16MB |
| PSRAM | 8MB |
| Display | 3.5" IPS capacitive touch (320×480) |
| Display Driver | AXS15231 (QSPI interface) |
| Touch Controller | I2C capacitive |
| SD Card | Supported (SPI interface) |
| USB | Type-C (power + programming) |
| WiFi/Bluetooth | Built-in |

#### Scenario: Display capability

- **WHEN** rendering maps on the display
- **THEN** the 320×480 resolution provides adequate detail for navigation
- **AND** 8MB PSRAM allows buffering of map tiles

#### Scenario: Storage capability

- **WHEN** storing offline maps and GPS tracks
- **THEN** the SD card slot allows use of large capacity storage
- **AND** 16MB flash provides space for firmware and configuration

---

### Requirement: GPS Module Specifications

The Beitian BN-180 provides the following capabilities:

| Specification | Value |
|--------------|-------|
| Satellite Systems | GPS, GLONASS, Galileo, BeiDou, QZSS, SBAS |
| Channels | 72 |
| Position Accuracy | 2.0m CEP |
| Velocity Accuracy | 0.1 m/s |
| Interface | UART (TTL) |
| Update Rate | 1-10 Hz (default 1Hz) |
| Voltage | 3.3V - 5V |

#### Scenario: Multi-constellation positioning

- **WHEN** acquiring GPS position
- **THEN** the receiver should use satellites from all available constellations
- **AND** achieve position lock within 30 seconds (cold start) in open sky

#### Scenario: Track recording

- **WHEN** recording movement track
- **THEN** position updates at configurable rate (1-10 Hz)
- **AND** accuracy is sufficient for trail navigation (< 5m typical)

---

### Requirement: Power System

The device is powered by a single 18650 Li-Ion battery.

| Specification | Value |
|--------------|-------|
| Battery Type | 18650 Li-Ion |
| Nominal Voltage | 3.7V |
| Full Charge | 4.2V |
| Cutoff Voltage | ~3.0V |
| Charging | External (not on device) |
| Monitoring | ADC via built-in voltage divider on IO5 |

#### Scenario: Battery monitoring

- **WHEN** the device is powered on
- **THEN** battery voltage is readable via IO5 ADC
- **AND** software can calculate approximate remaining charge percentage

#### Scenario: Low battery warning

- **WHEN** battery voltage drops below 3.3V
- **THEN** the device should warn the user
- **AND** continue operating until ~3.0V cutoff

---

### Requirement: GPIO Pin Assignments

Available GPIO pins on the JC3248W535:

| Pin | Assignment | Notes |
|-----|-----------|-------|
| IO5 | Battery ADC | Built-in voltage divider (33k/100k) |
| IO6 | GPS UART TX | Connect to BN-180 RX |
| IO7 | GPS UART RX | Connect to BN-180 TX |
| IO10 | SD Card CS | Reserved for SD card |
| IO11 | SD Card MOSI | Reserved for SD card |
| IO12 | SD Card CLK | Reserved for SD card |
| IO13 | SD Card MISO | Reserved for SD card |
| IO9 | Available | Future use (buttons) |
| IO14 | Available | Future use (buttons) |
| IO46 | Available | Future use (input-only) |

#### Scenario: Peripheral connections

- **WHEN** connecting the GPS module
- **THEN** use IO6 (TX) and IO7 (RX) for UART communication
- **AND** configure UART at 9600 baud (BN-180 default)

#### Scenario: Future expansion

- **WHEN** adding physical buttons in the future
- **THEN** IO9, IO14, and IO46 are available
- **AND** resistor ladder technique can multiplex additional buttons on a single ADC pin
