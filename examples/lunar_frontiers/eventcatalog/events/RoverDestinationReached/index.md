---
name: RoverDestinationReached
summary: "Indicates that a rover has reached its most recent destination"
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
This event is emitted when the rover moves into a new grid coordinate and that coordinate is the same as its current destination. This is emitted _in addition to_ any other motion-related events and the rover's current destination will be cleared after this, transitioning the rover into a stopped state with no destination.

<Mermaid />

## Schema
<SchemaViewer />
