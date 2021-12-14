use crate::color::{Color, FillRule, FillStyle};
use crate::geometry::{Point, Primitive, Shape};
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
    pub num_channels: u8,
    pub background_color: Color,
}

impl Default for CanvasDescription {
    fn default() -> Self {
        CanvasDescription {
            width: 600,
            height: 600,
            pixel_size: 256,
            num_channels: 4,
            background_color: Color::default(),
        }
    }
}

#[derive(Debug)]
pub struct Canvas {
    buffer: Vec<f64>,
    accumulation_buffer: Vec<AccumulationCell>,
    pub desc: CanvasDescription,
}

impl Canvas {
    pub fn new(desc: CanvasDescription) -> Canvas {
        let image_size = desc.width * desc.height * (desc.num_channels as usize);
        let mut buffer = vec![0.0_f64; image_size];

        buffer
            .as_mut_slice()
            .chunks_mut(desc.num_channels as usize)
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
                return (*value * FACTOR) as u8;
            })
            .collect::<Vec<u8>>();
    }

    ///
    /// # Note
    ///
    /// Expects `start` and `end` to be lexicographically sorted (by `x` and `y`).
    ///
    fn draw_line(&mut self, start: &Point, end: &Point) {
        let start = start.round_to_grid(self.desc.pixel_size);
        let end = end.round_to_grid(self.desc.pixel_size);
    }

    pub fn draw_shape(&mut self, shape: Shape, _fill_style: FillStyle, _fill_rule: FillRule) {
        for primitive in shape.iter() {
            match primitive {
                Primitive::Line { start, end } => {
                    self.draw_line(start, end);
                }
            }
        }
    }
}
