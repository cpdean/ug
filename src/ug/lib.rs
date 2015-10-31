// totally don't get this, just copying from racer
#![cfg_attr(all(test, feature = "nightly"), feature(test))] // we only need test feature when testing

extern crate regex;

#[macro_use]
pub mod core;
