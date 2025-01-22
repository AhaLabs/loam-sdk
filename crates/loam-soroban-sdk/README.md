# loam-soroban-sdk

Wrapper around Soroban SDK which adds features needed by Loam SDK, such as:

 - lazy getting and setting
 - implementation of `loamstorage` macro 
 - *deprecated* implementation of Loam's`IntoKey` derive macro that relies on Soroban SDK's specifics 
 - abstraction of Soroban SDK's `env` to enable Loam SDK-authored contracts to avoid referencing it at all and use Rust's standard mutable/immutable method definition syntax (`my_method(&self, ...)` vs `my_method(&mut self, ...)`).