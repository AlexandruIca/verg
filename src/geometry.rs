#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Copy)]
pub enum PathOps {
    MoveTo {
        x: f64,
        y: f64,
    },
    MoveToRel {
        x: f64,
        y: f64,
    },
    LineTo {
        x: f64,
        y: f64,
    },
    LineToRel {
        x: f64,
        y: f64,
    },
    QuadTo {
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
    },
    QuadToRel {
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
    },
    CubicTo {
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        x3: f64,
        y3: f64,
    },
    CubicToRel {
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        x3: f64,
        y3: f64,
    },
    Close,
}

pub type Path<'a> = &'a [PathOps];

pub struct BoundingBox {
    pub min_x: usize,
    pub min_y: usize,
    pub max_x: usize,
    pub max_y: usize,
}

impl Default for BoundingBox {
    fn default() -> Self {
        Self {
            min_x: usize::MAX,
            min_y: usize::MAX,
            max_x: usize::MIN,
            max_y: usize::MIN,
        }
    }
}

fn approximate_integral(x: f64) -> f64 {
    const D: f64 = 0.67;
    x / (1.0 - D + f64::powf(f64::powf(D, 4.0) + 0.25 * x * x, 0.25))
}

fn approximate_inverse_integral(x: f64) -> f64 {
    const B: f64 = 0.39;

    x * (1.0 - B + f64::sqrt(B * B - 0.25 * x * x))
}

pub struct ParabolaParams {
    x0: f64,
    x2: f64,
    scale: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct QuadraticBezier {
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
}

impl QuadraticBezier {
    pub fn new(p0: Point, p1: Point, p2: Point) -> Self {
        Self {
            x0: p0.x,
            y0: p0.y,
            x1: p1.x,
            y1: p1.y,
            x2: p2.x,
            y2: p2.y,
        }
    }

    pub fn eval(&self, t: f64) -> Point {
        let one_minus_t = 1.0 - t;
        let x =
            self.x0 * one_minus_t * one_minus_t + 2.0 * self.x1 * t * one_minus_t + self.x2 * t * t;
        let y =
            self.y0 * one_minus_t * one_minus_t + 2.0 * self.y1 * t * one_minus_t + self.y2 * t * t;

        Point { x, y }
    }

    pub fn map_to_basic(&self) -> ParabolaParams {
        let ddx = 2.0 * self.x1 - self.x0 - self.x2;
        let ddy = 2.0 * self.y1 - self.y0 - self.y2;
        let u0 = (self.x1 - self.x0) * ddx + (self.y1 - self.y0) * ddy;
        let u2 = (self.x2 - self.x1) * ddx + (self.y2 - self.y1) * ddy;
        let cross = (self.x2 - self.x0) * ddy - (self.y2 - self.y0) * ddx;
        let x0 = u0 / cross;
        let x2 = u2 / cross;
        let scale = f64::abs(cross) / (f64::hypot(ddx, ddy) * f64::abs(x2 - x0));

        ParabolaParams { x0, x2, scale }
    }

