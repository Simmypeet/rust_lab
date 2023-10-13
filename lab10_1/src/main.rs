use std::process::ExitCode;

fn main() -> ExitCode {
    let arg = match commr::Config::from_args() {
        Ok(ok) => ok,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::FAILURE;
        }
    };

    if let Err(err) = commr::run(arg) {
        eprintln!("{err}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
