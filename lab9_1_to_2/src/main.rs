use std::process::ExitCode;

fn main() -> ExitCode {
    let ok = match grepr::Config::from_args() {
        Ok(ok) => ok,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::FAILURE;
        }
    };

    grepr::run(ok);

    ExitCode::SUCCESS
}
