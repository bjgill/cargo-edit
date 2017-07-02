//! Show and Edit Cargo's Manifest Files

#![cfg_attr(test, allow(dead_code))]
#![warn(missing_docs, missing_debug_implementations, missing_copy_implementations, trivial_casts, trivial_numeric_casts, unsafe_code, unstable_features, unused_import_braces)]

extern crate cargo;
#[macro_use]
extern crate quick_error;
extern crate regex;
extern crate reqwest;
extern crate semver;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate toml;

pub mod fetch;
mod manifest;
mod dependency;

pub use dependency::Dependency;
pub use manifest::Manifest;
