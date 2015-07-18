// Copyright 2014 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_name = "string_cache"]
#![crate_type = "rlib"]

#![deny(warnings)]
#![cfg_attr(test, feature(test, filling_drop))]
#![cfg_attr(bench, feature(rand))]
#![cfg_attr(feature = "unstable", feature(unsafe_no_drop_flag, plugin))]
#![cfg_attr(feature = "unstable", plugin(string_cache_plugin))]

#[cfg(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

extern crate rand;

#[cfg(feature = "log-events")]
extern crate rustc_serialize;

extern crate serde;

extern crate string_cache_shared;

pub use atom::Atom;
pub use namespace::{Namespace, QualName};

#[macro_export]
macro_rules! qualname (($ns:tt, $local:tt) => (
    ::string_cache::namespace::QualName {
        ns: ns!($ns),
        local: atom!($local),
    }
));

#[cfg(not(feature = "unstable"))]
include!(concat!(env!("OUT_DIR"), "/ns_macro_without_plugin.rs"));

#[cfg(feature = "log-events")]
#[macro_use]
pub mod event;

pub mod atom;
pub mod namespace;

// A private module so that macro-expanded idents like
// `::string_cache::atom::Atom` will also work in this crate.
//
// `libstd` uses the same trick.
#[doc(hidden)]
mod string_cache {
    pub use atom;
    pub use namespace;
}
