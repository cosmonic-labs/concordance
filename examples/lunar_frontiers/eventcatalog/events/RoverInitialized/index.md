---
name: RoverInitialized
summary: "Indicates that a rover has been initialized on a given moon"
version: 1.0.0
producers:
    - 'Rover Aggregate'
consumers:
    - 'Rover Aggregate'
    - 'Rover Projector'
    - 'Rover Pilot Process Manager'
externalLinks: []
badges: []
tags:
    - label: 'event'      
---
Once a colony has been claimed, a mothership may deploy a rover to the surface of the moon. This event indicates that a rover has been initialized on a given moon and contains reference keys for the moon and mothership, as well as an indicator of the _pilot_ (a WebAssembly component) that will drive the rover.

<Mermaid />

## Schema
<SchemaViewer />
