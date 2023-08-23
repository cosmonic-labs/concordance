---
name: ReleaseFunds
summary: "A request to release funds from a wire transfer"
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
Requests that funds held for a given wire transfer are to be released. Note that this command can be rejected if no such 
wire transfer is known to the aggregate.

<Mermaid />

## Schema
<SchemaViewer />