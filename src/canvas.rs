use crate::color::{clamp, Color, FillRule, FillStyle};
use crate::geometry::{Path, PathOps, Point};
use crate::renderer::draw_line;
use std::vec::Vec;

#[derive(Debug, Clone, Copy)]
pub struct AccumulationCell {
    pub area: f32,
    pub id: i32,
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
                AccumulationCell { area: 0.0, id: 0 };
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

    fn buffer_set_at(&mut self, x: usize, y: usize, color: &Color) {
        let starting: usize = y * self.desc.width * NUM_CHANNELS + x * NUM_CHANNELS;

        self.buffer[starting] = color.r;
        self.buffer[starting + 1] = color.g;
        self.buffer[starting + 2] = color.b;
        self.buffer[starting + 3] = color.a;
    }

    fn buffer_get_at(&self, x: usize, y: usize) -> Color {
        let starting: usize = y * self.desc.width * NUM_CHANNELS + x * NUM_CHANNELS;

        Color {
            r: self.buffer[starting],
            g: self.buffer[starting + 1],
            b: self.buffer[starting + 2],
            a: self.buffer[starting + 3],
        }
    }

    pub fn draw_shape(&mut self, path: Path, fill_style: FillStyle, fill_rule: FillRule) {
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

        let mut line_id = 0;

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
                    line_id += 1;
                    draw_line(
                        &mut self.accumulation_buffer,
                        &self.desc,
                        &currently_at,
                        &Point { x: *x, y: *y },
                        line_id,
                    );

                    currently_at.x = *x;
                    currently_at.y = *y;

                    update_bounds(*x, *y);
                }
                PathOps::LineToRel { x, y } => {
                    line_id += 1;
                    draw_line(
                        &mut self.accumulation_buffer,
                        &self.desc,
                        &currently_at,
                        &Point {
                            x: currently_at.x + *x,
                            y: currently_at.y + *y,
                        },
                        line_id,
                    );

                    currently_at.x += *x;
                    currently_at.y += *y;

                    update_bounds(currently_at.x, currently_at.y);
                }
                PathOps::Close => {
                    line_id += 1;
                    draw_line(
                        &mut self.accumulation_buffer,
                        &self.desc,
                        &currently_at,
                        &start_point,
                        line_id,
                    );

                    currently_at.x = start_point.x;
                    currently_at.y = start_point.y;
                }
            }
        }

        for y in min_y..=max_y {
            let mut acc = 0.0_f32;
            let mut filling = -1.0_f32;
            let mut prev_cell = AccumulationCell { area: 0.0, id: 0 };

            for x in min_x..max_x {
                let cell = &mut self.accumulation_buffer[y * self.desc.width + x];
                let area = cell.area;

                let alpha = match fill_rule {
                    FillRule::EvenOdd => {
                        if cell.id > 0 && cell.id != prev_cell.id {
                            prev_cell.id = cell.id;
                            filling = -filling;
                        }

                        if cell.id == prev_cell.id {
                            acc += filling * area.abs();

                            if acc < 0.0 || acc > 1.0 {
                                acc = (!(filling > 0.0) as i32) as f32;
                            }
                        } else {
                            acc = ((filling > 0.0) as i32) as f32;
                        }

                        clamp(acc.abs(), 0.0, 1.0)
                    }
                    FillRule::NonZero => {
                        acc += area;
                        acc.abs()
                    }
                };

                cell.area = 0.0_f32;

                let dest = self.buffer_get_at(x, y);
                let src = match fill_style {
                    FillStyle::Plain(Color { r, g, b, a: _ }) => Color {
                        r,
                        g,
                        b,
                        a: alpha as f64,
                    },
                };

                self.buffer_set_at(
                    x,
                    y,
                    &Color {
                        r: src.r * src.a + dest.r * (1.0 - src.a),
                        g: src.g * src.a + dest.g * (1.0 - src.a),
                        b: src.b * src.a + dest.b * (1.0 - src.a),
                        a: dest.a,
                    }
                    .clamp(),
                );
            }
        }
    }
}
