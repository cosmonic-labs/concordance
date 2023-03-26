# Run this script after interbank_1 to simulate a failure from an interbank gateway
nats req cc.events.interbank_transfer_failed "`cat ibt_failed.json | jq -c`"
