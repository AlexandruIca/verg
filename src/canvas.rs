use crate::color::{Color, FillRule, FillStyle};
use crate::geometry::{Path, PathOps, Point};
use std::vec::Vec;

#[derive(Debug, Clone)]
pub struct AccumulationCell {
    pub area: f64,
}

#[derive(Debug)]
pub struct CanvasDescription {
    pub width: usize,
    pub height: usize,
    pub pixel_size: usize,
    pub background_color: Color,
}

impl Default for CanvasDescription {
    fn default() -> Self {
        CanvasDescription {
            width: 600,
            height: 600,
            pixel_size: 256,
            background_color: Color::default(),
        }
    }
}

const NUM_CHANNELS: usize = 4;

#[derive(Debug)]
pub struct Canvas {
    buffer: Vec<f64>,
    accumulation_buffer: Vec<AccumulationCell>,
    pub desc: CanvasDescription,
}

impl Canvas {
    pub fn new(desc: CanvasDescription) -> Canvas {
        let image_size = desc.width * desc.height * NUM_CHANNELS;
        let mut buffer = vec![0.0_f64; image_size];

        buffer
            .as_mut_slice()
            .chunks_mut(NUM_CHANNELS)
            .for_each(|chunk| {
                chunk[0] = desc.background_color.r;
                chunk[1] = desc.background_color.g;
                chunk[2] = desc.background_color.b;
                chunk[3] = desc.background_color.a;
            });

        Canvas {
            buffer,
            accumulation_buffer: vec![AccumulationCell { area: 0.0_f64 }; desc.width * desc.height],
            desc,
        }
    }

    pub fn to_u8(&self) -> Vec<u8> {
        return self
            .buffer
            .iter()
            .map(|value| {
                // https://stackoverflow.com/a/56842762/8622014
                const FACTOR: f64 = (u8::MAX as f64) - f64::EPSILON * 128_f64;

                (*value * FACTOR) as u8
            })
            .collect::<Vec<u8>>();
    }

    fn _buffer_set_at(&mut self, x: i64, y: i64, color: &Color) {
        let xu = x as usize;
        let yu = y as usize;
        let starting: usize = yu * self.desc.width * NUM_CHANNELS + xu * NUM_CHANNELS;

        self.buffer[starting] = color.r;
        self.buffer[starting + 1] = color.g;
        self.buffer[starting + 2] = color.b;
        self.buffer[starting + 3] = color.a;
    }

    fn buffer_set_at_usize(&mut self, x: usize, y: usize, color: &Color) {
        let starting: usize = y * self.desc.width * NUM_CHANNELS + x * NUM_CHANNELS;

        self.buffer[starting] = color.r;
        self.buffer[starting + 1] = color.g;
        self.buffer[starting + 2] = color.b;
        self.buffer[starting + 3] = color.a;
    }

    fn _buffer_get_at(&self, x: i64, y: i64) -> Color {
        let xu = x as usize;
        let yu = y as usize;
        let starting: usize = yu * self.desc.width * NUM_CHANNELS + xu * NUM_CHANNELS;

        Color {
            r: self.buffer[starting],
            g: self.buffer[starting + 1],
            b: self.buffer[starting + 2],
            a: self.buffer[starting + 3],
        }
    }

    fn buffer_get_at_usize(&self, x: usize, y: usize) -> Color {
        let starting: usize = y * self.desc.width * NUM_CHANNELS + x * NUM_CHANNELS;

        Color {
            r: self.buffer[starting],
            g: self.buffer[starting + 1],
            b: self.buffer[starting + 2],
            a: self.buffer[starting + 3],
        }
    }

    fn _accumulation_buffer_set_at(&mut self, x: i64, y: i64, _area: i64) {
        let _xu = x as usize;
        let _yu = y as usize;

        // self.accumulation_buffer[yu * self.desc.width + xu].area += area as u8;
    }

