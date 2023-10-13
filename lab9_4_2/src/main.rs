use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Argument {
    #[arg(help = "the number of layer")]
    pub layer_count: usize,
    #[arg(help = "the output file of the svg output")]
    pub output_file: PathBuf,
}

fn main() {
    let argument = Argument::parse();

    let mut thread_rng = rand::thread_rng();

    let layer = point_layer::gen_layer_list(&mut thread_rng, argument.layer_count);

    let output_file =
        std::fs::File::create(argument.output_file).expect("failed to create an output file");

    svg_gen::save_layers(layer.iter(), output_file)
}
