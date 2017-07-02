//! `cargo upgrade`

#![warn(missing_docs, missing_debug_implementations, missing_copy_implementations, trivial_casts, trivial_numeric_casts, unsafe_code, unstable_features, unused_import_braces, unused_qualifications)]

extern crate docopt;
extern crate pad;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::error::Error;
use std::io::{self, Write};
use std::process;

extern crate cargo_edit;
use cargo_edit::{Manifest, fetch};

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

fn update_manifest(
    manifest_path: &Option<String>,
    only_update: &[String],
) -> Result<(), Box<Error>> {
    let manifest_path = manifest_path.as_ref().map(From::from);
    let mut manifest = Manifest::open(&manifest_path).unwrap();
    let manifest_input = manifest.clone();

    // Look for dependencies in all three sections.
    for section in &["dev-dependencies", "build-dependencies", "dependencies"] {
        let dependencies = match manifest_input.data.get(&section.to_string()) {
            Some(&toml::Value::Table(ref table)) => table,
            // It's possible for some/all sections to be missing. We need not consider those
            // further.
            _ => continue,
        };

        dependencies
            .iter()
            .filter(|&(name, _value)| {
                // If the user specifies a list of dependencies, we only update those dependencies.
                only_update.is_empty() || only_update.contains(name)
            })
            .map(|(name, _value)| {

                let latest_version = fetch::get_latest_dependency(name, false)?;

                // Simply overwrite the old entry.
                manifest.update_table_entry(
                    &[section.to_string()],
                    &latest_version,
                )?;

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
