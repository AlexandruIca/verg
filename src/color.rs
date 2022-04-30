#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

pub fn clamp<T: std::cmp::PartialOrd>(v: T, min: T, max: T) -> T {
    return if v < min {
        min
    } else if v > max {
        max
    } else {
        v
    };
}

impl Color {
    pub fn clamp(&self) -> Color {
        let (min, max) = (0.0_f64, 1.0_f64);
        Color {
            r: clamp(self.r, min, max),
            g: clamp(self.g, min, max),
            b: clamp(self.b, min, max),
            a: clamp(self.a, min, max),
        }
    }
}

impl Default for Color {
    fn default() -> Color {
        Color {
            r: 0.0_f64,
            g: 0.0_f64,
            b: 0.0_f64,
            a: 1.0_f64,
        }
    }
}

pub enum FillStyle {
    Plain(Color),
}

pub enum FillRule {
    EvenOdd,
    NonZero,
}

impl Default for FillRule {
    fn default() -> FillRule {
        FillRule::NonZero
    }
}
