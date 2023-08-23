---
name: WireTransferFailed
summary: "Indicates that a wire transfer process failed"
version: 0.0.1
consumers: 
    - 'Wire Transfer Process Manager'
tags:
    - label: 'event'
    - label: 'external'
externalLinks: []
badges: []
---
This event is published from an external source to indicate that the wire transfer process failed.
Note that this event doesn't have any internal information like customer ID because it originates from outside the system.

<Mermaid />

## Schema
<SchemaViewer />