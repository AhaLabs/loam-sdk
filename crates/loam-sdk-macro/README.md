# loam-sdk-macro

This crate contains the source for the macros that all subcontracts depend on, such as the `#[subcontract]` macro itself. Rust macros generate code, allowing users to write less. These macros generate the code necessary for all Subcontracts, and do so in a way that makes it easy to author your own Subcontracts.

`#[subcontract]` is an [attribute procedural macro](https://doc.rust-lang.org/reference/procedural-macros.html#:~:text=Attribute%20macros%20are%20defined%20by,not%20including%20the%20outer%20delimiters.) (proc macro) that you need when you create your own subcontracts.

Aside from `#[subcontract]`, this crate also contains the implementation for some [derive macros](https://veykril.github.io/tlborm/proc-macros/methodical/derive.html) such as `IntoKey`, which structs in the subcontract need to derive in order to lazily load and store the type. 

For more information about how to use and author Subcontracts, see the [loam-sdk README](../loam-sdk/README.md).

See [lib.rs](src/lib.rs) for the implementations of `subcontract`, `IntoKey`, and other macros.