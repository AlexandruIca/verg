use crate::geometry::Point;
use crate::math::Angle;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

pub fn clamp<T: std::cmp::PartialOrd>(v: T, min: T, max: T) -> T {
    if v < min {
        min
    } else if v > max {
        max
    } else {
        v
    }
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

impl Color {
    pub const fn black() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }
    }

    pub const fn white() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }
    }

    pub const fn red() -> Self {
        Self {
            r: 1.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }
    }

    pub const fn blue() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 1.0,
            a: 1.0,
        }
    }

    pub const fn cyan() -> Self {
        Self {
            r: 0.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }
    }

    pub const fn coral() -> Self {
        Self {
            r: 1.0,
            g: 0.498,
            b: 0.313,
            a: 1.0,
        }
    }

    pub const fn dark_slate_blue() -> Self {
        Self {
            r: 0.282,
            g: 0.239,
            b: 0.545,
            a: 1.0,
        }
    }

    pub const fn crimson() -> Self {
        Self {
            r: 0.862,
            g: 0.078,
            b: 0.235,
            a: 1.0,
        }
    }

    pub const fn forest_green() -> Self {
        Self {
            r: 0.133,
            g: 0.545,
            b: 0.133,
            a: 1.0,
        }
    }

    pub const fn yellow() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 0.0,
            a: 1.0,
        }
    }

    pub const fn steel_blue() -> Self {
        Self {
            r: 0.274,
            g: 0.509,
            b: 0.705,
            a: 1.0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FillStyle<'a> {
    Plain(Color),
    LinearGradient {
        stops: &'a [(Color, f64)],
        angle: Angle,
    },
    RadialGradient {
        stops: &'a [(Color, f64)],
        translation: Point,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum FillRule {
    EvenOdd,
    NonZero,
}

impl Default for FillRule {
    fn default() -> FillRule {
        FillRule::NonZero
    }
}
