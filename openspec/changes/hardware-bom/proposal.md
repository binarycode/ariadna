# Hardware Bill of Materials

## Why

Ariadna is a GPS navigation device built from off-the-shelf components sourced from AliExpress. Before any software development can begin, we need to define and document the hardware platform - the components that will be purchased, their specifications, and how they interconnect.

## What Changes

- Document the complete bill of materials for the Ariadna GPS device
- Specify component models, purposes, and estimated costs
- Define GPIO pin assignments for peripheral connections
- Establish the hardware constraints that will guide firmware development

## Capabilities

### New Capabilities

- `hardware-spec`: Complete hardware specification for the Ariadna GPS device including components, pinouts, and interconnections

## Impact

- `openspec/specs/hardware/`: New hardware specification documents
- Future firmware development will reference these specs for pin assignments and peripheral interfaces
