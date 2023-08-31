---
name: FundsReleased
summary: "Indicates that reserved funds were released"
version: 0.0.1
consumers:
    - 'Bank Account Aggregate'
    - 'Wire Transfer Process Manager'
    - 'Bank Account Projector'
producers:
    - 'Bank Account Aggregate'
tags:
    - label: 'event'
externalLinks: []
badges: []
---
Indicates that held funds were released as part of a failed or canceled transfer.

<Mermaid />

## Schema
<SchemaViewer />