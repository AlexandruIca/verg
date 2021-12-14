use crate::color::{FillRule, FillStyle};
use crate::geometry::{Primitive, Shape};
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
}

impl Default for CanvasDescription {
    fn default() -> Self {
        CanvasDescription {
            width: 600,
            height: 600,
            pixel_size: 256,
            num_channels: 4,
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

        Canvas {
            buffer: vec![0.0f64; image_size],
            accumulation_buffer: vec![
                AccumulationCell { cover: 0, area: 0 };
                desc.width * desc.height
            ],
            desc,
        }
    }

    pub fn to_u8(&self) -> Vec<u8> {
        let mut result = vec![0u8; self.buffer.len()];

        self.buffer.iter().enumerate().for_each(|(index, value)| {
            // https://stackoverflow.com/a/56842762/8622014
            const FACTOR: f64 = (u8::MAX as f64) - f64::EPSILON * 128.0f64;
            result[index] = (*value * FACTOR) as u8;
        });

        return result;
    }

    pub fn draw_shape(&mut self, shape: Shape, _fill_style: FillStyle, _fill_rule: FillRule) {
        for primitive in shape.iter() {
            match primitive {
                Primitive::Line { start: _, end: _ } => {
                    unimplemented!();
                }
            }
        }
    }
}
