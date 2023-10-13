use std::path::PathBuf;

use clap::Parser;
use lab10_3::{Boundary, Circle, Color, FillMode, Point, RandConfig, Svg, SvgEllipse};

#[derive(Parser, Debug, Clone, PartialEq, PartialOrd)]
pub struct RawArguments {
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
    pub count: usize,

    pub circle1_x: f64,
    pub circle1_y: f64,
    pub circle1_radius: f64,

    pub circle2_x: f64,
    pub circle2_y: f64,
    pub circle2_radius: f64,

    pub output_file: PathBuf,

    #[clap(default_value = "500")]
    pub size: u32,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Arguments {
    pub rand_config: RandConfig,
    pub point_count: usize,
    pub boundary: Boundary,
    pub output_file: PathBuf,
    pub size: u32,
}

impl Arguments {
    pub fn from_raw(raw: RawArguments) -> Self {
        Self {
            rand_config: RandConfig {
                x_min: raw.x_min,
                x_max: raw.x_max,
                y_min: raw.y_min,
                y_max: raw.y_max,
            },
            point_count: raw.count,
            boundary: Boundary {
                circle1: Circle::new(Point::new(raw.circle1_x, raw.circle1_y), raw.circle1_radius),
                circle2: Circle::new(Point::new(raw.circle2_x, raw.circle2_y), raw.circle2_radius),
            },
            output_file: raw.output_file,
            size: raw.size,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let raw_args = RawArguments::parse();
    let args = Arguments::from_raw(raw_args);

    let mut rng = rand::thread_rng();
    let points = lab10_3::gen_point_list(&mut rng, &args.rand_config, args.point_count);
    let results = lab10_3::locate_n_point2(&args.boundary, &points);

    let mut output_file = std::fs::File::create(args.output_file)?;
    let mut svg = Svg {
        width: args.size,
        height: args.size,
        elilipse: Vec::new(),
    };

    svg.elilipse.push(SvgEllipse::map_scale(
        &args.boundary.circle1,
        FillMode::None,
        args.rand_config.x_min..args.rand_config.x_max,
        args.rand_config.y_min..args.rand_config.y_max,
        args.size,
    ));
    svg.elilipse.push(SvgEllipse::map_scale(
        &args.boundary.circle2,
        FillMode::None,
        args.rand_config.x_min..args.rand_config.x_max,
        args.rand_config.y_min..args.rand_config.y_max,
        args.size,
    ));

    let circle_radius = (args.rand_config.x_max - args.rand_config.x_min)
        .max(args.rand_config.y_max - args.rand_config.y_min)
        / 100.0;

    for result in results {
        let (point, color) = match result {
            lab10_3::LocatePoint2::Both(point) => (point, Color::new(255, 0, 0)),
            lab10_3::LocatePoint2::First(point) => (point, Color::new(0, 255, 0)),
            lab10_3::LocatePoint2::Second(point) => (point, Color::new(0, 0, 255)),
            lab10_3::LocatePoint2::Outside(point) => (point, Color::new(0, 0, 0)),
        };

        svg.elilipse.push(SvgEllipse::map_scale(
            &Circle::new(point, circle_radius),
            FillMode::Fill(color),
            args.rand_config.x_min..args.rand_config.x_max,
            args.rand_config.y_min..args.rand_config.y_max,
            args.size,
        ));
    }

    use std::io::Write;

    write!(output_file, "{}", svg)?;

    Ok(())
}
