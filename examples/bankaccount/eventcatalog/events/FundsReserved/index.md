---
name: FundsReserved
summary: "Indicates funds have been placed on hold for a wire transfer"
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
Indicates that the funds to be used in a wire transfer have been reserved/placed on hold.

<Mermaid />

## Schema
<SchemaViewer />