use std::{error::Error, path::PathBuf};

use clap::{App, Arg};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Output {
    Line(usize),
    Byte(usize),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Input {
    Files(Vec<PathBuf>),
    Stdin,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Argument {
    pub input: Input,
    pub output: Output,
}

impl Argument {
    fn parse_number(number_string: &str, argument_name: &str) -> Result<usize, Box<dyn Error>> {
        match number_string.parse() {
            Ok(0) | Err(_) => {
                Err(format!("head: illegal {} count -- {}", argument_name, number_string).into())
            }
            Ok(number) => Ok(number),
        }
    }

    pub fn from_env_args() -> Result<Self, Box<dyn Error>> {
        let matches = App::new("headr")
            .version("0.1.0")
            .author("SE15 <66011245@kmitl.ac.th>")
            .about("A `head` clone written in Rust")
            .arg(
                Arg::with_name("lines")
                    .short("n")
                    .long("lines")
                    .value_name("LINES")
                    .help("Number of lines [default: 10]"),
            )
            .arg(
                Arg::with_name("bytes")
                    .short("c")
                    .long("bytes")
                    .value_name("BYTES")
                    .help("Number of bytes"),
            )
            .arg(
                Arg::with_name("files")
                    .value_name("FILES")
                    .help("Input file(s)")
                    .multiple(true)
                    .default_value("-"),
            )
            .get_matches();

        let output = match (matches.value_of("lines"), matches.value_of("bytes")) {
            (None, None) => Output::Line(10),
            (Some(lines), None) => Output::Line(Self::parse_number(lines, "line")?),
            (None, Some(bytes)) => Output::Byte(Self::parse_number(bytes, "byte")?),
            (Some(_), Some(_)) => {
                return Err(
                    "The argument '--lines <LINES>' cannot be used with '--bytes <BYTES>'".into(),
                )
            }
        };

        let files = matches
            .values_of("files")
            .expect("should exist")
            .collect::<Vec<_>>();

        let input = if files.first().map_or(false, |x| *x == "-") && files.len() == 1 {
            Input::Stdin
        } else {
            Input::Files(files.iter().map(PathBuf::from).collect())
        };

        Ok(Self { input, output })
    }
}

pub fn print_output(mut reader: impl std::io::BufRead, output: Output) -> std::io::Result<()> {
    let string = {
        let mut string = String::new();
        reader.read_to_string(&mut string)?;
        string
    };
    match output {
        Output::Line(line_number) => {
            let mut encountered_line_number = 0;
            let chars: Vec<char> = string.chars().collect();
            let mut index = 0;

            while index < chars.len() && encountered_line_number < line_number {
                match (chars.get(index).copied(), chars.get(index + 1).copied()) {
                    // handle CLRF line ending
                    (Some('\r'), Some('\n')) => {
                        encountered_line_number += 1;
                        // skip the extra '\n'
                        index += 1;
                        print!("\r\n");
                    }
                    // handle LF line ending
                    (Some('\n'), _) => {
                        encountered_line_number += 1;
                        println!()
                    }

                    (Some(char), _) => {
                        print!("{char}");
                    }

                    (_, _) => unreachable!(),
                }

                index += 1;
            }
        }
        Output::Byte(byte_numbers) => {
            let upper_bound = std::cmp::min(string.len(), byte_numbers);
            let string = String::from_utf8_lossy(&string.as_bytes()[..upper_bound]);
            print!("{string}");
        }
    }

    Ok(())
}
