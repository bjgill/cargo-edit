//! `cargo upgrade`

#![warn(missing_docs, missing_debug_implementations, missing_copy_implementations, trivial_casts,
        trivial_numeric_casts, unsafe_code, unstable_features, unused_import_braces,
        unused_qualifications)]

extern crate cargo;
extern crate docopt;
extern crate pad;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::error::Error;
use std::io::{self, Write};
use std::process;

use cargo::Config;
use cargo::core::{Dependency, Package};
use cargo::core::shell::{ColorConfig, Verbosity};
use cargo::util::important_paths::find_root_manifest_for_wd;

extern crate cargo_edit;
use cargo_edit::{Manifest, get_latest_dependency};

static USAGE: &'static str = r"
Upgrade all dependencies in a manifest file to the latest version.

Usage:
    cargo upgrade [--dependency <dep>...] [--manifest-path <path>]
    cargo upgrade (-h | --help)
    cargo upgrade (-V | --version)

Options:
    -d --dependency <dep>       Specific dependency to upgrade. If this option is used, only the
                                specified dependencies will be upgraded.
    --manifest-path <path>      Path to the manifest to upgrade.
    -h --help                   Show this help page.
    -V --version                Show version.

Dev, build, and all target dependencies will also be upgraded. Only dependencies from crates.io are
supported. Git/path dependencies will be ignored.
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
    all_dependencies: &[Dependency],
) -> Result<(), Box<Error>> {
    let manifest_path = manifest_path.as_ref().map(From::from);
    let mut manifest = Manifest::open(&manifest_path).unwrap();

    all_dependencies
        .into_iter()
        .filter(|dependency| {
            // If the user specifies a list of dependencies, only update those dependencies.
            only_update.is_empty() || only_update.contains(&dependency.name().to_string())
        })
        .filter(|dependency| dependency.source_id().is_registry())
        .map(|dependency| {
            let name = dependency.name();

            let latest_version = get_latest_dependency(name, false)?;

            let new = latest_version
                .version()
                .ok_or("Failed to get latest version")?;

            let new_dependency = dependency
                .clone_inner()
                .set_version_req(new.parse()?)
                .into_dependency();

            manifest.update_dependency(&new_dependency)?;

            Ok(())
        })
        .collect::<Result<Vec<_>, Box<Error>>>()?;

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

    let cargo_config = match Config::default() {
        Ok(cfg) => cfg,
        Err(e) => {
            let mut shell = cargo::shell(Verbosity::Verbose, ColorConfig::Auto);
            cargo::exit_with_error(e.into(), &mut shell)
        }
    };

    let root = find_root_manifest_for_wd(args.flag_manifest_path.clone(), cargo_config.cwd())
        .unwrap();

    let pkg = Package::for_path(&root, &cargo_config).unwrap();

    let dependencies = pkg.manifest().dependencies();

    if let Err(err) = update_manifest(
        &args.flag_manifest_path,
        &args.flag_dependency,
        dependencies,
    ) {
        writeln!(
            io::stderr(),
            "Command failed due to unhandled error: {}\n",
            err
        ).unwrap();
        process::exit(1);
    }
}
