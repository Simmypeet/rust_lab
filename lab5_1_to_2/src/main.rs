use std::io::Read;

use wcr::{Counting, Input};

fn main() {
    let argument = match wcr::Argument::from_env_args() {
        Ok(ok) => ok,
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    };

    match argument.input {
        Input::Files(files) => {
            assert!(!files.is_empty());

            let file_contents = files.iter().map(|x| (x, std::fs::read_to_string(x)));

            let multiple = file_contents.clone().count() > 1;

            let mut total_count = Counting::default();

            for (file_path, content) in file_contents {
                let content = match content {
                    Ok(content) => content,
                    Err(err) => {
                        eprintln!("{}: {err}", file_path.display());
                        continue;
                    }
                };

                let counting = Counting::count_from_str(&content, argument.counting_config);

                if let Some(words) = counting.word_count {
                    *total_count.word_count.get_or_insert(0) += words;
                }

                if let Some(lines) = counting.line_count {
                    *total_count.line_count.get_or_insert(0) += lines;
                }

                if let Some(characters_or_bytes) = counting.character_or_byte_count {
                    *total_count.character_or_byte_count.get_or_insert(0) += characters_or_bytes;
                }

                print_count(counting, Some(file_path.to_str().unwrap()))
            }

            if multiple {
                print_count(total_count, Some("total"));
            }
        }

        Input::Stdin => {
            let mut user_input = String::new();
            let mut stdin = std::io::stdin();
            stdin.read_to_string(&mut user_input).unwrap();

            let counting = Counting::count_from_str(&user_input, argument.counting_config);

            print_count(counting, None);
        }
    }
}

fn print_count(counting: Counting, post_fix_str: Option<&str>) {
    if let Some(lines) = counting.line_count {
        print!("{lines:>8}");
    }

    if let Some(words) = counting.word_count {
        print!("{words:>8}");
    }

    if let Some(character_or_bytes) = counting.character_or_byte_count {
        print!("{character_or_bytes:>8}");
    }

    if let Some(postfix_string) = post_fix_str {
        print!(" {postfix_string}");
    }

    println!()
}
