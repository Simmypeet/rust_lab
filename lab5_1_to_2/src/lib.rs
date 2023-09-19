use std::{error::Error, iter::Peekable, path::PathBuf, str::Chars};

use clap::{App, Arg};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Input {
    Files(Vec<PathBuf>),
    Stdin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CharacterCounting {
    ByChars,
    ByBytes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CountingConfig {
    pub count_line: bool,
    pub count_word: bool,
    pub character_counting: Option<CharacterCounting>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Argument {
    pub input: Input,
    pub counting_config: CountingConfig,
}

impl Argument {
    pub fn from_env_args() -> Result<Self, Box<dyn Error>> {
        let matches = App::new("wcr")
            .version("0.1.0")
            .about("The rust cloned of `wc`")
            .author("66011245@kmitl.ac.th")
            .arg(
                Arg::with_name("file")
                    .help("Input file(s) [default: -]")
                    .multiple(true)
                    .value_name("FILE"),
            )
            .arg(
                Arg::with_name("bytes")
                    .help("Show byte count")
                    .short("c")
                    .long("bytes"),
            )
            .arg(
                Arg::with_name("chars")
                    .help("Show character count")
                    .short("m")
                    .long("chars"),
            )
            .arg(
                Arg::with_name("lines")
                    .help("Show line count")
                    .short("l")
                    .long("lines"),
            )
            .arg(
                Arg::with_name("words")
                    .help("Show word count")
                    .short("w")
                    .long("words"),
            )
            .get_matches();

        let no_positional_arguments = !(matches.is_present("words")
            || matches.is_present("lines")
            || matches.is_present("chars")
            || matches.is_present("bytes"));

        Ok(Argument {
            input: matches.values_of_lossy("file").map_or_else(
                || Input::Stdin,
                |vec| {
                    if vec.get(0).map_or(false, |x| x == "-") && vec.len() == 1 {
                        Input::Stdin
                    } else {
                        Input::Files(vec.into_iter().map(PathBuf::from).collect())
                    }
                },
            ),
            counting_config: CountingConfig {
                count_line: if no_positional_arguments {
                    true
                } else {
                    matches.is_present("lines")
                },
                count_word: if no_positional_arguments {
                    true
                } else {
                    matches.is_present("words")
                },
                character_counting: if no_positional_arguments {
                    Some(CharacterCounting::ByBytes)
                } else {
                    match (matches.is_present("chars"), matches.is_present("bytes")) {
                        (true, true) => {
                            return Err(
                                "error: The argument '--bytes' cannot be used with '--chars'"
                                    .into(),
                            );
                        }
                        (true, false) => Some(CharacterCounting::ByChars),
                        (false, true) => Some(CharacterCounting::ByBytes),
                        (false, false) => None,
                    }
                },
            },
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Counting {
    pub line_count: Option<usize>,
    pub word_count: Option<usize>,
    pub character_or_byte_count: Option<usize>,
}

impl Counting {
    pub fn count_from_str(str: &str, counting_config: CountingConfig) -> Self {
        let mut counting = Counting {
            line_count: if counting_config.count_line {
                Some(0)
            } else {
                None
            },
            word_count: if counting_config.count_word {
                Some(0)
            } else {
                None
            },
            character_or_byte_count: counting_config.character_counting.map(|x| match x {
                CharacterCounting::ByChars => str.chars().count(),
                CharacterCounting::ByBytes => str.len(),
            }),
        };

        let mut chars = str.chars().peekable();

        while counting.scan(&mut chars, counting_config) {}

        counting
    }

    fn proceed_if(chars: &mut Peekable<Chars>, pred: impl Fn(char) -> bool) {
        loop {
            let Some(char) = chars.peek() else {
                return;
            };

            if !pred(*char) {
                return;
            }

            chars.next();
        }
    }

    pub fn scan(&mut self, chars: &mut Peekable<Chars>, counting_config: CountingConfig) -> bool {
        let Some(char) = chars.next() else {
            return false;
        };

        match char {
            // handle cr or crlf: +line
            '\r' => {
                if counting_config.count_line {
                    *self.line_count.as_mut().unwrap() += 1;
                }

                // eat crlf
                if chars.peek() == Some(&'\n') {
                    chars.next();
                }
            }

            // handle lf: +line
            '\n' => {
                if counting_config.count_line {
                    *self.line_count.as_mut().unwrap() += 1;
                }
            }

            char => {
                // skip to next non-whitespace character
                if char.is_whitespace() {
                    Self::proceed_if(chars, |x| x.is_whitespace())
                } else {
                    if counting_config.count_word {
                        *self.word_count.as_mut().unwrap() += 1;
                    }

                    Self::proceed_if(chars, |x| !x.is_whitespace() && x != '\n' && x != '\r')
                }
            }
        }

        true
    }
}
