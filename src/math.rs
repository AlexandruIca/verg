use crate::canvas::CanvasDescription;
use crate::geometry::Point;

#[derive(Debug, Clone, Copy, Default)]
pub struct Angle(f64);

impl Angle {
    pub fn from_radians(radians: f64) -> Self {
        Self(radians)
    }

    pub fn from_degrees(degrees: f64) -> Self {
        Self(degrees.to_radians())
    }

    pub fn to_radians(&self) -> f64 {
        self.0
    }

    pub fn to_degrees(&self) -> f64 {
        self.0 * 180.0 / core::f32::consts::PI as f64
    }
}

//
// Affine transformations are represented using an augmented matrix:
//
// | a00  a01  x |
// |             |
// | a10  a11  y |
//
#[derive(Debug, Clone, Copy)]
struct AffineTransform {
    pub a00: f64,
    pub a01: f64,
    pub a10: f64,
    pub a11: f64,
    pub x: f64,
    pub y: f64,
}

impl AffineTransform {
    pub const fn identity() -> Self {
        Self {
            a00: 1.0,
            a01: 0.0,
            a10: 1.0,
            a11: 0.0,
            x: 0.0,
            y: 0.0,
        }
    }

    //
    //                        | a00  a01  x |
    // | point.x  point.y | * |             |
    //                        | a10  a11  y |
    //
    pub fn transform_point(&self, point: &Point) -> Point {
        Point {
            x: (point.x * self.a00 + point.y * self.a10) + self.x,
            y: (point.x * self.a01 + point.y * self.a11) + self.y,
        }
    }

    pub fn translate(x: f64, y: f64) -> Self {
        Self {
            a00: 1.0,
            a01: 0.0,
            a10: 0.0,
            a11: 1.0,
            x,
            y,
        }
    }

    pub fn rotate(angle: Angle) -> Self {
        let (sin, cos) = angle.to_radians().sin_cos();

        Self {
            a00: cos,
            a01: sin,
            a10: -sin,
            a11: cos,
            x: 0.0,
            y: 0.0,
        }
    }

    pub fn rotate_around(point: &Point, angle: Angle) -> Self {
        Self::translate(point.x, point.y)
            .then_rotate(angle)
            .then_translate(-point.x, -point.y)
    }

    pub fn scale(x: f64, y: f64) -> Self {
        Self {
            a00: x,
            a01: 0.0,
            a10: 0.0,
            a11: y,
            x: 0.0,
            y: 0.0,
        }
    }

    pub fn skew(x: Angle, y: Angle) -> Self {
        Self {
            a00: 1.0,
            a01: f64::tan(y.to_radians()),
            a10: f64::tan(x.to_radians()),
            a11: 1.0,
            x: 0.0,
            y: 0.0,
        }
    }

    pub fn combine(first: &AffineTransform, second: &AffineTransform) -> Self {
        let a00 = first.a00 * second.a00 + first.a10 * second.a01;
        let a10 = first.a00 * second.a10 + first.a10 * second.a11;
        let a01 = first.a01 * second.a00 + first.a11 * second.a01;
        let a11 = first.a01 * second.a10 + first.a11 * second.a11;
        let x = first.x * second.a00 + first.y * second.a01 + second.x;
        let y = first.x * second.a10 + first.y * second.a11 + second.y;

        Self {
            a00,
            a10,
            a01,
            a11,
            x,
            y,
        }
    }

    pub fn then(&self, other: &AffineTransform) -> Self {
        Self::combine(self, other)
    }

    pub fn then_translate(&self, x: f64, y: f64) -> Self {
        let mut translate = *self;

        translate.x += x;
        translate.y += y;

        translate
    }

    pub fn then_rotate(&self, angle: Angle) -> Self {
        Self::combine(self, &Self::rotate(angle))
    }

    pub fn then_rotate_around(&self, point: &Point, angle: Angle) -> Self {
        Self::combine(self, &Self::rotate_around(point, angle))
    }

    pub fn then_scale(&self, x: f64, y: f64) -> Self {
        Self::combine(self, &Self::scale(x, y))
    }

    pub fn then_skew(&self, x: Angle, y: Angle) -> Self {
        Self::combine(self, &Self::skew(x, y))
    }
}

// https://www.w3.org/TR/SVG2/coords.html#ComputingAViewportsTransform
pub fn map_viewbox(canvas: &CanvasDescription, point: &Point) -> Point {
    let view = canvas.viewbox;

    let (vbx, vby, vb_width, vb_height) = (view.x, view.y, view.width, view.height);
    let (ex, ey, e_width, e_height) = (0.0, 0.0, canvas.width as f64, canvas.height as f64);

    let scale_x = e_width / vb_width;
    let scale_y = e_height / vb_height;

    let mut translate_x = ex - (vbx * scale_x);
    let mut translate_y = ey - (vby * scale_y);

    translate_x += (e_width - vb_width * scale_x) / 2.0;
    translate_y += (e_height - vb_height * scale_y) / 2.0;

    let translated = AffineTransform::translate(translate_x, translate_y).transform_point(point);
    let result = AffineTransform::scale(scale_x, scale_y).transform_point(&translated);

    result
}
