---
name: RoverStarted
summary: "Indicates that a rover has started moving"
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

This event is published when a rover transitions from a stationary state to moving. 
<Mermaid />

## Schema
<SchemaViewer />
