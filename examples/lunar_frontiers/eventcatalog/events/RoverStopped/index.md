---
name: RoverStopped
summary: "Indicates that a rover has stopped moving"
version: 1.0.0
producers:
    - 'Rover Aggregate'
consumers:
    - 'Rover Aggregate'
    - 'Rover Projector'
    - 'Rover Pilot Process Manager'
tags:
    - label: 'event'
externalLinks: []
badges: []
---
This event is published when a rover transitions from moving to stopped. This event is always published when this happens, even when the rover reaches a destination and that event is also published.

<Mermaid />

## Schema
<SchemaViewer />
