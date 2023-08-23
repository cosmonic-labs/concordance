---
name: Wire Transfer Process Manager
summary: |
  The process manager for managing wire transfer processes
tags:
    - label: 'procman'
---

This process manager is responsible for managing the process of wire transfers. It listens for the `WireTransferInitiated` event and then emits the appropriate commands to continue the process

<Mermaid/>

## Interest
The following indicates the sequential flow of the process manager's interest, which is required for defining link definitions. It's important to note that the process doesn't complete when it receives the fail/succeed events from the outside world. The process is only considered completed when the funds held by the wire transfer are released or committed.

* `start` - [WireTransferInitiated](../../events/WireTransferInitiated)
* `advance` - [FundsReserved](../../events/FundsReserved), [WireTransferSucceeded](../../events/WireTransferSucceeded), [WireTransferFailed](../../events/WireTransferFailed)
* `end` - [FundsCommitted](../../events/FundsCommitted), [FundsReleased](../../events/FundsReleased)