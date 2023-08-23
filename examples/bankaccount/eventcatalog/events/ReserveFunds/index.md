---
name: ReserveFunds
summary: "A request to place wire transfer funds on hold"
version: 0.0.1
consumers:
    - 'Bank Account Aggregate'
producers:
    - 'Wire Transfer Process Manager'
tags:
    - label: 'command'
externalLinks: []
badges: []
---
A request to place on hold the funds to be involved in a given wire transfer

<Mermaid />

## Schema
<SchemaViewer />