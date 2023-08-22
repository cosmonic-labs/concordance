---
name: RoverPositionChanged
summary: "Indicates that a rover has moved to a new grid coordinate"
version: 1.0.0
producers:
    - 'Rover Aggregate'
consumers:
    - 'Rover Projector'
    - 'Rover Aggregate'
    - 'Rover Pilot Process Manager'
tags:
    - label: 'event'
externalLinks: []
badges: []
---
Rover positions change as a result of external stimuli, or _drivers_. When running alongside a simulator-mode driver, the positions will change according to the physics and velocity being managed by the simulator. When running in a "real" environment where live updates are received from an actual rover, the position change is reported as an aggregate of telemetry data.

There might be multiple drivers, including:

* Game driver
* Simulation driver for realistic experimentation
* Real driver, obtaining telemetry from hardware

All drivers operate in the event sourcing role of `injector`.

This event will only be emitted when a _whole_ grid unit has been traversed, and not during partial intervals.
<Mermaid />

## Schema
<SchemaViewer />
