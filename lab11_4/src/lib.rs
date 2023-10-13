use std::{
    collections::HashMap,
    io::{BufRead, Write},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Xpm2 {
    colors: HashMap<char, Color>,
    pixels: Vec<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum ReadError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error("expected `! XPM2` header")]
    InvalidHeaderFormat,

    #[error("The file contain insufficient data")]
    NotEnoughData,

    #[error("The file contains invalid values")]
    InvalidValues,

    #[error("The file contains invalid color format")]
    InvalidColorFormat,

    #[error("The file contains extraneous data")]
    ExtraneousData,

    #[error("Image with zero dimension is not allowed")]
    ZeroDimension,

    #[error("The file contains invalid pixel data")]
    InvalidPixel,
}

impl Xpm2 {
    pub fn read(bufread: impl BufRead) -> Result<Self, ReadError> {
        let mut lines = bufread.lines();

        let header = lines.next().ok_or(ReadError::NotEnoughData)??;

        if header != "! XPM2" {
            return Err(ReadError::InvalidHeaderFormat);
        }

        let (width, height, color_count) = {
            let values_line = lines.next().ok_or(ReadError::NotEnoughData)??;

            let mut values = values_line.split_whitespace();

            match (
                values
                    .next()
                    .ok_or(ReadError::InvalidValues)?
                    .parse::<usize>(),
                values
                    .next()
                    .ok_or(ReadError::InvalidValues)?
                    .parse::<usize>(),
                values
                    .next()
                    .ok_or(ReadError::InvalidValues)?
                    .parse::<usize>(),
                values
                    .next()
                    .ok_or(ReadError::InvalidValues)?
                    .parse::<usize>(),
            ) {
                // 1 char per pixel is enough
                (Ok(width), Ok(height), Ok(color_count), Ok(1)) => {
                    if values.next().is_some() {
                        return Err(ReadError::InvalidValues);
                    }

                    if width == 0 || height == 0 {
                        return Err(ReadError::ZeroDimension);
                    }

                    (width, height, color_count)
                }
                _ => return Err(ReadError::InvalidValues),
            }
        };

        let colors = {
            let mut colors = HashMap::new();

            for _ in 0..color_count {
                let color_line = lines.next().ok_or(ReadError::NotEnoughData)??;

                let mut color = color_line.split_whitespace();

                let char = color.next().ok_or(ReadError::InvalidColorFormat)?;
                let chars = char.chars();

                if chars.count() != 1 {
                    return Err(ReadError::InvalidColorFormat);
                }

                let char = char.chars().next().unwrap();

                if color.next().ok_or(ReadError::NotEnoughData)? != "c" {
                    return Err(ReadError::InvalidColorFormat);
                }

                let hex_string = color.next().unwrap();

                if hex_string.len() != 7 {
                    return Err(ReadError::InvalidColorFormat);
                }

                if hex_string.as_bytes()[0] != b'#' {
                    return Err(ReadError::InvalidColorFormat);
                }

                let r = u8::from_str_radix(&hex_string[1..3], 16);
                let g = u8::from_str_radix(&hex_string[3..5], 16);
                let b = u8::from_str_radix(&hex_string[5..7], 16);

                match (r, g, b) {
                    (Ok(r), Ok(g), Ok(b)) => {
                        colors.insert(char, Color { r, g, b });
                    }
                    _ => return Err(ReadError::InvalidColorFormat),
                }
            }

            colors
        };

        let mut pixels = Vec::new();
        for _ in 0..height {
            let line = lines.next().ok_or(ReadError::NotEnoughData)??;

            if line.len() != width {
                return Err(ReadError::InvalidPixel);
            }

            for char in line.chars() {
                if !colors.contains_key(&char) {
                    return Err(ReadError::InvalidPixel);
                }
            }

            pixels.push(line);
        }

        if lines.next().is_some() {
            return Err(ReadError::ExtraneousData);
        }

        Ok(Self { colors, pixels })
    }

    pub fn write_as_svg(
        &self,
        mut write: impl Write,
        pixel_size: usize,
    ) -> Result<(), std::io::Error> {
        let total_width = self.pixels[0].len() * pixel_size;
        let total_height = self.pixels.len() * pixel_size;
        // write svg header
        writeln!(
            write,
            r#"<svg width="{total_width}" height="{total_height}" xmlns="http://www.w3.org/2000/svg">"#
        )?;

        // write svg style
        writeln!(write, "\t<style type=\"text/css\">")?;
        writeln!(write, "\t\trect {{ stroke: #00FFFF }} ")?;

        for (char, color) in &self.colors {
            writeln!(
                write,
                "\t\trect.c{} {{ fill: #{:02X}{:02X}{:02X} }}",
                *char as u32, color.r, color.g, color.b
            )?;
        }

        writeln!(write, "\t</style>")?;

        for (line_num, line) in self.pixels.iter().enumerate() {
            for (index, char) in line.chars().enumerate() {
                let x = index * pixel_size;
                let y = line_num * pixel_size;
                let char_val = char as u32;
                writeln!(
                    write,
                    "\t<rect class=\"c{char_val}\", x=\"{x}\", y=\"{y}\", width=\"{pixel_size}\", height=\"{pixel_size}\" />"
                )?;
            }
        }

        writeln!(write, "\t</style>")?;

        // write svg footer
        writeln!(write, "</svg>")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    const TEST_XPM2: &str = r#"! XPM2
4 4 2 1
# c #000000
- c #ffffff
##--
##--
--##
--##"#;

    #[test]
    fn read_xpm2_test() {
        let xpm2 = super::Xpm2::read(TEST_XPM2.as_bytes()).unwrap();

        assert_eq!(xpm2.colors.len(), 2);
        assert_eq!(xpm2.pixels.len(), 4);

        assert_eq!(xpm2.colors[&'#'].r, 0);
        assert_eq!(xpm2.colors[&'#'].g, 0);
        assert_eq!(xpm2.colors[&'#'].b, 0);

        assert_eq!(xpm2.colors[&'-'].r, 255);
        assert_eq!(xpm2.colors[&'-'].g, 255);
        assert_eq!(xpm2.colors[&'-'].b, 255);

        assert_eq!(xpm2.pixels[0], "##--");
        assert_eq!(xpm2.pixels[1], "##--");
        assert_eq!(xpm2.pixels[2], "--##");
        assert_eq!(xpm2.pixels[3], "--##");
    }
}
