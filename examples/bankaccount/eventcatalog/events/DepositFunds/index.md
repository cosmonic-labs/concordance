---
name: DepositFunds
summary: "A request to deposit funds into an account"
version: 0.0.1
consumers:
    - 'Bank Account Aggregate'
tags:
    - label: 'command'
externalLinks: []
badges: []
---
Requests the deposit of a specified amount into the account. This command can fail to process if the parameters are invalid.

<Mermaid />

## Schema
<SchemaViewer />