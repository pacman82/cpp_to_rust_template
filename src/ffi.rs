//! Foreign Function Interface
//!
//! Exposes `Foo`s creation, destruction and usage with a C ABI
use super::*;

/// Returns an error code. 0 suggests successful creation of a foo. We take a pointer to pointer to
/// return the Foo to the user.
pub extern fn foo_create(out: * mut * mut Foo) -> i32{
    let foo = Box::new(Foo::new());
    unsafe{
        //pass ownership
        * out = Box::into_raw(foo);
    }
    // Everything went fine
    0
}

pub extern fn foo_free(foo: * mut Foo){
    unsafe {
        // take ownership and destroy at end of scope
        Box::from_raw(foo);
    }
}

pub extern fn foo_bar(foo: * const Foo){
    unsafe {
        (*foo).bar()
    }
}