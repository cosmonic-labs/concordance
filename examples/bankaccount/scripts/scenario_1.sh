# /bin/sh
# This script assumes the existence of the nats command line tool and a running NATS server that has JetStream enabled.
# It also requires the following:
# * Concordance capability provider running in a host
# * All relevant actors (agg, pm, proj, etc) running
# * All linkdefs set
# * the jq command installed

# uncomment if you want to start fresh
# nats stream purge CC_EVENTS -f
# nats stream purge CC_COMMANDS -f

nats req cc.commands.bankaccount "`cat create_account_cmd.json | jq -c`"
nats req cc.commands.bankaccount "`cat deposit_cmd_1.json | jq -c`"
nats req cc.commands.bankaccount "`cat deposit_cmd_2.json | jq -c`"
nats req cc.commands.bankaccount "`cat withdraw_cmd_1.json | jq -c`"

nats kv get CC_STATE agg.bankaccount.ABC123

