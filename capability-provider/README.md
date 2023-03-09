# Concordance Event Sourcing Capability Provider
TBD


## Replay
TBD

Reset a consumer (replay to its interested party):
```
nats consumer info test foo --json > foo.json

nats consumer rm -f test foo && nats consumer add test --config foo.json
```