use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use clap::{App, Arg};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Input {
    BothFile(PathBuf, PathBuf),
    FirstStdin(PathBuf),
    SecondStdin(PathBuf),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Config {
    pub input: Input,
    pub show_col1: bool,
    pub show_col2: bool,
    pub show_col3: bool,
    pub insensitive: bool,
    pub delimiter: String,
}

impl Config {
    pub fn from_args() -> Result<Self, Box<dyn std::error::Error>> {
        let matches = App::new("commr")
            .version("0.1.0")
            .author("66011245@kmitl.ac.th")
            .about("A rewrite in rust of `comm`")
            .arg(
                Arg::with_name("file1")
                    .value_name("FILE1")
                    .takes_value(true)
                    .required(true)
                    .help("Input file 1"),
            )
            .arg(
                Arg::with_name("file2")
                    .value_name("FILE2")
                    .takes_value(true)
                    .required(true)
                    .help("Input file 2"),
            )
            .arg(
                Arg::with_name("1")
                    .short("1")
                    .help("Supress printing of column 1"),
            )
            .arg(
                Arg::with_name("2")
                    .short("2")
                    .help("Supress printing of column 2"),
            )
            .arg(
                Arg::with_name("3")
                    .short("3")
                    .help("Supress printing of column 3"),
            )
            .arg(
                Arg::with_name("case-insensitive")
                    .short("i")
                    .help("Case-insensitive comparison of lines"),
            )
            .arg(
                Arg::with_name("output-delimiter")
                    .short("d")
                    .long("output-delimiter")
                    .value_name("DELIM")
                    .help("Output delimiter")
                    .default_value("\t"),
            )
            .get_matches();

        Ok(Config {
            input: match (
                matches.value_of("file1").expect("it's required argument"),
                matches.value_of("file2").expect("it's required argument"),
            ) {
                ("-", "-") => return Err("Both input files cannot be STDIN (\"-\")".into()),
                ("-", file2) => Input::FirstStdin(PathBuf::from(file2)),
                (file1, "-") => Input::SecondStdin(PathBuf::from(file1)),
                (file1, file2) => Input::BothFile(PathBuf::from(file1), PathBuf::from(file2)),
            },
            show_col1: !matches.is_present("1"),
            show_col2: !matches.is_present("2"),
            show_col3: !matches.is_present("3"),
            insensitive: matches.is_present("case-insensitive"),
            delimiter: matches
                .value_of("output-delimiter")
                .expect("should at least has tab as a default value")
                .to_string(),
        })
    }
}

fn open<P: AsRef<Path>>(input: Option<P>) -> Result<Box<dyn BufRead>, Box<dyn Error>> {
    match input {
        Some(file_path) => {
            let file = File::open(&file_path)
                .map_err(|err| format!("{}: {err}", file_path.as_ref().display()))?;

            Ok(Box::new(BufReader::new(file)))
        }
        None => Ok(Box::new(BufReader::new(std::io::stdin()))),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Property {
    FirstExclusive,
    SecondExclusive,
    Mutual,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let (first_file, second_file) = match config.input {
        Input::BothFile(first_file, second_file) => {
            (open(Some(first_file))?, open(Some(second_file))?)
        }
        Input::FirstStdin(second_file) => {
            (open(Option::<PathBuf>::None)?, open(Some(second_file))?)
        }
        Input::SecondStdin(first_file) => (open(Some(first_file))?, open(Option::<PathBuf>::None)?),
    };

    let first_lines = first_file
        .lines()
        .map(|x| {
            x.map(|mut x| {
                if config.insensitive {
                    x.make_ascii_lowercase();
                    x
                } else {
                    x
                }
            })
        })
        .collect::<Result<HashSet<_>, _>>()?;
    let seoncd_lines = second_file
        .lines()
        .map(|x| {
            x.map(|mut x| {
                if config.insensitive {
                    x.make_ascii_lowercase();
                    x
                } else {
                    x
                }
            })
        })
        .collect::<Result<HashSet<_>, _>>()?;

    let first_exclusive = first_lines
        .difference(&seoncd_lines)
        .map(|x| (x, Property::FirstExclusive));
    let second_exclusive = seoncd_lines
        .difference(&first_lines)
        .map(|x| (x, Property::SecondExclusive));
    let mutual = first_lines
        .intersection(&seoncd_lines)
        .map(|x| (x, Property::Mutual));
    let mut representation = first_exclusive
        .chain(second_exclusive)
        .chain(mutual)
        .collect::<Vec<_>>();
    representation.sort_by(|(a, _), (b, _)| a.cmp(b));

    for (line, prop) in representation {
        match prop {
            Property::FirstExclusive if config.show_col1 => {
                println!("{line}")
            }
            Property::SecondExclusive if config.show_col2 => {
                println!(
                    "{}{line}",
                    if config.show_col1 {
                        &config.delimiter
                    } else {
                        ""
                    }
                )
            }
            Property::Mutual if config.show_col3 => {
                println!(
                    "{}{}{line}",
                    if config.show_col1 {
                        &config.delimiter
                    } else {
                        ""
                    },
                    if config.show_col2 {
                        &config.delimiter
                    } else {
                        ""
                    }
                )
            }
            _ => {}
        }
    }

    Ok(())
}
