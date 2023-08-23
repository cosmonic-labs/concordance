---
name: FundsCommitted
summary: "Indicates that reserved funds were committed and withdrawn"
version: 0.0.1
consumers:
    - 'Bank Account Aggregate'
    - 'Wire Transfer Process Manager'
producers:
    - 'Bank Account Aggregate'
tags:
    - label: 'event'
externalLinks: []
badges: []
---
Indicates that previously held funds were withdrawn from the account. In the interest of simplicity, this example doesn't support partially committed funds or funds that are required to clear in increments of some small value.

<Mermaid />

## Schema
<SchemaViewer />