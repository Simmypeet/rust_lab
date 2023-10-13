use std::fs::File;

use point_layer::Layer;

const SVG_HEADER: &str = "<svg width=\"500\" height=\"500\" xmlns=\"http://www.w3.org/2000/svg\">";
const SVG_RECT: &str = "\t<rect width=\"100%\" height=\"100%\" fill=\"#EEEEEE\" />";

pub fn save_layers<'a>(layers: impl Iterator<Item = &'a Layer>, mut output_file: File) {
    use std::io::Write;

    writeln!(output_file, "{SVG_HEADER}").expect("failed to write to the output file");
    writeln!(output_file, "{SVG_RECT}").expect("failed to write to the output file");

    for layer in layers {
        for point in &layer.points {
            writeln!(
                output_file,
                "<circle cx=\"{}\", cy=\"{}\", r=\"50\", fill=\"{}\" />",
                (point.x + 100.) * 2.5,
                500. - (point.y + 100.) * 2.5,
                layer.color
            )
            .expect("failed to write to the output file");
        }
    }

    writeln!(output_file, "</svg>").expect("failed to write to the output file")
}
