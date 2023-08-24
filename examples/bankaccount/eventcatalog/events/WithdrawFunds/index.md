---
name: WithdrawFunds
summary: "A request to withdraw funds from an account"
version: 0.0.1
consumers:
    - 'Bank Account Aggregate'
tags:
    - label: 'command'
externalLinks: []
badges: []
---
Requests the withdrawal of a specified amount from the account. This command can fail to process if the parameters are invalid or if the account does not have sufficient funds.

Note that there is a design decision here. You can allow the withdrawal to go through even if there is insufficient funds, and then also emit an overdraft event. Or all commands attempting to withdraw below the minimum (or 0 if omitted) are rejected. This is a domain/application decision and
not really something that can be decided by the framework.

<Mermaid />

## Schema
<SchemaViewer />