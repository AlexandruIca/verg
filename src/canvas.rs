use crate::color::{Color, FillRule, FillStyle};
use crate::geometry::{intersect_line_with_grid, GridPoint, Path, PathOps, Point};
use std::vec::Vec;

#[derive(Debug, Clone)]
struct AccumulationCell {
    cover: u8,
    area: u8,
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
            accumulation_buffer: vec![
                AccumulationCell { cover: 0, area: 0 };
                desc.width * desc.height
            ],
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

    fn buffer_set_at(&mut self, x: i64, y: i64, color: &Color) {
        let xu = x as usize;
        let yu = y as usize;
        let starting: usize = yu * self.desc.width * NUM_CHANNELS + xu * NUM_CHANNELS;

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

    ///
    /// # Note
    ///
    /// Expects `start` and `end` to be lexicographically sorted (by `x` and `y`).
    ///
    fn draw_line(&mut self, start: &Point, end: &Point) {
        let start = start.round_to_grid(self.desc.pixel_size);
        let end = end.round_to_grid(self.desc.pixel_size);
        let points = intersect_line_with_grid(&start, &end, &self.desc);

        let first = points.first().unwrap();
        self.buffer_set_at(first.x.0, first.y.0, &Color::default());

        points.windows(2).for_each(|p: &[GridPoint]| {
            let a = &p[0];
            let b = &p[1];

            if (a.x.0 - b.x.0).abs() > 1 || (a.y.0 - b.y.0).abs() > 1 {
                let (x, y) = (((a.x.0 + b.x.0) / 2), ((a.y.0 + b.y.0) / 2));
                self.buffer_set_at(x, y, &Color::default());
            }

            self.buffer_set_at(b.x.0, b.y.0, &Color::default());
        });
    }

    pub fn draw_shape(&mut self, path: Path, _fill_style: FillStyle, _fill_rule: FillRule) {
        let mut currently_at = Point {
            x: 0.0_f64,
            y: 0.0_f64,
        };

        for op in path.iter() {
            match op {
                PathOps::MoveTo { x, y } => {
                    currently_at.x = *x;
                    currently_at.y = *y;
                }
                PathOps::MoveToRel { x, y } => {
                    currently_at.x += *x;
                    currently_at.y += *y;
                }
                PathOps::LineTo { x, y } => {
                    self.draw_line(&currently_at, &Point { x: *x, y: *y });
                }
                PathOps::LineToRel { x, y } => {
                    self.draw_line(
                        &currently_at,
                        &Point {
                            x: currently_at.x + *x,
                            y: currently_at.y + *y,
                        },
                    );
                }
            }
        }
    }
}
