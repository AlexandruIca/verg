use crate::color::{Color, FillRule, FillStyle};
use crate::geometry::{Path, Point};
use crate::renderer::{blend_func, fill_path, render_path, BlendFunc, RenderState, NUM_CHANNELS};
use std::vec::Vec;

#[derive(Debug, Clone, Copy)]
pub struct AccumulationCell {
    pub area: f32,
    pub id: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct ViewBox {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct CanvasDescription {
    pub width: usize,
    pub height: usize,
    pub viewbox: ViewBox,
    pub tolerance: f64,
    pub background_color: Color,
}

impl Default for CanvasDescription {
    fn default() -> Self {
        CanvasDescription {
            width: 600,
            height: 600,
            viewbox: ViewBox {
                x: 0.0,
                y: 0.0,
                width: 600.0,
                height: 600.0,
            },
            tolerance: 1.5,
            background_color: Color::default(),
        }
    }
}

pub struct Canvas {
    pub buffer: Vec<f64>,
    pub accumulation_buffer: Vec<AccumulationCell>,
    pub desc: CanvasDescription,
    pub blend: BlendFunc,
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
            blend: blend_func::source_over,
        }
    }

    pub fn set_blending_function(&mut self, f: BlendFunc) {
        self.blend = f;
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

    pub fn draw_shape(
        &mut self,
        path: Path,
        fill_style: FillStyle,
        fill_rule: FillRule,
        transform: impl Fn(&Point) -> Point,
    ) {
        let mut state = RenderState {
            canvas: self,
            id: 0,
        };
        let bounds = render_path(&mut state, path, transform);
        fill_path(&mut state, fill_style, fill_rule, &bounds)
    }
}
