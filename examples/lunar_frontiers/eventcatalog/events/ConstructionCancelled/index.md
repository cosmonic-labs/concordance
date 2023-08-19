---
name: ConstructionCancelled
summary: "Indicates that a construction project has been aborted"
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

The construction process of any site (except `headquarters`) can be cancelled. This event will be emitted when the construction has been cancelled. Rovers have the option of specifying free-form text to indicate the reason for the cancellation for audit purposes.

<Mermaid />

## Schema
<SchemaViewer />
