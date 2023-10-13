use std::{error::Error, fmt::format, ops::Range, path::PathBuf};

use clap::{App, Arg};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TakeValue {
    PlusZero,
    TakeNum(i64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DisplayMode {
    Bytes,
    Lines,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Config {
    pub files: Vec<PathBuf>,
    pub take_value: TakeValue,
    pub display_mode: DisplayMode,
    pub no_header: bool,
}

fn parse_take_value(str: &str) -> Result<TakeValue, Box<dyn Error>> {
    if str == "+0" {
        return Ok(TakeValue::PlusZero);
    }

    if str.as_bytes().first() == Some(&b'+') {
        Ok(TakeValue::TakeNum(str[1..].parse::<i64>()?))
    } else {
        Ok(TakeValue::TakeNum(-str.parse::<i64>()?))
    }
}

impl Config {
    pub fn get() -> Result<Self, Box<dyn Error>> {
        let matches = App::new("grepr")
            .version("0.1.0")
            .author("66011245@kmitl.ac.th")
            .about("A rewrite in rust of `grep1")
            .arg(
                Arg::with_name("files")
                    .value_name("FILES")
                    .takes_value(true)
                    .required(true)
                    .multiple(true)
                    .help("Input file(s)"),
            )
            .arg(
                Arg::with_name("quiet")
                    .short("q")
                    .long("quiet")
                    .help("Supress headers"),
            )
            .arg(
                Arg::with_name("bytes")
                    .value_name("BYTES")
                    .short("c")
                    .long("bytes")
                    .takes_value(true)
                    .conflicts_with("lines")
                    .help("Number of bytes"),
            )
            .arg(
                Arg::with_name("lines")
                    .value_name("LINES")
                    .short("n")
                    .long("lines")
                    .takes_value(true)
                    .help("Number of lines"),
            )
            .get_matches();

        let (display_mode, take_value) =
            match (matches.value_of("bytes"), matches.value_of("lines")) {
                (None, None) => (DisplayMode::Lines, TakeValue::TakeNum(-10)),
                (None, Some(str)) => (
                    DisplayMode::Lines,
                    parse_take_value(str).map_err(|_| format!("illegal line count -- {str}"))?,
                ),
                (Some(str), None) => (
                    DisplayMode::Bytes,
                    parse_take_value(str).map_err(|_| format!("illegal byte count -- {str}"))?,
                ),
                (Some(_), Some(_)) => unreachable!("clap should prevent this"),
            };

        Ok(Self {
            files: matches
                .values_of("files")
                .expect("it was a required argument")
                .map(PathBuf::from)
                .collect(),
            display_mode,
            take_value,
            no_header: matches.is_present("quiet"),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Metadata {
    line_count: usize,
    byte_count: usize,
}

impl Metadata {
    fn get(str: &str) -> Self {
        Self {
            line_count: str.lines().count(),
            byte_count: str.len(),
        }
    }
}

fn parse_take_value_to_range(take_value: TakeValue, upper_bound: usize) -> Range<usize> {
    match take_value {
        TakeValue::PlusZero => 0..upper_bound,
        TakeValue::TakeNum(positive) if positive >= 0 => {
            ((positive as usize).min(upper_bound))..upper_bound
        }
        TakeValue::TakeNum(negative) => {
            (upper_bound.saturating_add_signed(negative as isize))..upper_bound
        }
    }
}

fn get_bytes(str: &str, take_value: TakeValue) -> String {
    let range = parse_take_value_to_range(take_value, str.len());
    String::from_utf8_lossy(&str.as_bytes()[range]).to_string()
}

fn get_line_byte_positions(text: &str) -> Vec<Range<usize>> {
    let mut current_position = 0;
    let mut results = Vec::new();

    let mut skip = false;

    for (byte, char) in text.char_indices() {
        if skip {
            skip = false;
            continue;
        }

        // ordinary lf
        if char == '\n' {
            #[allow(clippy::range_plus_one)]
            results.push(current_position..byte + 1);

            current_position = byte + 1;
        }

        // crlf
        if char == '\r' {
            if text.as_bytes().get(byte + 1) == Some(&b'\n') {
                #[allow(clippy::range_plus_one)]
                results.push(current_position..byte + 2);

                current_position = byte + 2;

                skip = true;
            } else {
                #[allow(clippy::range_plus_one)]
                results.push(current_position..byte + 1);

                current_position = byte + 1;
            }
        }
    }

    if current_position != text.len() {
        results.push(current_position..text.len());
    }

    results
}

fn get_lines(str: &str, take_value: TakeValue) -> String {
    let line_ranges = get_line_byte_positions(str);
    let range = parse_take_value_to_range(take_value, line_ranges.len());
    dbg!(&line_ranges, &take_value, &range);
    range
        .map(|i| &str[line_ranges[i].clone()])
        .collect::<Vec<&str>>()
        .join("")
}

pub fn run(config: Config) {
    let display_header = config.files.len() > 1 && !config.no_header;

    for file in config.files {
        let string = match std::fs::read_to_string(&file) {
            Ok(ok) => ok,
            Err(err) => {
                eprintln!("{}: {err}", file.display());
                continue;
            }
        };

        if display_header {
            println!("==> {} <==", file.display());
        }

        match config.display_mode {
            DisplayMode::Bytes => print!("{}", get_bytes(&string, config.take_value)),
            DisplayMode::Lines => print!("{}", get_lines(&string, config.take_value)),
        }

        if display_header {
            println!();
        }
    }
}
