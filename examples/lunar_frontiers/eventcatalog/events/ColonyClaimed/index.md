---
name: ColonyClaimed
summary: "Indicates that a new colony has been claimed. Contains colony initialization parameters."
version: 1.0.0
producers:
    - 'Colony Aggregate'
consumers:
    - 'Colony Aggregate'
    - 'Colony Projector'
tags:
    - label: 'event'
externalLinks: []
badges: []
---
The colony claimed event signals the very beginning of a colony on the moon. No rovers can be deployed nor can any activity take place until the colony has been claimed. This event contains startup parameters for the colony like the height and width of the surface plot boundaries.

As the simulation becomes more robust, more parameters will likely be added to this event.

<Mermaid />

## Schema
<SchemaViewer />
