# Makefile for Concordance event sourcing provider
# used to generate a .par.gz file

PROJECT = concordance
CAPABILITY_ID = cosmonic:eventsourcing
VENDOR   = "Cosmonic"
NAME     = "Concordance Event Sourcing"
VERSION  = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] .version' | head -1)
REVISION = 0
oci_url  = localhost:5000/v2/$(PROJECT):$(VERSION)
oci_insecure = --insecure

include ./provider.mk

test:
	killall target/debug/concordance || true
	RUST_BACKTRACE=1 cargo test -- --test-threads=1 --nocapture