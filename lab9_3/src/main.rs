use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Argument {
    #[arg(help = "Number of layers")]
    pub n: usize,

    #[arg(help = "The output file of the program")]
    pub output: PathBuf,
}
fn main() {
    // i assume we don't need to use clap for this exercise
    let arg = Argument::parse();

    let mut thread_rng = rand::thread_rng();
    let layers = point_layer::gen_layer_list(&mut thread_rng, arg.n);

    let mut output_file =
        std::fs::File::create(arg.output).expect("failed to create an output file");

    for layer in layers {
        use std::io::Write;

        write!(output_file, "\"{}\", {}", layer.name, layer.color)
            .expect("failed to write to the output file");

        for point in layer.points {
            write!(output_file, ", {}, {}", point.x, point.y)
                .expect("failed to write to the output file");
        }

        writeln!(output_file).expect("failed to write to the output file");
    }
}
