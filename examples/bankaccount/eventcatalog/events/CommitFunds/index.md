---
name: CommitFunds
summary: "A request to commit funds under hold to a wire transfer"
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
A request to commit the funds currently on hold for a given wire transfer.

<Mermaid />

## Schema
<SchemaViewer />