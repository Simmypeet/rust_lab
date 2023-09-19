fn main() {
    let config = match findr::Config::get() {
        Ok(ok) => ok,
        Err(err) => {
            eprintln!("findr: {err}");
            std::process::exit(1)
        }
    };

    for path in config.paths {
        let matches = findr::get_matches(&path, &config.entry_types, &config.regexes);

        for result in matches {
            match result {
                Ok(path) => println!("{}", path.display()),
                Err(err) => {
                    if let Some(path) = err.path {
                        eprintln!("{}: {}", path.display(), err.error);
                    } else {
                        eprintln!("{}", err.error);
                    }
                }
            }
        }
    }
}
