use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter},
    path::Path,
};

use uniqr::Line;

fn get_buf_read(input: Option<&Path>) -> Result<Box<dyn BufRead>, String> {
    if let Some(path) = input {
        let file = match File::open(path) {
            Ok(x) => x,
            Err(e) => {
                return Err(format!("{}: {}", path.display(), e));
            }
        };

        Ok(Box::new(BufReader::new(file)))
    } else {
        Ok(Box::new(BufReader::new(std::io::stdin())))
    }
}

fn main() {
    let arg = uniqr::Argument::get();

    let buf_reader = match get_buf_read(arg.input.as_ref().map(|x| x.as_ref())) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };

    let mut write = match arg.output.map_or_else(
        || Ok(Box::new(BufWriter::new(std::io::stdout())) as Box<dyn std::io::Write>),
        |path| File::create(path).map(|x| Box::new(BufWriter::new(x)) as Box<dyn std::io::Write>),
    ) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };

    let lines = match Line::from_reader(buf_reader) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };

    if let Err(e) = write_to_buffer(&mut write, lines, arg.count) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn write_to_buffer<T: std::io::Write>(
    buffer: &mut T,
    lines: Vec<uniqr::Line>,
    count: bool,
) -> std::io::Result<()> {
    let mut index = 0;

    while index < lines.len() {
        let line = &lines[index];

        // somehow, if an empty line is found between messages we print it.
        // however, if they apear last, we don't print it
        if count && !(line.value.is_empty() && index == lines.len() - 1) {
            write!(buffer, "{:>4} ", line.count)?;
        }

        write!(buffer, "{}", line.value)?;

        // if the line has multiple count, it always prints out the line feed.
        if (index < lines.len() - 1) || (line.count > 1) {
            writeln!(buffer)?;
        }

        index += 1;
    }

    Ok(())
}
