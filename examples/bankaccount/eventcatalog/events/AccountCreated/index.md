---
name: AccountCreated
summary: "Indicates the creation of a new bank account"
version: 0.0.1
consumers:
    - 'Bank Account Aggregate'
    - 'Bank Account Projector'
producers:
    - 'Bank Account Aggregate'
tags:
    - label: 'event'
externalLinks: []
badges: []
---
Indicates that a bank account has been created. As with all events, this is immutable truth.

<Mermaid />

## Schema
<SchemaViewer />