use std::{error::Error, io::BufRead, path::PathBuf};

use clap::{App, Arg};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Argument {
    pub input: Option<PathBuf>,
    pub output: Option<PathBuf>,
    pub count: bool,
}

impl Argument {
    pub fn get() -> Argument {
        let matches = App::new("uniqr")
            .version("0.1.0")
            .about("The rust cloned of `uniq`")
            .author("66011245@kmitl.ac.th")
            .arg(
                Arg::with_name("in_file")
                    .help("Input file")
                    .default_value("-")
                    .value_name("IN_FILE"),
            )
            .arg(
                Arg::with_name("out_file")
                    .help("Output file")
                    .value_name("OUT_FILE"),
            )
            .arg(
                Arg::with_name("count")
                    .help("Show counts")
                    .short("c")
                    .long("count"),
            )
            .get_matches();

        let input = match matches
            .value_of("in_file")
            .expect("should've had a default value")
        {
            "-" => None,
            path => Some(PathBuf::from(path)),
        };
        let output = matches.value_of("out_file").map(PathBuf::from);
        let count = matches.is_present("count");

        Argument {
            input,
            output,
            count,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Line {
    pub value: String,
    pub count: usize,
}

impl Line {
    pub fn from_reader(mut reader: impl BufRead) -> Result<Vec<Line>, Box<dyn Error>> {
        let mut lines = Vec::<Line>::new();
        let mut string = String::new();
        reader.read_to_string(&mut string)?;

        // asuume we use LF as the line break
        for line in string.split('\n') {
            if let Some(previous_line) = lines.last_mut() {
                if previous_line.value == line {
                    previous_line.count += 1;
                    continue;
                }
            }

            lines.push(Line {
                value: line.to_string(),
                count: 1,
            });
        }

        Ok(lines)
    }
}
