---
name: ConstructionCompleted
summary: "Indicates that construction has completed"
version: 1.0.0
producers:
    - 'Building Aggregate'
consumers:
    - 'Building Aggregate'
    - 'Rover Pilot Process Manager'
tags:
    - label: 'event'
externalLinks: []
badges: []
---
Rovers can construct buildings. They must be in the grid coordinate where they intend to construct the building, and they must remain in that grid coordinate until construction is complete. 

This event is published when construction on a given building is complete.

<Mermaid />

## Schema
<SchemaViewer />
