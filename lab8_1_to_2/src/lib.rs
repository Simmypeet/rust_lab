use std::{
    error::Error,
    io::{BufRead, BufReader},
    ops::Range,
    path::PathBuf,
};

use clap::{App, Arg};
use csv::{ReaderBuilder, StringRecord};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ExtractKind {
    Fields,
    Bytes,
    Chars,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Extract {
    kind: ExtractKind,
    ranges: Vec<Range<usize>>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Input {
    Files(Vec<PathBuf>),
    Stdin,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Config {
    input: Input,
    delimiter: u8,
    extract: Extract,
}

fn parse_range(range_str: &str) -> Result<Vec<Range<usize>>, Box<dyn Error>> {
    let ranges = range_str.split(',');
    let mut result = Vec::new();

    let illegal_list_value_err = || format!("illegal list value: \"{range_str}\"");

    for range in ranges {
        // parse range will bounds
        if range.contains('-') {
            let mut bounds = range.split('-');

            // must not contain '+'
            let lower_bound = bounds
                .next()
                .map(|x| -> Result<&str, Box<dyn Error>> {
                    if x.contains('+') {
                        Err(illegal_list_value_err().into())
                    } else {
                        Ok(x)
                    }
                })
                .transpose()?
                .and_then(|x| x.parse::<usize>().ok())
                .ok_or_else(illegal_list_value_err)?;
            let upper_bound = bounds
                .next()
                .map(|x| -> Result<&str, Box<dyn Error>> {
                    if x.contains('+') {
                        Err(illegal_list_value_err().into())
                    } else {
                        Ok(x)
                    }
                })
                .transpose()?
                .and_then(|x| x.parse::<usize>().ok())
                .ok_or_else(illegal_list_value_err)?;

            if bounds.next().is_some() {
                return Err(illegal_list_value_err().into());
            }

            // lower bound and upper bound must not be zero.
            if lower_bound == 0 {
                return Err(format!("illegal list value: \"{lower_bound}\"").into());
            }
            if upper_bound == 0 {
                return Err(format!("illegal list value: \"{upper_bound}\"").into());
            }

            if lower_bound > upper_bound {
                return Err(format!("First number in range ({lower_bound}) must be lower than second number ({upper_bound})").into());
            }

            result.push(lower_bound - 1..upper_bound);
        } else {
            // check if range contains '+'
            if range.contains('+') {
                return Err(illegal_list_value_err().into());
            }

            // parse unit range
            let number = range
                .parse::<usize>()
                .map_err(|_| illegal_list_value_err())?;

            // the number can't be zero
            if number == 0 {
                return Err(illegal_list_value_err().into());
            }

            result.push(number - 1..number);
        }
    }

    Ok(result)
}

impl Config {
    pub fn get_from_args() -> Result<Self, Box<dyn Error>> {
        let matches = App::new("cutr")
            .version("0.1.0")
            .author("66011245@kmitl.ac.th")
            .about("Rust clone of `cut`")
            .arg(
                Arg::with_name("file")
                    .value_name("FILE")
                    .multiple(true)
                    .takes_value(true)
                    .help("Input file(s)")
                    .default_value("-"),
            )
            .arg(
                Arg::with_name("bytes")
                    .value_name("BYTES")
                    .long("bytes")
                    .short("b")
                    .takes_value(true)
                    .help("Selected bytes")
                    .conflicts_with("chars")
                    .conflicts_with("fields"),
            )
            .arg(
                Arg::with_name("chars")
                    .value_name("CHARS")
                    .long("chars")
                    .short("c")
                    .takes_value(true)
                    .help("Selected characters")
                    .conflicts_with("bytes")
                    .conflicts_with("fields"),
            )
            .arg(
                Arg::with_name("fields")
                    .value_name("FIELDS")
                    .long("fields")
                    .short("f")
                    .takes_value(true)
                    .help("Selected fields")
                    .conflicts_with("bytes")
                    .conflicts_with("chars"),
            )
            .arg(
                Arg::with_name("delimiter")
                    .value_name("DELIMITER")
                    .help("Field delimiter")
                    .long("delim")
                    .short("d")
                    .takes_value(true)
                    .default_value("\t"),
            )
            .get_matches();

        let extract = match (
            matches.value_of("bytes"),
            matches.value_of("chars"),
            matches.value_of("fields"),
        ) {
            // bytes extract
            (Some(bytes), None, None) => Extract {
                kind: ExtractKind::Bytes,
                ranges: parse_range(bytes)?,
            },
            // chars extract
            (None, Some(chars), None) => Extract {
                kind: ExtractKind::Chars,
                ranges: parse_range(chars)?,
            },
            // fields extract
            (None, None, Some(chars)) => Extract {
                kind: ExtractKind::Fields,
                ranges: parse_range(chars)?,
            },
            // no extract
            (None, None, None) => {
                return Err("Must have --fields, --bytes, or --chars".to_string().into())
            }
            _ => unreachable!(),
        };

        let delimiter = {
            let value = matches
                .value_of("delimiter")
                .expect("should've a value, atleast default");

            if value.as_bytes().len() != 1 {
                return Err(format!("--delim \"{value}\" must be a single byte").into());
            }

            value.as_bytes()[0]
        };

        let files: Vec<PathBuf> = matches
            .values_of("file")
            .expect("should be at least one value")
            .map(PathBuf::from)
            .collect();

        // for input with ["-"]
        let input = if files.get(0).and_then(|x| x.to_str()) == Some("-") && files.len() == 1 {
            Input::Stdin
        } else {
            Input::Files(files)
        };

        Ok(Config {
            input,
            delimiter,
            extract,
        })
    }
}

fn extract_chars(line: &str, char_ranges: &[Range<usize>]) -> String {
    let all_chars = line.chars().collect::<Vec<_>>();

    let mut result = String::new();

    for char_range in char_ranges {
        // if char_range is not in the range of all_chars, skip it
        if char_range.end > all_chars.len() {
            continue;
        }

        // get the chars from all_chars
        for i in char_range.clone() {
            result.push(all_chars[i]);
        }
    }

    result
}

fn extract_bytes(line: &str, byte_ranges: &[Range<usize>]) -> String {
    let mut bytes = Vec::new();

    for byte_range in byte_ranges {
        // if char_range is not in the range of all_chars, skip it
        if byte_range.end > line.as_bytes().len() {
            continue;
        }

        // get the chars from all_chars
        for i in byte_range.clone() {
            bytes.push(line.as_bytes()[i]);
        }
    }

    // iteratively transforms the byte into a utf-8 character
    String::from_utf8_lossy(&bytes).to_string()
}

fn extract_fields(record: &StringRecord, field_ranges: &[Range<usize>]) -> Vec<String> {
    let mut result = Vec::new();

    for field_range in field_ranges {
        // if field_range is not in the range of record, skip it
        if field_range.end > record.len() {
            continue;
        }

        // get the fields from record

        for i in field_range.clone() {
            result.push(record[i].to_string());
        }
    }

    result
}

pub fn run(config: Config) {
    let mut readers = Vec::new();
    match config.input {
        Input::Files(files) => {
            for file in files {
                match std::fs::File::open(&file) {
                    Ok(file) => readers.push(Box::new(BufReader::new(file)) as Box<dyn BufRead>),
                    Err(error) => eprintln!("{}: {error}", file.display()),
                }
            }
        }
        Input::Stdin => {
            readers.push(Box::new(BufReader::new(std::io::stdin())) as Box<dyn BufRead>)
        }
    }

    let delimiter_str = {
        let mut string = String::new();
        string.push(config.delimiter as char);
        string
    };

    for reader in readers {
        match config.extract.kind {
            // use csv crate to parse delimited file
            ExtractKind::Fields => {
                let mut reader = ReaderBuilder::new()
                    .delimiter(config.delimiter)
                    .from_reader(reader);

                let headers = match reader.headers() {
                    Ok(headers) => headers,
                    Err(err) => {
                        eprintln!("{err}");
                        continue;
                    }
                };

                println!(
                    "{}",
                    extract_fields(headers, &config.extract.ranges).join(&delimiter_str)
                );

                let records = reader.records();

                for record in records {
                    let record = match record {
                        Ok(record) => record,
                        Err(err) => {
                            eprintln!("{err}");
                            continue;
                        }
                    };

                    println!(
                        "{}",
                        extract_fields(&record, &config.extract.ranges).join(&delimiter_str)
                    );
                }
            }

            ExtractKind::Bytes | ExtractKind::Chars => {
                for line in reader.lines() {
                    let line = match line {
                        Ok(ok) => ok,
                        Err(err) => {
                            eprintln!("{err}");
                            continue;
                        }
                    };

                    let output = match config.extract.kind {
                        ExtractKind::Bytes => extract_bytes(&line, &config.extract.ranges),
                        ExtractKind::Chars => extract_chars(&line, &config.extract.ranges),
                        _ => unreachable!(),
                    };

                    println!("{output}")
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use csv::StringRecord;

    #[test]
    fn test_parse_range() {
        // The empty string is an error assert!(parse_pos("").is_err());
        // Zero is an error
        assert!(super::parse_range("0").is_err());

        let res = super::parse_range("0");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"0\"");
        let res = super::parse_range("0-1");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"0\"");
        // A leading "+" is an error
        let res = super::parse_range("+1");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"+1\"");
        let res = super::parse_range("+1-2");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"+1-2\"");

        let res = super::parse_range("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1]);
        let res = super::parse_range("01");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1]);
        let res = super::parse_range("1,3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 2..3]);
        let res = super::parse_range("001,0003");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 2..3]);
        let res = super::parse_range("1-3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..3]);
        let res = super::parse_range("0001-03");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..3]);
        let res = super::parse_range("1,7,3-5");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 6..7, 2..5]);
        let res = super::parse_range("15,19-20");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![14..15, 18..20]);
    }

    // tecniquely, the character á is not considered as a single utf-8 character. There seems to be
    // a character 'a' ascii and some utf-8 character that combines with the previous character to
    // form a new character. So, the character á is considered as two characters.

    #[test]
    fn test_extract_chars() {
        assert_eq!(super::extract_chars("", &[0..1]), "".to_string());
        assert_eq!(super::extract_chars("Ébc", &[0..1]), "É".to_string());
        assert_eq!(super::extract_chars("Ébc", &[0..1, 2..3]), "Éc".to_string());
        assert_eq!(super::extract_chars("Ébc", &[0..3]), "Ébc".to_string());
        assert_eq!(super::extract_chars("Ébc", &[2..3, 1..2]), "cb".to_string());
        assert_eq!(
            super::extract_chars("Ébc", &[0..1, 1..2, 4..5]),
            "Éb".to_string()
        );
    }

    #[test]
    fn test_extract_bytes() {
        assert_eq!(super::extract_bytes("Ébc", &[0..1]), "�".to_string());
        assert_eq!(super::extract_bytes("Ébc", &[0..2]), "É".to_string());
        assert_eq!(super::extract_bytes("Ébc", &[0..3]), "Éb".to_string());
        assert_eq!(super::extract_bytes("Ébc", &[0..4]), "Ébc".to_string());
        assert_eq!(super::extract_bytes("Ébc", &[3..4, 2..3]), "cb".to_string());
        assert_eq!(super::extract_bytes("Ébc", &[0..2, 5..6]), "É".to_string());
    }

    #[test]
    fn test_extract_fields() {
        let rec = StringRecord::from(vec!["Captain", "Sham", "12345"]);

        assert_eq!(super::extract_fields(&rec, &[0..1]), &["Captain"]);
        assert_eq!(super::extract_fields(&rec, &[1..2]), &["Sham"]);
        assert_eq!(
            super::extract_fields(&rec, &[0..1, 2..3]),
            &["Captain", "12345"]
        );
        assert_eq!(super::extract_fields(&rec, &[0..1, 3..4]), &["Captain"]);
        assert_eq!(
            super::extract_fields(&rec, &[1..2, 0..1]),
            &["Sham", "Captain"]
        );
    }
}

