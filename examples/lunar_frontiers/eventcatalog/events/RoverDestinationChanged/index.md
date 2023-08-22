---
name: RoverDestinationChanged
summary: "Indicates that the destination of a rover has been changed"
version: 1.0.0
producers:
    - 'Rover Aggregate'
consumers:
    - 'Rover Aggregate'
    - 'Rover Projector'
    - 'Rover Pilot Process Manager'
    
externalLinks: []
tags:
    - label: 'event'
badges: []
---
Rovers can change their destinations at the behest of the _pilot_ software (wasm component) running on the rover.  This event is published when the destination of a rover is changed. The rate at which a rover will advance toward its destination is determined entirely by the current _driver_, and is measured in terms of _ticks_ required to advance.

<Mermaid />

## Schema
<SchemaViewer />
