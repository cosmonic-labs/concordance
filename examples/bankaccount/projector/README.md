# Bank Account Projector
This projector is responsible for storing read-optimized view data for bank accounts as a function application over the stream of inbound bank account events.

This projector maintains the following projections:
* **balances** - The current balance of any account can be looked up immediately by querying the key `balance.{account_number}`
* **ledger** - The ledger (chronological transaction history) of any account can be received as a JSON string via the key `ledger.{account_number}`

⚠️ NOTE: for testing purposes please don't use non-alphanumeric characters for the fake account numbers as it could potentially mess up key value storage depending on the chosen provider's support for complex keys.

# Configuration
This actor needs to be linked (bound) to two capability providers. One must support the `cosmonic:eventsourcing` contract. The Concordance provider for this contract requires the following configuration:

* `ROLE` - `projector`
* `INTEREST` - `account_created,funds_deposited,funds_withdrawn,wire_funds_reserved,wire_funds_released`
* `NAME` - `bankaccount_projector`

Note that stateless event handlers (whether you're using them as projectors, notifiers, gateways, etc) must declare their interest in events _explicitly_ in a comma-delimited list. Because of the use of commas in this data, it's probably easier and more reliable to use `wash ctl put link` rather than using the graphical wasmCloud dashboard.

This actor will also need to be linked to a `wasmcloud:keyvalue` capability provider, the implementation of which is entirely up to the developer and the configuration is likely specific to the implementation chosen (e.g. Redis vs NATS, etc).

# Manual Testing
You can start a wasmCloud host, start all of the bank account actors, and then start both the Concordance provider and your key-value provider of choice. Set the link definitions accordingly and then run the `scenario_1.sh` script in the [scripts](../scripts/) directory. You should then see the aggregate state stored in the `CC_STATE` bucket, the resulting events in the `CC_EVENTS` stream, and, assuming you used Redis, you'll see a balance projection in `balance.ABC123` and the ledger JSON structure in `ledger.ABC123`.
