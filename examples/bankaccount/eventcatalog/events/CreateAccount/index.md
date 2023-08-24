---
name: CreateAccount
summary: "Requests the creation of a new bank account"
version: 0.0.1
consumers:
    - 'Bank Account Aggregate'
tags:
    - label: 'command'
externalLinks: []
badges: []
---
Requests the creation of a new bank account. This command can fail to process if the parameters are invalid or if the account already exists.

<Mermaid />

## Schema
<SchemaViewer />