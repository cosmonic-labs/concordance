# Concordance Event Sourcing Capability Provider
This is the [wasmCloud](https://wasmcloud.com) capability provider for the Concordance event sourcing framework. This provider
and all associated developer experiences are under heavy, rapidly iterating development at the moment. API surface area and even
binary compatibility may change frequently at this stage.


## Replay
⚠️ _under construction_: The following is more of a note to self on how to manually initiate a replay of a given consumer

Reset a consumer (replay to its interested party):
```
nats consumer info test foo --json > foo.json

nats consumer rm -f test foo && nats consumer add test --config foo.json
```