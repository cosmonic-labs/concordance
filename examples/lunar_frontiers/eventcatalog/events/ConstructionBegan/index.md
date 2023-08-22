---
name: ConstructionBegan
summary: "Indicates that construction has begun on a building"
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
Rovers can construct buildings. They must be in the grid coordinate where they intend to construct the building, and they must remain in that grid coordinate until construction is complete. The construction process can be canceled and, if not canceled, will take `completionTicks` time slices to complete.

Note that "progress" events won't be emitted. Drivers know how long the construction will take in terms of ticks, so they can calculate the progress themselves.

<Mermaid />

## Schema
<SchemaViewer />
