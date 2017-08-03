//! A simple crate exposing a struct `Foo` with a method `Bar` to C

pub use foo::Foo;
pub use ffi::{foo_create, foo_free, foo_bar};
mod foo;
mod ffi;