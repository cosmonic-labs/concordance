# Concordance Code Generator
This crate is an _opt-in_ code generation tool for developers building applications using Concordance. If you are building
your event sourced components in **Rust**, then you'll be able to use this crate from your event sourced (wasm) component by using the `generate!` macro.

This code generator works by being pointed at an [eventcatalog](https://www.eventcatalog.dev/) site as the single source of truth for the event flow and event schemas. Simply tell the code generator where the site is, and the name and type of component you're generating, and it will take care of creating the appropriate trait for you to implement.

Internally, the `eventsourcing` model contains the core data types for dealing with the `cosmonic:eventsourcing` wasmCloud capability contract. Developers using this crate do not need to interact with that interface directly.

## Usage
To use this crate, you'll need to add it to your `Cargo.toml` file as a dependency. Then, you can use the `generate` macro:

```rust
use bankaccount_model::state::*;

use anyhow::Result;

use serde::{Deserialize, Serialize};
use wasmcloud_interface_logging::error;

concordance_gen::generate!({
    path: "../eventcatalog",
    role: "aggregate",
    entity: "bankaccount"
});
```

Note that _all_ of the data types involved in this flow are generated from the JSON schemas found alongside their markdown documentation. You do not need to create any data types unless you're building a _stateful_ component (aggregate, process manager). Then you'll need to create a state struct that conforms to Concordance's naming convention.

The valid roles are the same as the list of valid roles in Concordance link definitions:

* `aggregate`
* `projector`
* `process_manager`
* `notifier`

