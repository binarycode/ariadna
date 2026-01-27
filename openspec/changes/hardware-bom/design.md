# Hardware Design

## Context

Ariadna is a DIY GPS navigation device built from inexpensive off-the-shelf components. The design prioritizes:
- Low cost (< $50 total)
- Easy sourcing (AliExpress availability)
- Minimal assembly (no custom PCBs, soldering only)
- Sufficient capability for outdoor navigation with raster/vector maps

## Goals / Non-Goals

**Goals:**
- Define a minimal viable hardware platform for GPS navigation
- Use components with good community support and documentation
- Leave room for future expansion (buttons, sensors)

**Non-Goals:**
- On-device battery charging (batteries charged externally)
- Battery hotswap capability
- Waterproof enclosure design (future consideration)
- Custom PCB design

## Decisions

### Decision 1: Guition JC3248W535 as main board

**Rationale:** This board provides an excellent integration of ESP32-S3 with a quality IPS touch display. The 16MB flash and 8MB PSRAM are essential for map rendering. The SD card slot enables large map storage. At ~$20-25, it's cost-effective compared to buying components separately.

**Trade-offs:**
- Limited GPIO (mitigated by resistor ladder for buttons)
- QSPI display uses several pins internally
- Less flexible than bare ESP32 + separate display

### Decision 2: Beitian BN-180 GPS module

**Rationale:** Full multi-constellation support (all 5 major systems) at a low price point (~$6-10). Well-documented, popular in drone/Arduino communities. Simple UART interface requires only 2 GPIO pins.

**Alternatives considered:**
- LC76G: Similar capability, slightly higher price
- NEO-6M: Cheaper but GPS-only (no GLONASS/Galileo/BeiDou)
- ATGM336H: Cheaper but only GPS+BeiDou

### Decision 3: Single 18650 battery with external charging

**Rationale:** Simplifies the device design significantly. Most users already have 18650 chargers. Eliminates need for TP4056 charging circuit and associated complexity.

**Trade-offs:**
- Less convenient than USB charging on device
- User must carry spare batteries for long trips

### Decision 4: Software-based battery monitoring via ADC

**Rationale:** The JC3248W535 already has a voltage divider on IO5 connected to battery input. Using ADC to read voltage is simple and requires no additional hardware. While less accurate than a dedicated fuel gauge IC (like MAX17048), it's sufficient for showing approximate battery level.

**Implementation notes:**
- Voltage divider ratio: 33k/100k (need to verify on actual hardware)
- ADC reading → voltage → percentage lookup table
- Non-linear discharge curve means percentage is approximate

## Wiring Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                    Guition JC3248W535                       │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │              3.5" IPS Touch Display                 │   │
│  │                   (320 × 480)                       │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
│   IO5 ←── Battery ADC (built-in divider)                   │
│   IO6 ──→ GPS TX (to BN-180 RX)                            │
│   IO7 ←── GPS RX (from BN-180 TX)                          │
│   IO10-13 ←→ SD Card (SPI)                                 │
│                                                             │
│   IO9, IO14, IO46 ── Available for future buttons          │
│                                                             │
└───────────────────────────┬─────────────────────────────────┘
                            │
              ┌─────────────┴─────────────┐
              │                           │
              ▼                           ▼
    ┌─────────────────┐         ┌─────────────────┐
    │   BN-180 GPS    │         │  18650 Battery  │
    │                 │         │    + Holder     │
    │  VCC ← 3.3V     │         │                 │
    │  GND ← GND      │         │  + ──→ VIN      │
    │  TX  ──→ IO7    │         │  - ──→ GND      │
    │  RX  ←── IO6    │         │                 │
    └─────────────────┘         └─────────────────┘
```
