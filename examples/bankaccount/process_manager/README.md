# Bank Account Sample - Interbank Transfer Process Manager
In event sourcing lingo, a _process manager_ is roughly equivalent to the domain-driven design concept of a _saga_. A process manager is responsible for coordinating any long-running process by consuming events and emitting commands (essentially the inverse of an aggregate). Note that "long-running" in this case refers to the plurality of events involved in any given process and not the passage of real time.

# Configuration
Use the following configuration settings on a link definition to set up the process manager:
* `ROLE` - `process_manager`
* `NAME` - `interbank_xfer` (results in a consumer named `PM_interbank_xfer`)
* `INTEREST` - A JSON payload with the fields `start`, `advance` (list), and `stop` (list). For an example see the [interest.json](./interest.json) file. This declaration of interest does _not_ change across environments, as it dictates the list of events dispatched to a process manager and when processes are started and stopped.

Note that you won't be able to create this link definition in the dashboard from the wasmCloud OTP host, you'll need to use `wash`, as shown below (note the public keys in your environment will likely be different):

```
wash ctl link put MC5EQZ6NZY2T5US5JJTCJVAWHETCIIZLLVFUPERSTO2T3AR2NF62JWKI VAUL6FF47DZIMDOGNWZBCPVABWDK3UEZ5U7DGAM4HCKCHOEPTBZHGTAM cosmonic:eventsourcing ROLE=process_manager NAME=interbankxfer INTEREST='{"start":"wire_transfer_requested","advance":["wire_funds_reserved","interbank_transfer_initiated"],"stop":["interbank_transfer_completed","interbank_transfer_failed"]}'
```

TBD