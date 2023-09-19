use std::{fs::File, io::Read, process::ExitCode};

fn main() -> ExitCode {
    let config = pointr::Config::from_args();

    let config = match config {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::FAILURE;
        }
    };

    let reader = match config.input {
        pointr::Input::File(file) => Box::new(match File::open(&file) {
            Ok(file) => Box::new(file) as Box<dyn Read>,
            Err(err) => {
                eprintln!("{}: {err}", file.display());
                return ExitCode::FAILURE;
            }
        }),
        pointr::Input::Stdin => Box::new(std::io::stdin()) as Box<dyn Read>,
    };

    let writer = match File::create(&config.output) {
        Ok(writer) => writer,
        Err(err) => {
            eprintln!("{}: {err}", config.output.display());
            return ExitCode::FAILURE;
        }
    };

    let points = pointr::load_points(reader);
    let tagged_point = pointr::tag_points(&points);
    pointr::save_points(writer, &tagged_point);

    ExitCode::SUCCESS
}
