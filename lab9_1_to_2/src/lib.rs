use std::{
    error::Error,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use clap::{App, Arg};
use regex::{Regex, RegexBuilder};
use walkdir::WalkDir;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Input {
    Stdin,
    Files(Vec<PathBuf>),
}

#[derive(Debug, Clone)]
pub struct Config {
    pub input: Input,
    pub recursive: bool,
    pub pattern: Regex,
    pub count: bool,
    pub invert_match: bool,
}

impl Config {
    pub fn from_args() -> Result<Self, Box<dyn Error>> {
        let matches = App::new("grepr")
            .version("0.1.0")
            .author("66011245@kmitl.ac.th")
            .about("A rewrite in rust of `grep1")
            .arg(
                Arg::with_name("pattern")
                    .value_name("PATTERN")
                    .takes_value(true)
                    .required(true)
                    .help("Search pattern"),
            )
            .arg(
                Arg::with_name("file")
                    .value_name("FILE")
                    .takes_value(true)
                    .multiple(true)
                    .default_value("-")
                    .help("Input file(s)"),
            )
            .arg(
                Arg::with_name("count")
                    .short("c")
                    .long("count")
                    .help("Count occurrences"),
            )
            .arg(
                Arg::with_name("insensitive")
                    .short("i")
                    .long("insensitive")
                    .help("Case-insensitive"),
            )
            .arg(
                Arg::with_name("invert-match")
                    .short("v")
                    .long("invert-match")
                    .help("Invert match"),
            )
            .arg(
                Arg::with_name("recursive")
                    .short("r")
                    .long("recursive")
                    .help("Recusive search"),
            )
            .get_matches();

        let pattern = matches.value_of("pattern").expect("it's required argument");
        let regex = RegexBuilder::new(pattern)
            .case_insensitive(matches.is_present("insensitive"))
            .build()
            .map_err(|_| format!("Invalid pattern \"{pattern}\""))?;

        let files = matches
            .values_of_lossy("file")
            .expect("it's required argument");

        let input = if let (Some("-"), 1) = (files.get(0).map(AsRef::as_ref), files.len()) {
            Input::Stdin
        } else {
            Input::Files(files.into_iter().map(PathBuf::from).collect())
        };

        Ok(Self {
            input,
            recursive: matches.is_present("recursive"),
            pattern: regex,
            count: matches.is_present("count"),
            invert_match: matches.is_present("invert-match"),
        })
    }
}

fn find_files<'a>(
    paths: impl Iterator<Item = &'a Path>,
    recursive: bool,
) -> Vec<Result<PathBuf, Box<dyn Error>>> {
    let mut result = Vec::new();

    for path in paths {
        let metadata = match std::fs::metadata(path) {
            Ok(ok) => ok,
            Err(err) => {
                result.push(Err(format!("{}: {err}", path.display()).into()));
                continue;
            }
        };

        match (metadata.is_dir(), recursive) {
            (true, false) => result.push(Err(format!("{} is a directory", path.display()).into())),
            (false, _) => result.push(Ok(path.into())),
            (true, true) => {
                let walker = WalkDir::new(path).into_iter();

                for entry in walker.into_iter() {
                    let entry = match entry {
                        Ok(entry) => entry,
                        Err(err) => {
                            result.push(Err(match err.path() {
                                Some(path) => format!("{}: {err}", path.display()).into(),
                                None => format!("{err}").into(),
                            }));
                            continue;
                        }
                    };

                    let metadata = match entry.metadata() {
                        Ok(ok) => ok,
                        Err(err) => {
                            result.push(Err(match err.path() {
                                Some(path) => format!("{}: {err}", path.display()).into(),
                                None => format!("{err}").into(),
                            }));
                            continue;
                        }
                    };

                    if metadata.is_file() {
                        result.push(Ok(entry.into_path()));
                    }
                }
            }
        }
    }

    result
}

fn find_lines(
    file: impl BufRead,
    pattern: &Regex,
    invert_match: bool,
) -> Result<Vec<String>, Box<dyn Error>> {
    let mut matches = Vec::new();

    for line in file.lines() {
        let line = line?;
        let line_match = pattern.is_match(&line);

        if line_match != invert_match {
            matches.push(line);
        }
    }

    Ok(matches)
}

