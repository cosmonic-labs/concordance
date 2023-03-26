# Bank Account Aggregate
This aggregate represents the sum of events on the `bankaccount` stream, which is keyed by the account number on the commands and events in this logical stream.

# Configuration
The following configuration values should be set for this aggregate to work properly.
* `ROLE` - `aggregate`
* `INTEREST` - `bankaccount`
* `NAME` - `bankaccount`
* `KEY` - `account_number`

# Manual Testing
You can send the following commands manually to watch the aggregate perform its tasks:

## Creating an Account
You can use the following `nats req` command (edit the data as you see fit) to create a new account by submitting a new `create_account` command:
```
nats req cc.commands.bankaccount '{"command_type": "create_account", "key": "ABC123", "data": {"account_number": "ABC123", "initial_balance": 4000, "min_balance": 100, "customer_id": "CUSTBOB"}}'
```
You should receive a reply that looks something like this:
```
11:25:05 Sending request on "cc.commands.bankaccount"
11:25:05 Received with rtt 281.083µs
{"stream":"CC_COMMANDS", "seq":2}
```

And now you can verify that you have indeed created the `ABC123` account (note the key is account number and not customer ID).
```
nats kv get CC_STATE agg.bankaccount.ABC123
CC_STATE > agg.bankaccount.ABC123 created @ 20 Mar 23 15:25 UTC

{"balance":4000,"min_balance":100,"account_number":"ABC123"}
```

