//! WIP to switch `cargo upgrade` over to use tomllib. Needs comments, proper
//! handling of the 3 dependency kinds (raw, table, and inline table), error handling.
#![allow(missing_docs, missing_debug_implementations)]

use std::fs::{self, File, OpenOptions};
use std::io::Read;
use std::path::{Path, PathBuf};

use tomllib::TOMLParser;
use tomllib::types::{Children, Value};

pub struct Manifest<'a> {
    parser: TOMLParser<'a>,
    // contents: String
}

#[derive(Debug)]
pub struct Dependency {
    name: String,
    path: String,
}

impl<'a> Manifest<'a> {
    pub fn open(contents: &'a str) -> Self {
        let parser = TOMLParser::new();
        let (parser, _result) = parser.parse(contents);

        Self { parser }
    }

    // c.f. `manifest::get_sections`. Could do with some refactoring...
    pub fn get_all_deps(&self) -> Vec<Dependency> {
        let mut dependencies = Vec::new();

        for dependency_type in &["dev-dependencies", "build-dependencies", "dependencies"] {
            // Dependencies can be in the three standard sections...
            match self.parser.get_children(*dependency_type) {
                Some(&Children::Keys(ref deps)) => deps.borrow().iter().for_each(|dep| {
                    dependencies.push(Dependency {
                        name: dep.clone(),
                        path: Children::combine_keys(dependency_type.to_string(), dep.clone()),
                    })
                }),
                _ => (),
            }

            // ... and in `target.<target>.(build-/dev-)dependencies`.
            if let Some(&Children::Keys(ref targets)) = self.parser.get_children("target") {
                for target in targets.borrow().iter() {
                    let target_path = Children::combine_keys("target", target);
                    let path = Children::combine_keys(target_path, dependency_type.to_string());

                    if let Some(&Children::Keys(ref target_deps)) =
                        self.parser.get_children(path.clone())
                    {
                        target_deps.borrow().iter().for_each(|dep| {
                            dependencies.push(Dependency {
                                name: dep.clone(),
                                path: Children::combine_keys(path.clone(), dep.to_string()),
                            })
                        });
                    }
                }
            }
        }

        dependencies
    }

    // Very basic - need to handle the case where we get a table.
    pub fn set_dep_version(&mut self, dependency: Dependency, version: String) {
        self.parser.set_value(dependency.path, Value::basic_string(version).unwrap());
    }
}

mod test {
    use super::*;

    use tomllib::TOMLParser;
    use tomllib::types::Value;

    #[test]
    fn open_manifest() {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open("Cargo.toml")
            .unwrap();

        let mut contents = String::new();

        file.read_to_string(&mut contents);

        let manifest = Manifest::open(&contents);

        panic!("{:?}", manifest.get_all_deps());
    }

    #[test]
    fn try_getting_deps() {
        let parser = TOMLParser::new();
        let toml_doc = include_str!("../Cargo.toml");

        // Get back the parser and a result from the parse method in a tuple
        let (mut parser, result) = parser.parse(toml_doc);

        let val = parser.get_value("dependencies.docopt");

        panic!("{:?}", val);
    }
}
