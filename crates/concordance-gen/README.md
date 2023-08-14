# Concordance Code Generator
This crate is an _opt-in_ code generation tool for developers building applications using Concordance. If you are building
your event sourced components in **Rust**, then you'll be able to use this crate from your event sourced (wasm) component by using the `generate!` macro.

## Usage
To use this crate, you'll need to add it to your `Cargo.toml` file as a dependency. Then, you can use the `generate` macro:

```rust
use bankaccount_model::commands::*;
use bankaccount_model::events::*;
use bankaccount_model::state::*;

use anyhow::Result;

// use concordance_codegen::eventsourcing:: ??

use serde::{Deserialize, Serialize};
use wasmcloud_interface_logging::error;

concordance_gen::generate!({
    path: "../bankaccount-model.ttl",
    role: "aggregate",
    entity: "bankaccount"
});
```

It's important to note that it is the developer's responsibility to ensure that all of the "model" types are available to the generate macro, since that macro makes a number of assumptions about the availability of types when it generates traits and implementations.

The valid roles are the same as the list of valid roles in Concordance link definitions:
* aggregate
* projector
* process_manager
* notifier

## Generating Documentation
You can produce markdown documentation by invoking the `generate_doc` function inside your `build.rs` and pointing it at the model (Turtle RDF) file. The markdown documentation will produce index files enumerating all of the events, commands, aggregates, projectors, and process managers in your even model, as well as linking to the corresponding events and commands in describing the inbound and outbound flows.