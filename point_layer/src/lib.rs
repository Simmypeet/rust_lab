use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Layer {
    pub name: String,
    pub color: String,
    pub points: Vec<Point>,
}

pub fn gen_layer(name: String, color: String, rng: &mut impl Rng) -> Layer {
    let mut layer = Layer {
        name,
        color,
        points: Vec::new(),
    };
    let count = rng.gen_range(20..=50);

    for _ in 0..count {
        let x = rng.gen_range(-100.0..=100.0);
        let y = rng.gen_range(-100.0..=100.0);

        layer.points.push(Point { x, y });
    }

    layer
}

pub fn gen_layer_list(rng: &mut impl Rng, n: usize) -> Vec<Layer> {
    let mut layers = Vec::new();
    for i in 0..n {
        let layer_name = format!("Layer {i}");
        let r = rng.gen_range(0..=255);
        let g = rng.gen_range(0..=255);
        let b = rng.gen_range(0..=255);
        let a = rng.gen_range(0..=255);
        let layer_color = format!("#{:02X}{:02X}{:02X}{:02X}", r, g, b, a);

        layers.push(gen_layer(layer_name, layer_color, rng));
    }
    layers
}

#[cfg(test)]
mod tests {
    #[test]
    fn gen_layer_test() {
        let mut thread_rng = rand::thread_rng();
        let layer = super::gen_layer("TEST".to_string(), "#FFFFFFFF".to_string(), &mut thread_rng);

        assert_eq!(layer.name, "TEST");
        assert_eq!(layer.color, "#FFFFFFFF");

        assert!((20..=50).contains(&layer.points.len()));

        for point in layer.points {
            assert!((-100.0..=100.0).contains(&point.x));
            assert!((-100.0..=100.0).contains(&point.y));
        }
    }

    #[test]
    fn gen_layer_list_test() {
        let mut thread_rng = rand::thread_rng();

        let layers = super::gen_layer_list(&mut thread_rng, 10);

        assert_eq!(layers.len(), 10);

        for (index, layer) in layers.iter().enumerate() {
            assert_eq!(layer.name, format!("Layer {}", index));
            assert!(layer.color.starts_with('#'))
        }
    }
}