    pub fn subdivide(&self, err: f64) -> Vec<f64> {
        let params = self.map_to_basic();
        let a0 = approximate_integral(params.x0);
        let a2 = approximate_integral(params.x2);
        let count = 0.5 * f64::abs(a2 - a0) * f64::sqrt(params.scale / err);
        let n = f64::ceil(count);
        let u0 = approximate_inverse_integral(a0);
        let u2 = approximate_inverse_integral(a2);
        let mut result = vec![0_f64];

        for i in 1..(n as i32) {
            let u = approximate_inverse_integral(a0 + ((a2 - a0) * (i as f64)) / n);
            let t = (u - u0) / (u2 - u0);
            result.push(t);
        }

        result.push(1.0);
        result
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CubicBezier {
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    x3: f64,
    y3: f64,
}

impl CubicBezier {
    pub fn new(p0: Point, p1: Point, p2: Point, p3: Point) -> Self {
        Self {
            x0: p0.x,
            y0: p0.y,
            x1: p1.x,
            y1: p1.y,
            x2: p2.x,
            y2: p2.y,
            x3: p3.x,
            y3: p3.y,
        }
    }

    pub fn eval(&self, t: f64) -> Point {
        let one_minus_t = 1.0 - t;
        let one_minus_t_squared = (1.0 - t) * (1.0 - t);
        let one_minus_t_cubed = (1.0 - t) * (1.0 - t) * (1.0 - t);

        let x = one_minus_t_cubed * self.x0
            + 3.0 * t * one_minus_t_squared * self.x1
            + 3.0 * t * 2.0 * one_minus_t * self.x2
            + t * t * t * self.x3;

        let y = one_minus_t_cubed * self.y0
            + 3.0 * t * one_minus_t_squared * self.y1
            + 3.0 * t * 2.0 * one_minus_t * self.y2
            + t * t * t * self.y3;

        Point { x, y }
    }

    fn split(&self) -> (CubicBezier, CubicBezier) {
        let m = Point {
            x: (self.x1 + self.x2) / 2.0,
            y: (self.y1 + self.y2) / 2.0,
        };

        let mut left = [Point { x: 0.0, y: 0.0 }; 4];
        let mut right = [Point { x: 0.0, y: 0.0 }; 4];

        left[0].x = self.x0;
        left[0].y = self.y0;
        right[3].x = self.x3;
        right[3].y = self.y3;
        left[1].x = (self.x0 + self.x1) / 2.0;
        left[1].y = (self.y0 + self.y1) / 2.0;
        right[2].x = (self.x2 + self.x3) / 2.0;
        right[2].y = (self.y2 + self.y3) / 2.0;
        left[2].x = (left[1].x + m.x) / 2.0;
        left[2].y = (left[1].y + m.y) / 2.0;
        right[1].x = (m.x + right[2].x) / 2.0;
        right[1].y = (m.y + right[2].y) / 2.0;
        left[3].x = (left[2].x + right[1].x) / 2.0;
        left[3].y = (left[2].y + right[1].y) / 2.0;
        right[0].x = left[3].x;
        right[0].y = left[3].y;

        (
            CubicBezier {
                x0: left[0].x,
                y0: left[0].y,
                x1: left[1].x,
                y1: left[1].y,
                x2: left[2].x,
                y2: left[2].y,
                x3: left[3].x,
                y3: left[3].y,
            },
            CubicBezier {
                x0: right[0].x,
                y0: right[0].y,
                x1: right[1].x,
                y1: right[1].y,
                x2: right[2].x,
                y2: right[2].y,
                x3: right[3].x,
                y3: right[3].y,
            },
        )
    }

    fn is_sufficiently_flat(&self, tolerance: f64) -> bool {
        let mut ux = 3.0 * self.x1 - 2.0 * self.x0 - self.x3;
        let mut uy = 3.0 * self.y1 - 2.0 * self.y0 - self.y3;
        let mut vx = 3.0 * self.x2 - 2.0 * self.x3 - self.x0;
        let mut vy = 3.0 * self.y2 - 2.0 * self.y3 - self.y0;

        ux *= ux;
        uy *= uy;
        vx *= vx;
        vy *= vy;

        if ux < vx {
            ux = vx;
        }
        if uy < vy {
            uy = vy;
        }

        ux + uy <= tolerance
    }

    fn subdivide_impl(&self, lines: &mut Vec<Point>, tolerance: f64) {
        if self.is_sufficiently_flat(tolerance) {
            lines.push(Point {
                x: self.x3,
                y: self.y3,
            });

            return;
        }
        let (left, right) = self.split();

        left.subdivide_impl(lines, tolerance);
        right.subdivide_impl(lines, tolerance);
    }

    pub fn subdivide(&self, tolerance: f64) -> Vec<Point> {
        let mut result = vec![self.eval(0.0)];

        self.subdivide_impl(&mut result, tolerance);

        result
    }
}
