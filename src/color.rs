#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

fn clamp_f64(v: f64, min: f64, max: f64) -> f64 {
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
            r: clamp_f64(self.r, min, max),
            g: clamp_f64(self.g, min, max),
            b: clamp_f64(self.b, min, max),
            a: clamp_f64(self.a, min, max),
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
