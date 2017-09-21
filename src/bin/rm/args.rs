//! Handle `cargo rm` arguments
use cargo_edit::DependencyKind;

#[derive(Debug, Deserialize)]
/// Docopts input args.
pub struct Args {
    /// Crate name
    pub arg_crate: String,
    /// dev-dependency
    pub flag_dev: bool,
    /// build-dependency
    pub flag_build: bool,
    /// `Cargo.toml` path
    pub flag_manifest_path: Option<String>,
    /// `--version`
    pub flag_version: bool,
}

impl Args {
    /// Get dependency type
    pub fn get_dependency_type(&self) -> DependencyKind {
        match (self.flag_dev, self.flag_build) {
            (true, false) => DependencyKind::Development,
            (false, true) => DependencyKind::Build,
            (false, false) => DependencyKind::Normal,
            (true, true) => unreachable!(),
        }
    }
}

impl Default for Args {
    fn default() -> Args {
        Args {
            arg_crate: "demo".to_owned(),
            flag_dev: false,
            flag_build: false,
            flag_manifest_path: None,
            flag_version: false,
        }
    }
}