pub fn run(config: Config) {
    match config.input {
        Input::Stdin => {
            let lines = find_lines(
                BufReader::new(std::io::stdin()),
                &config.pattern,
                config.invert_match,
            );

            match lines {
                Ok(lines) => {
                    if config.count {
                        println!("{}", lines.len());
                    } else {
                        for line in lines {
                            println!("{}", line);
                        }
                    }
                }
                Err(err) => eprintln!("{err}"),
            }
        }
        Input::Files(files) => {
            let print_header = files.len() != 1 || config.recursive;
            let entries = find_files(files.iter().map(|x| x.as_path()), config.recursive);

            for entry in entries {
                let entry = match entry {
                    Ok(ok) => ok,
                    Err(err) => {
                        eprintln!("{err}");
                        continue;
                    }
                };

                let file = match std::fs::File::open(&entry) {
                    Ok(file) => file,
                    Err(err) => {
                        eprintln!("{}: {err}", entry.display());
                        continue;
                    }
                };

                let lines = find_lines(BufReader::new(file), &config.pattern, config.invert_match);

                match lines {
                    Ok(lines) => {
                        if config.count {
                            if print_header {
                                print!("{}:", entry.display());
                            }
                            println!("{}", lines.len());
                        } else {
                            for line in lines {
                                if print_header {
                                    print!("{}:", entry.display());
                                }

                                println!("{}", line);
                            }
                        }
                    }
                    Err(err) => eprintln!("{}: {err}", entry.display()),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        io::Cursor,
        path::{Path, PathBuf},
    };

    use super::find_files;
    use rand::{distributions::Alphanumeric, Rng};
    use regex::{Regex, RegexBuilder};

    #[test]
    fn test_find_files() {
        // Verify that the function finds a file known to exist
        let files = super::find_files([Path::new("./tests/inputs/fox.txt")].into_iter(), false);
        assert_eq!(files.len(), 1);
        assert_eq!(
            files[0].as_ref().unwrap(),
            Path::new("./tests/inputs/fox.txt")
        );
        // The function should reject a directory
        // without the recursive option
        let files = find_files([Path::new("./tests/inputs")].into_iter(), false);
        assert_eq!(files.len(), 1);
        if let Err(e) = &files[0] {
            assert_eq!(e.to_string(), "./tests/inputs is a directory");
        }
        // Verify the function recurses to find four files in the directory
        let res = find_files([Path::new("./tests/inputs")].into_iter(), true);
        let mut files: Vec<String> = res
            .iter()
            .map(|r| r.as_ref().unwrap().to_str().unwrap().replace('\\', "/"))
            .collect();
        files.sort();
        assert_eq!(files.len(), 4);
        assert_eq!(
            files,
            vec![
                "./tests/inputs/bustle.txt",
                "./tests/inputs/empty.txt",
                "./tests/inputs/fox.txt",
                "./tests/inputs/nobody.txt",
            ]
        );
        // Generate a random string to represent a nonexistent file
        let bad: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();
        // Verify that the function returns the bad file as an error
        let files = find_files([PathBuf::from(bad)].iter().map(|x| x.as_path()), false);
        assert_eq!(files.len(), 1);
        assert!(files[0].is_err());
    }

    #[test]
    fn test_find_lines() {
        let text = b"Lorem\nIpsum\r\nDOLOR";
        // The pattern _or_ should match the one line, "Lorem"
        let re1 = Regex::new("or").unwrap();
        let matches = super::find_lines(Cursor::new(&text), &re1, false);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 1);
        // When inverted, the function should match the other two lines
        let matches = super::find_lines(Cursor::new(&text), &re1, true);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 2);
        // This regex will be case-insensitive
        let re2 = RegexBuilder::new("or")
            .case_insensitive(true)
            .build()
            .unwrap();
        // The two lines "Lorem" and "DOLOR" should match
        let matches = super::find_lines(Cursor::new(&text), &re2, false);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 2);
        // When inverted, the one remaining line should match
        let matches = super::find_lines(Cursor::new(&text), &re2, true);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 1);
    }
}
