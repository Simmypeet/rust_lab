use std::process::ExitCode;

fn main() -> ExitCode {
    let config = match tailr::Config::get() {
        Ok(ok) => ok,
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::FAILURE;
        }
    };

    tailr::run(config);

    ExitCode::SUCCESS
}
