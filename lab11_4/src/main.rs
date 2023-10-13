use std::{error::Error, path::PathBuf};

use clap::Parser;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Parser)]
pub struct Config {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::parse();

    let input_path = config.input_path;
    let file = std::fs::File::open(input_path)?;

    let xpm2 = xpm2r::Xpm2::read(std::io::BufReader::new(file))?;

    let output_path = config.output_path;
    let file = std::fs::File::create(output_path)?;

    xpm2.write_as_svg(std::io::BufWriter::new(file), 20)?;

    Ok(())
}
