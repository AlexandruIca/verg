#[derive(Debug)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl Default for Color {
    fn default() -> Color {
        Color {
            r: 0.0_f64,
            g: 0.0_f64,
            b: 0.0_f64,
            a: 0.0_f64,
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
