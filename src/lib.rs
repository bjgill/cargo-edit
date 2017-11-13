//! Show and Edit Cargo's Manifest Files
#![cfg_attr(test, allow(dead_code))]
#![warn(missing_docs, missing_debug_implementations, missing_copy_implementations, trivial_casts,
        trivial_numeric_casts, unsafe_code, unstable_features, unused_import_braces,
        unused_qualifications)]

#[macro_use]
extern crate error_chain;
extern crate regex;
extern crate reqwest;
extern crate semver;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate termcolor;
extern crate toml;
extern crate tomllib;

mod dependency;
mod errors;
mod fetch;
mod manifest;
pub mod manifest_new;

pub use dependency::Dependency;
pub use errors::*;
pub use fetch::{get_crate_name_from_github, get_crate_name_from_gitlab, get_crate_name_from_path,
                get_latest_dependency};
pub use manifest::Manifest;
