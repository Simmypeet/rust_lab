use std::{fs::File, io::BufReader};

use headr::Input;

fn main() {
    let argument = match headr::Argument::from_env_args() {
        Ok(ok) => ok,
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    };

    match argument.input {
        Input::Files(files) => {
            let with_header = files.len() > 1;
            for i in 0..files.len() {
                let file_name = &files[i];

                let file = match File::open(file_name) {
                    Ok(file) => file,
                    Err(err) => {
                        eprint!("{}: {err}", file_name.display());
                        std::process::exit(1);
                    }
                };

                if with_header {
                    println!("==> {} <==", file_name.display());
                }

                let reader = BufReader::new(file);

                if let Err(err) = headr::print_output(reader, argument.output) {
                    eprintln!("{err}");
                    std::process::exit(1);
                }

                if i != files.len() - 1 {
                    println!();
                }
            }
        }
        Input::Stdin => {
            let reader = BufReader::new(std::io::stdin());

            if let Err(err) = headr::print_output(reader, argument.output) {
                eprintln!("{err}");
                std::process::exit(1);
            }
        }
    }
}
