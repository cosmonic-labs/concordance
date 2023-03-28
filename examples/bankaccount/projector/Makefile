# examples/actor/echo

PROJECT  = bankaccount_projector
VERSION  = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] .version' | head -1)
REVISION = 1
# list of all contract claims for actor signing (space-separated)
CLAIMS   = cosmonic:eventsourcing wasmcloud:keyvalue
# registry url for our actor
REG_URL  = localhost:5000/v2/$(PROJECT):$(VERSION)
# command to upload to registry (without last wasm parameter)
PUSH_REG_CMD = wash reg push --insecure $(REG_URL)
ACTOR_NAME = "Bank Account Projector"

include ../../../build/actor.mk