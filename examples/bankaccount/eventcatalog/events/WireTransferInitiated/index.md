---
name: WireTransferInitiated
summary: "Indicates that a wire transfer process has begun"
version: 0.0.1
consumers:    
    - 'Bank Account Projector'
    - 'Wire Transfer Process Manager'
    - 'Bank Account Aggregate'
producers:
    - 'Bank Account Aggregate'
tags:
    - label: 'event'
externalLinks: []
badges: []
---
Indicates that the **process** of a wire transfer has been initiated. External stimuli from a gateway can then emit events to indicate the completion (successful or otherwise) of this process. Funds involved in the transfer are _reserved_ from the account, but not yet _withdrawn_. The funds will either be released or fully withdrawn pending the outcome of the transfer.

<Mermaid />

## Schema
<SchemaViewer />