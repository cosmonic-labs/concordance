# uncomment if you want to start fresh
# nats stream purge CC_EVENTS -f
# nats stream purge CC_COMMANDS -f

nats req cc.commands.bankaccount "`cat ibt_create_account_cmd.json | jq -c`"
nats req cc.commands.bankaccount "`cat ibt_request_wire_xfer.json | jq -c`"

nats kv get CC_STATE agg.bankaccount.834597DRML130 --raw | jq
nats kv get CC_STATE pm.interbankxfer.9c0562bb-6657-4918-ad21-bec63f38bc10 --raw | jq
