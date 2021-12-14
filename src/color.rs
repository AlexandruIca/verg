pub struct Color {
    r: f64,
    g: f64,
    b: f64,
    a: f64,
}

impl Default for Color {
    fn default() -> Color {
        Color {
            r: 0.0f64,
            g: 0.0f64,
            b: 0.0f64,
            a: 0.0f64,
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
