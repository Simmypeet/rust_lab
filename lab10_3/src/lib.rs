use std::{fmt::Display, ops::Range};

use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn distance(&self, other: &Self) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(center: Point, radius: f64) -> Self {
        Self { center, radius }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum LocatePoint {
    Inside(Point, f64),
    Outside(Point, f64),
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct RandConfig {
    pub x_min: f64,
    pub x_max: f64, // exclusive
    pub y_min: f64,
    pub y_max: f64, // exclusive
}

pub fn gen_point_list(rnd: &mut impl Rng, configuration: &RandConfig, count: usize) -> Vec<Point> {
    let mut points = Vec::with_capacity(count);
    for _ in 0..count {
        points.push(Point {
            x: rnd.gen_range(configuration.x_min..configuration.x_max),
            y: rnd.gen_range(configuration.y_min..configuration.y_max),
        });
    }
    points
}

pub fn locate_n_point(circle: &Circle, points: &[Point]) -> Vec<LocatePoint> {
    let mut results = Vec::with_capacity(points.len());

    for point in points {
        let kind = if point.distance(&circle.center) > circle.radius {
            LocatePoint::Outside
        } else {
            LocatePoint::Inside
        };

        results.push(kind(*point, point.distance(&circle.center)));
    }

    results
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum LocatePoint2 {
    Both(Point),
    First(Point),
    Second(Point),
    Outside(Point),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Boundary {
    pub circle1: Circle,
    pub circle2: Circle,
}

pub fn locate_n_point2(bound: &Boundary, points: &[Point]) -> Vec<LocatePoint2> {
    let mut results = Vec::with_capacity(points.len());

    for point in points {
        let distance1 = point.distance(&bound.circle1.center);
        let distance2 = point.distance(&bound.circle2.center);

        let kind = match (
            distance1 <= bound.circle1.radius,
            distance2 <= bound.circle2.radius,
        ) {
            (true, true) => LocatePoint2::Both,
            (true, false) => LocatePoint2::First,
            (false, true) => LocatePoint2::Second,
            (false, false) => LocatePoint2::Outside,
        };

        results.push(kind(*point));
    }

    results
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FillMode {
    None,
    Fill(Color),
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SvgEllipse {
    pub center: Point,
    pub radius_x: f64,
    pub radius_y: f64,
    pub fill_mode: FillMode,
}

impl SvgEllipse {
    pub fn map_scale(
        circle: &Circle,
        fill_mode: FillMode,
        x_range: Range<f64>,
        y_range: Range<f64>,
        svg_size: u32,
    ) -> SvgEllipse {
        let x_scale = svg_size as f64 / (x_range.end - x_range.start);
        let y_scale = svg_size as f64 / (y_range.end - y_range.start);

        let center = Point::new(
            (circle.center.x - x_range.start) * x_scale,
            (circle.center.y - y_range.start) * y_scale,
        );

        let radius_x = circle.radius * x_scale;
        let radius_y = circle.radius * y_scale;

        SvgEllipse {
            center,
            radius_x,
            radius_y,
            fill_mode,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Svg {
    pub width: u32,
    pub height: u32,
    pub elilipse: Vec<SvgEllipse>,
}

const SVG_HEADER: &str = "<svg width=\"500\" height=\"500\" xmlns=\"http://www.w3.org/2000/svg\">";
const SVG_RECT: &str = "\t<rect width=\"100%\" height=\"100%\" fill=\"#EEEEEE\" />";

impl Display for Svg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{SVG_HEADER}")?;
        writeln!(f, "{SVG_RECT}")?;

        for ellipse in &self.elilipse {
            let fill = match ellipse.fill_mode {
                FillMode::None => "stroke=\"black\", stroke-width=\"2\", fill=\"none\"".to_string(),
                FillMode::Fill(color) => format!(" fill=\"{}\"", color.to_hex()),
            };

            writeln!(
                f,
                "<ellipse cx=\"{}\", cy=\"{}\", rx=\"{}\", ry=\"{}\"{}/>",
                ellipse.center.x, ellipse.center.y, ellipse.radius_x, ellipse.radius_y, fill
            )?;
        }

        writeln!(f, "<svg/>")
    }
}

#[cfg(test)]
mod tests {
    use crate::{Circle, Point};

    #[test]
    fn locate_n_point_test() {
        let points = [
            Point::new(1.0, 1.0),
            Point::new(0.4, 0.3),
            Point::new(0.5, 0.5),
            Point::new(0.0, 0.0),
        ];

        let point_kinds = super::locate_n_point(&Circle::new(Point::new(0.0, 0.0), 1.0), &points);

        assert!(matches!(point_kinds[0], super::LocatePoint::Outside(..)));
        assert!(matches!(point_kinds[1], super::LocatePoint::Inside(..)));
        assert!(matches!(point_kinds[2], super::LocatePoint::Inside(..)));
        assert!(matches!(point_kinds[3], super::LocatePoint::Inside(..)));
    }
}
