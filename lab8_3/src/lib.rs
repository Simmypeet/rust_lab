use std::{
    error::Error,
    io::{Read, Write},
    path::PathBuf,
};

use clap::{App, Arg};
use csv::{ReaderBuilder, Writer};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Point {
    x: f64,
    y: f64,
    color: String,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            color: "#00000000".to_string(),
        }
    }

    pub fn new_with_color(x: f64, y: f64, color: String) -> Self {
        Self { x, y, color }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

const GREEN_TONE: &str = "#80FF8080";
const RED_TONE: &str = "#FF808080";

pub fn tag_points(pt_list: &[Point]) -> Vec<Point> {
    let mut result = Vec::new();

    for mut pt in pt_list.iter().cloned() {
        if pt.magnitude() > 1. {
            pt.color = RED_TONE.to_string();
        } else {
            pt.color = GREEN_TONE.to_string();
        }
        result.push(pt)
    }

    result
}

pub fn save_points<T: Write>(writer: T, pt_list: &[Point]) -> T {
    let mut writer = Writer::from_writer(writer);

    for pt in pt_list {
        writer
            .write_record(&[pt.x.to_string(), pt.y.to_string(), pt.color.clone()])
            .expect("should've written a record");
    }

    writer.flush().expect("should've flushed the writer");

    writer.into_inner().unwrap()
}

pub fn load_points(reader: impl Read) -> Vec<Point> {
    let mut reader = ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(false)
        .trim(csv::Trim::All)
        .from_reader(reader);

    let mut points = Vec::new();

    for record in reader.records() {
        let Ok(record) = record else {
            continue;
        };

        let x: f64 = record
            .get(0)
            .expect("should've had a value for x")
            .parse::<f64>()
            .expect("should've been a number");
        let y: f64 = record
            .get(1)
            .expect("should've had a value for y")
            .parse::<f64>()
            .expect("should've been a number");

        points.push(Point::new(x, y));
    }

    points
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Input {
    File(PathBuf),
    Stdin,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Config {
    pub input: Input,
    pub output: PathBuf,
}

impl Config {
    pub fn from_args() -> Result<Self, Box<dyn Error>> {
        let matches = App::new("pointr")
            .version("0.1.0")
            .author("66011245@kmitl.ac.th")
            .about("exercise four")
            .arg(
                Arg::with_name("input")
                    .value_name("INPUT")
                    .takes_value(true)
                    .default_value("-")
                    .help("The input file for the program"),
            )
            .arg(
                Arg::with_name("output")
                    .value_name("OUTPUT")
                    .takes_value(true)
                    .help("The output file for the program"),
            )
            .get_matches();

        let input = match matches.value_of("input").ok_or("`input` expected")? {
            "-" => Input::Stdin,
            path => Input::File(path.into()),
        };

        let output = PathBuf::from(matches.value_of("output").ok_or("`output` expected")?);

        Ok(Config { input, output })
    }
}

#[cfg(test)]
mod tests {
    use crate::{Point, GREEN_TONE, RED_TONE};

    #[test]
    fn tag_points_test() {
        // 2 points in circle, 1 point outside circle
        let point = [
            Point::new(0.0, 0.0),
            Point::new(1.0, -1.0),
            Point::new(0.5, 0.5),
        ];

        let result = super::tag_points(&point);

        assert_eq!(
            &result,
            &[
                Point::new_with_color(0.0, 0.0, GREEN_TONE.to_string()),
                Point::new_with_color(1.0, -1.0, RED_TONE.to_string()),
                Point::new_with_color(0.5, 0.5, GREEN_TONE.to_string()),
            ]
        )
    }

    const EXPECTED_OUTPUT_SAVE_POINTS: &str = "0,0,#00000000
1,-1,#FF808080
0.5,0.5,#80FF8080\n";

    #[test]
    fn save_points_tests() {
        let mut buffer = Vec::new();

        let points = vec![
            Point::new_with_color(0.0, 0.0, "#00000000".to_string()),
            Point::new_with_color(1.0, -1.0, RED_TONE.to_string()),
            Point::new_with_color(0.5, 0.5, GREEN_TONE.to_string()),
        ];

        buffer = super::save_points(buffer, &points);

        let string = String::from_utf8(buffer).unwrap();

        assert_eq!(string, EXPECTED_OUTPUT_SAVE_POINTS)
    }
}
