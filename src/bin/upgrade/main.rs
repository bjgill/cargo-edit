//! `cargo upgrade`

#![warn(missing_docs, missing_debug_implementations, missing_copy_implementations, trivial_casts,
        trivial_numeric_casts, unsafe_code, unstable_features, unused_import_braces,
        unused_qualifications)]

extern crate docopt;
extern crate pad;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::error::Error;
use std::io::{self, Write};
use std::process;

extern crate cargo_edit;
use cargo_edit::{Manifest, get_latest_dependency};

static USAGE: &'static str = r"
Upgrade dependencies in a manifest file to the latest version.

Usage:
    cargo upgrade [--dependency <dep>...] [--manifest-path <path>]
    cargo upgrade (-h | --help)
    cargo upgrade (-V | --version)

Options:
    -h, --help                   Print this message
    --dependency -d <dep>        Dependency to update
    --manifest-path <path>       Path to the crate's manifest
    -V --version                 Show version

Only dependencies from crates.io are supported. Git/path dependencies will be ignored.
";

/// Docopts input args.
#[derive(Debug, Deserialize)]
struct Args {
    /// `--dependency -d <dep>`
    flag_dependency: Vec<String>,
    /// `--manifest-path <path>`
    flag_manifest_path: Option<String>,
    /// `--version`
    flag_version: bool,
}

fn is_version_dependency(dep: &toml::Value) -> bool {
    dep.as_table()
        .map(|table| {
            // Not a version dependency if the `git` or `path` keys are present.
            !(table.contains_key("git") || table.contains_key("path"))
        })
        .unwrap_or(true)
}

fn update_manifest(
    manifest_path: &Option<String>,
    only_update: &[String],
) -> Result<(), Box<Error>> {
    let manifest_path = manifest_path.as_ref().map(From::from);
    let mut manifest = Manifest::open(&manifest_path).unwrap();

    // Look for dependencies in all three sections.
    for (table_path, table) in manifest.get_sections() {
        table
            .iter()
            .filter(|&(name, _old_value)| {
                // If the user specifies a list of dependencies, only update those dependencies.
                only_update.is_empty() || only_update.contains(name)
            })
            .filter(|&(_name, old_value)| is_version_dependency(old_value))
            .map(|(name, _old_value)| {

                let latest_version = get_latest_dependency(name, false)?;

                // Simply overwrite the old entry.
                manifest.update_table_entry(&table_path, &latest_version)?;

                Ok(())
            })
            .collect::<Result<Vec<_>, Box<Error>>>()?;
    }

    let mut file = Manifest::find_file(&manifest_path)?;
    manifest.write_to_file(&mut file)
}

fn main() {
    let args = docopt::Docopt::new(USAGE)
        .and_then(|d| d.deserialize::<Args>())
        .unwrap_or_else(|err| err.exit());

    if args.flag_version {
        println!("cargo-upgrade version {}", env!("CARGO_PKG_VERSION"));
        process::exit(0);
    }

    if let Err(err) = update_manifest(&args.flag_manifest_path, &args.flag_dependency) {
        writeln!(
            io::stderr(),
            "Command failed due to unhandled error: {}\n",
            err
        ).unwrap();
        process::exit(1);
    }
}
