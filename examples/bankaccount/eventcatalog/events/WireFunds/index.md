---
name: WireFunds
summary: "A request to wire funds to another account at another bank"
version: 0.0.1
consumers:
    - 'Bank Account Aggregate'
tags:
    - label: 'command'
externalLinks: []
badges: []
---
Requests the wiring of a specified amount to another account at another bank. This command can fail to process if the parameters are invalid or if the source account does not have sufficient funds. This will result in the _holding_ of the funds until the wire is completed or cancelled.

<Mermaid />

## Schema
<SchemaViewer />