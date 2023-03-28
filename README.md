# Concordance from Cosmonic
Concordance is an open source, opinionated event sourcing framework from Cosmonic. Building on the power, portability, speed, and scalability of [wasmCloud](https://wasmcloud.com), Concordance allows you to reason about your application using the fundamental building blocks of event sourcing:
* **Aggregates** - WebAssembly components that derive state from an event stream, validate incoming commands, and emit events in response to those commands.
* **Projectors** - WebAssembly components that _project_ read-optimized data (aka materialized views) based on streams of incoming events
* **Process Managers** - Known as _sagas_ in other paradigms, a process manager is the inverse of an aggregate: it processes inbound events and starts, advances, and stops processes by emitting the corresponding commands.
* **Stateless Event Handlers** - used for entities such as **notifiers** and **gateways**, you can build WebAssembly components that receive events on a stream and then execute whatever you choose for your business logic.

_"Contraints liberate, liberties constrain"_ - Runar Bjarnason

This quote is a foundational principle behind this event sourcing library. You'll find that aggregates are unable to do anything other than return a list of events. They are unable to produce side-effects, which is a good thing:tm:. Likewise, process managers cannot produce side-effects, they can only return a list of commands. Side-effects are explicitly forced into the perview of notifiers (stateless event handlers).

For information on how to operate Concordance-based applications in production as well as how to build applications using Concordance, please take a look at the [developer's guide](https://cosmonic.com/docs/oss/concordance).

⚠️ **Under Development** ⚠️ - _Concordance is under heavy development right now and, as such, the APIs and developer experience will be rapidly changing (for the better). Building applications on it now should be done with the idea of rapid change in mind.