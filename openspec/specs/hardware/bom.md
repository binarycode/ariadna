# Ariadna Hardware Bill of Materials

## Components

| Component | Model | Purpose | Est. Price | Source |
|-----------|-------|---------|-----------|--------|
| Main Board | Guition JC3248W535 | ESP32-S3 + 3.5" touch display | ~$20-25 | AliExpress |
| GPS Module | Beitian BN-180 | Multi-GNSS receiver | ~$6-10 | AliExpress |
| Battery | 18650 Li-Ion + holder | Power source | ~$3-5 | AliExpress |

**Total: ~$30-40 USD**

---

## Main Board: Guition JC3248W535

### Specifications

| Spec | Value |
|------|-------|
| MCU | ESP32-S3-WROOM-1 (dual-core @ 240MHz) |
| Flash | 16MB |
| PSRAM | 8MB |
| Display | 3.5" IPS capacitive touch (320×480) |
| Display Driver | AXS15231 (QSPI) |
| SD Card | SPI interface |
| USB | Type-C |
| WiFi/BT | Built-in |

### GPIO Assignments

| Pin | Function | Notes |
|-----|----------|-------|
| IO5 | Battery ADC | Built-in voltage divider |
| IO6 | GPS UART TX | → BN-180 RX |
| IO7 | GPS UART RX | ← BN-180 TX |
| IO10 | SD CS | |
| IO11 | SD MOSI | |
| IO12 | SD CLK | |
| IO13 | SD MISO | |
| IO9 | Available | Future buttons |
| IO14 | Available | Future buttons |
| IO46 | Available | Input-only |

---

## GPS Module: Beitian BN-180

### Specifications

| Spec | Value |
|------|-------|
| Systems | GPS, GLONASS, Galileo, BeiDou, QZSS, SBAS |
| Channels | 72 |
| Accuracy | 2.0m CEP |
| Interface | UART @ 9600 baud (default) |
| Voltage | 3.3V - 5V |
| Update Rate | 1-10 Hz |

### Pinout

| Pin | Connection |
|-----|------------|
| VCC | 3.3V |
| GND | GND |
| TX | → IO7 (ESP RX) |
| RX | ← IO6 (ESP TX) |

---

## Power System

### Battery

| Spec | Value |
|------|-------|
| Type | 18650 Li-Ion |
| Voltage | 3.7V nominal (3.0-4.2V range) |
| Charging | External only |

### Monitoring

Battery voltage is read via IO5 ADC through built-in voltage divider (33k/100k).

---

## Wiring Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                    Guition JC3248W535                       │
│  ┌─────────────────────────────────────────────────────┐   │
│  │              3.5" IPS Touch Display                 │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
│   IO5 ←── Battery ADC                                      │
│   IO6 ──→ GPS TX                                           │
│   IO7 ←── GPS RX                                           │
│   IO10-13 ←→ SD Card                                       │
└───────────────────────────┬─────────────────────────────────┘
                            │
              ┌─────────────┴─────────────┐
              ▼                           ▼
    ┌─────────────────┐         ┌─────────────────┐
    │   BN-180 GPS    │         │  18650 Battery  │
    │  VCC─3.3V       │         │    + Holder     │
    │  GND─GND        │         │                 │
    │  TX──→IO7       │         │  +──→VIN        │
    │  RX←──IO6       │         │  -──→GND        │
    └─────────────────┘         └─────────────────┘
```
