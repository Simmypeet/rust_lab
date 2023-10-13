use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Argument {
    #[arg(help = "the color of the layer")]
    pub color: String,
    #[arg(help = "the output file of the svg output")]
    pub output_file: PathBuf,
}

fn main() {
    let argument = Argument::parse();

    let mut thread_rng = rand::thread_rng();

    let layer = point_layer::gen_layer("MY LAYERR".into(), "#00FF00FF".into(), &mut thread_rng);

    let output_file =
        std::fs::File::create(argument.output_file).expect("failed to create an output file");

    svg_gen::save_layers(std::iter::once(&layer), output_file)
}
