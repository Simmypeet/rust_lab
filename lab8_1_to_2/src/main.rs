use std::process::ExitCode;

fn main() -> ExitCode {
    let config = match cutr::Config::get_from_args() {
        Ok(ok) => ok,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::FAILURE;
        }
    };

    cutr::run(config);

    ExitCode::SUCCESS
}
