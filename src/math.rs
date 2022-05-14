use crate::canvas::CanvasDescription;
use crate::geometry::Point;

#[derive(Debug, Clone, Copy, Default)]
pub struct Angle(pub f64);

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

pub fn translate(point: &Point, x: f64, y: f64) -> Point {
    Point {
        x: point.x + x,
        y: point.y + y,
    }
}

pub fn rotate(point: &Point, angle: Angle) -> Point {
    let (sin, cos) = angle.to_radians().sin_cos();

    Point {
        x: point.x * sin - point.y * cos,
        y: point.x * cos + point.y * sin,
    }
}

pub fn rotate_around(point: &Point, around: &Point, angle: Angle) -> Point {
    let p = translate(point, -around.x, -around.y);
    let p = rotate(&p, angle);

    translate(&p, around.x, around.y)
}

pub fn scale(point: &Point, sx: f64, sy: f64) -> Point {
    Point {
        x: point.x * sx,
        y: point.y * sy,
    }
}

pub fn scale_around(point: &Point, around: &Point, sx: f64, sy: f64) -> Point {
    let p = translate(point, -around.x, -around.y);
    let p = scale(&p, sx, sy);

    translate(&p, around.x, around.y)
}

pub fn skew(point: &Point, x: Angle, y: Angle) -> Point {
    Point {
        x: point.x + point.y * y.to_radians().tan(),
        y: point.y + point.x * x.to_radians().tan(),
    }
}

pub fn skew_around(point: &Point, around: &Point, x: Angle, y: Angle) -> Point {
    let p = translate(point, -around.x, -around.y);
    let p = skew(&p, x, y);

    translate(&p, around.x, around.y)
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

    let translated = translate(point, translate_x, translate_y);
    scale(&translated, scale_x, scale_y)
}
