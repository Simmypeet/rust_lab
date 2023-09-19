use std::{
    error::Error,
    fs::Metadata,
    os::unix::prelude::OsStrExt,
    path::{Path, PathBuf},
};

use clap::{App, Arg};
use regex::bytes::Regex;
use walkdir::WalkDir;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EntryType {
    Dir,
    File,
    Link,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub paths: Vec<PathBuf>,
    pub regexes: Vec<Regex>,
    pub entry_types: Vec<EntryType>,
}

impl Config {
    pub fn get() -> Result<Self, Box<dyn Error>> {
        let app = App::new("findr")
            .about("The rust clone of `findr`")
            .author("66011245@kmitl.ac.th")
            .arg(
                Arg::with_name("path")
                    .value_name("PATH")
                    .multiple(true)
                    .takes_value(true)
                    .help("Search paths")
                    .default_value("."),
            )
            .arg(
                Arg::with_name("name")
                    .value_name("NAME")
                    .long("name")
                    .short("n")
                    .takes_value(true)
                    .multiple(true)
                    .help("Name"),
            )
            .arg(
                Arg::with_name("type")
                    .value_name("TYPE")
                    .long("type")
                    .short("t")
                    .takes_value(true)
                    .multiple(true)
                    .help("Entry type")
                    .possible_values(&["f", "d", "l"]),
            );

        let matches = app.get_matches();

        let paths = matches
            .values_of("path")
            .expect("at least there should be a single \".\" path as a default value")
            .map(PathBuf::from)
            .collect();

        let regexes: Result<Vec<_>, _> = matches
            .values_of("name")
            .into_iter()
            .flat_map(IntoIterator::into_iter)
            .map(|x| Regex::new(x).map_err(|_| format!("Invalid --name \"{x}\"")))
            .collect();

        let entry_types = matches
            .values_of("type")
            .into_iter()
            .flat_map(IntoIterator::into_iter)
            .map(|x| match x {
                "f" => EntryType::File,
                "d" => EntryType::Dir,
                "l" => EntryType::Link,
                _ => unreachable!("should've been filtered out by clap"),
            })
            .collect();

        Ok(Config {
            paths,
            regexes: regexes?,
            entry_types,
        })
    }
}

#[derive(Debug)]
pub struct IoErrorWithPath {
    pub path: Option<PathBuf>,
    pub error: std::io::Error,
}

fn filter(path: &Path, metadata: &Metadata, entries_type: &[EntryType], regexes: &[Regex]) -> bool {
    if !regexes.is_empty() {
        let match_regex = |regex: &Regex| regex.is_match(path.file_name().unwrap().as_bytes());
        if !regexes.iter().any(match_regex) {
            return false;
        }
    }

    if entries_type.is_empty() {
        true
    } else {
        let match_type = |entry_type| match entry_type {
            EntryType::Dir => metadata.is_dir(),
            EntryType::File => metadata.is_file(),
            EntryType::Link => metadata.is_symlink(),
        };

        entries_type.iter().copied().any(match_type)
    }
}

pub fn get_matches(
    path: &Path,
    entries_type: &[EntryType],
    regexes: &[Regex],
) -> Vec<Result<PathBuf, IoErrorWithPath>> {
    let matadata = match std::fs::metadata(path) {
        Ok(metada) => metada,
        Err(error) => {
            return vec![Err(IoErrorWithPath {
                path: Some(path.to_path_buf()),
                error,
            })]
        }
    };

    // early return for files
    if matadata.is_file() {
        if filter(path, &matadata, entries_type, regexes) {
            return vec![Ok(path.to_path_buf())];
        } else {
            return Vec::new();
        }
    }

    let mut result = Vec::new();
    let walker = WalkDir::new(path).contents_first(true).into_iter();

    for entry in walker {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                result.push(Err(IoErrorWithPath {
                    path: Some(path.to_path_buf()),
                    error: error.into(),
                }));
                continue;
            }
        };

        let path = entry.path();
        let metadata = match entry.metadata() {
            Ok(metadata) => metadata,
            Err(error) => {
                result.push(Err(IoErrorWithPath {
                    path: Some(path.to_path_buf()),
                    error: error.into(),
                }));
                continue;
            }
        };

        if filter(path, &metadata, entries_type, regexes) {
            result.push(Ok(path.to_path_buf()));
        }
    }

    result
}