    ///
    /// # Note
    ///
    /// Expects `start` and `end` to be lexicographically sorted (by `x` and `y`).
    ///
    /// Bresenham's line drawing algorithm, adapted from here:
    /// - https://github.com/ssloy/tinyrenderer/wiki/Lesson-1:-Bresenham%E2%80%99s-Line-Drawing-Algorithm
    ///
    /// Line drawing algorithm taken from here:
    /// - https://medium.com/@raphlinus/inside-the-fastest-font-renderer-in-the-world-75ae5270c445
    ///
    fn draw_line(&mut self, start: &Point, end: &Point) {
        let update = |area: f64, cell: &mut AccumulationCell| {
            cell.area += area;
        };
        let p0 = start;
        let p1 = end;

        if (p0.y - p1.y).abs() <= f64::EPSILON {
            return;
        }
        let (dir, p0, p1) = if p0.y < p1.y {
            (1.0, p0, p1)
        } else {
            (-1.0, p1, p0)
        };
        let dxdy = (p1.x - p0.x) / (p1.y - p0.y);
        let mut x = p0.x;
        let y0 = p0.y as usize;

        for y in y0..self.desc.height.min(p1.y.ceil() as usize) {
            let linestart = y * self.desc.width;
            let dy = ((y + 1) as f64).min(p1.y) - (y as f64).max(p0.y);
            let xnext = x + dxdy * dy;
            let d = dy * dir;
            let (x0, x1) = if x < xnext { (x, xnext) } else { (xnext, x) };
            let x0floor = x0.floor();
            let x0i = x0floor as i32;
            let x1ceil = x1.ceil();
            let x1i = x1ceil as i32;
            if x1i <= x0i + 1 {
                let xmf = 0.5 * (x + xnext) - x0floor;
                let linestart_x0i = linestart as isize + x0i as isize;
                if linestart_x0i < 0 {
                    continue; // oob index
                }
                // self.accumulation_buffer[linestart_x0i as usize].area += d - d * xmf;
                // self.accumulation_buffer[linestart_x0i as usize + 1].area += d * xmf;
                update(
                    d - d * xmf,
                    &mut self.accumulation_buffer[linestart_x0i as usize],
                );
                update(
                    d * xmf,
                    &mut self.accumulation_buffer[linestart_x0i as usize + 1],
                );
            } else {
                let s = (x1 - x0).recip();
                let x0f = x0 - x0floor;
                let a0 = 0.5 * s * (1.0 - x0f) * (1.0 - x0f);
                let x1f = x1 - x1ceil + 1.0;
                let am = 0.5 * s * x1f * x1f;
                let linestart_x0i = linestart as isize + x0i as isize;
                if linestart_x0i < 0 {
                    continue; // oob index
                }
                // self.accumulation_buffer[linestart_x0i as usize].area += d * a0;
                update(
                    d * a0,
                    &mut self.accumulation_buffer[linestart_x0i as usize],
                );

                if x1i == x0i + 2 {
                    /*
                    self.accumulation_buffer[linestart_x0i as usize + 1].area +=
                        d * (1.0 - a0 - am);
                        */
                    update(
                        d * (1.0 - a0 - am),
                        &mut self.accumulation_buffer[linestart_x0i as usize + 1],
                    );
                } else {
                    let a1 = s * (1.5 - x0f);
                    // self.accumulation_buffer[linestart_x0i as usize + 1].area += d * (a1 - a0);
                    update(
                        d * (a1 - a0),
                        &mut self.accumulation_buffer[linestart_x0i as usize + 1],
                    );

                    for xi in x0i + 2..x1i - 1 {
                        // self.accumulation_buffer[linestart + xi as usize].area += d * s;
                        update(
                            d * s,
                            &mut self.accumulation_buffer[linestart + xi as usize],
                        );
                    }
                    let a2 = a1 + (x1i - x0i - 3) as f64 * s;
                    /*
                    self.accumulation_buffer[linestart + (x1i - 1) as usize].area +=
                        d * (1.0 - a2 - am);
                    */
                    update(
                        d * (1.0 - a2 - am),
                        &mut self.accumulation_buffer[linestart + (x1i - 1) as usize],
                    );
                }
                // self.accumulation_buffer[linestart + x1i as usize].area += d * am;
                update(
                    d * am,
                    &mut self.accumulation_buffer[linestart + x1i as usize],
                );
            }
            x = xnext;
        }
    }

    pub fn draw_shape(&mut self, path: Path, fill_style: FillStyle, _fill_rule: FillRule) {
        let (mut min_x, mut min_y) = (usize::MAX, usize::MAX);
        let (mut max_x, mut max_y) = (usize::MIN, usize::MIN);

        let mut update_bounds = |x: f64, y: f64| {
            let x = x as usize;
            let y = y as usize;

            min_x = usize::min(min_x, x);
            min_y = usize::min(min_y, y);

            max_x = usize::max(max_x, x);
            max_y = usize::max(max_y, y);
        };

        let mut start_point = Point {
            x: 0.0_f64,
            y: 0.0_f64,
        };
        let mut currently_at = Point {
            x: 0.0_f64,
            y: 0.0_f64,
        };

        for op in path.iter() {
            match op {
                PathOps::MoveTo { x, y } => {
                    currently_at.x = *x;
                    currently_at.y = *y;

                    start_point.x = *x;
                    start_point.y = *y;

                    update_bounds(*x, *y);
                }
                PathOps::MoveToRel { x, y } => {
                    currently_at.x += *x;
                    currently_at.y += *y;

                    start_point.x = currently_at.x;
                    start_point.y = currently_at.y;

                    update_bounds(currently_at.x, currently_at.y);
                }
                PathOps::LineTo { x, y } => {
                    self.draw_line(&currently_at, &Point { x: *x, y: *y });

                    currently_at.x = *x;
                    currently_at.y = *y;

                    update_bounds(*x, *y);
                }
                PathOps::LineToRel { x, y } => {
                    self.draw_line(
                        &currently_at,
                        &Point {
                            x: currently_at.x + *x,
                            y: currently_at.y + *y,
                        },
                    );

                    currently_at.x += *x;
                    currently_at.y += *y;

                    update_bounds(currently_at.x, currently_at.y);
                }
                PathOps::Close => {
                    self.draw_line(&currently_at, &start_point);

                    currently_at.x = start_point.x;
                    currently_at.y = start_point.y;
                }
            }
        }

        for y in min_y..=max_y {
            let mut acc = 0.0_f64;

            for x in min_x..max_x {
                let cell = &mut self.accumulation_buffer[y * self.desc.width + x];
                let area = cell.area;
                acc += area;
                cell.area = 0.0_f64;

                let dest = self.buffer_get_at_usize(x, y);
                let src = match fill_style {
                    FillStyle::Plain(Color { r, g, b, a: _ }) => Color {
                        r,
                        g,
                        b,
                        a: acc.abs(),
                    },
                };

                self.buffer_set_at_usize(
                    x,
                    y,
                    &Color {
                        r: src.r * src.a + dest.r * (1.0_f64 - src.a),
                        g: src.g * src.a + dest.g * (1.0_f64 - src.a),
                        b: src.b * src.a + dest.b * (1.0_f64 - src.a),
                        a: dest.a,
                    }
                    .clamp(),
                );
            }
        }
    }
}
