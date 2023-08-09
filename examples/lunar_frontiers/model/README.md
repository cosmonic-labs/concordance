# Lunar Frontiers Shared Model
This crate contains the shared model for the Lunar Frontiers project. Here you'll find the commands, events, aggregate state, and other shared types used by the various services in the project. 

In a larger project or an application destined for production, you might not use a shared library but rather have each consuming crate
generate and compile the definitions locally. To avoid having to use two different IDLs (and to avoid using something like Smithy), this crate is a convenient way to share the definitions between the various event sourcing components.

Note that this doesn't mean that one aggregate can read the other aggregate's state, it just means that model definitions are defined in this shared crate.