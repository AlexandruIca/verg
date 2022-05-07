// This test draws a background on the whole canvas.

use crate::common::default_blending;
use verg::{
    canvas::{Canvas, CanvasDescription, ViewBox},
    color::{Color, FillRule, FillStyle},
    geometry::PathOps,
};

mod common;

const WIDTH: usize = 500;
const HEIGHT: usize = 500;

fn canvas_description() -> CanvasDescription {
    CanvasDescription {
        width: WIDTH,
        height: HEIGHT,
        viewbox: ViewBox {
            x: 0.0,
            y: 0.0,
            width: WIDTH as f64,
            height: HEIGHT as f64,
        },
        background_color: Color::coral(),
        ..Default::default()
    }
}

const PATH: [PathOps; 5] = [
    PathOps::MoveTo { x: 0.0, y: 0.0 },
    PathOps::LineTo {
        x: WIDTH as f64 - 1.0,
        y: 0.0,
    },
    PathOps::LineTo {
        x: WIDTH as f64 - 1.0,
        y: HEIGHT as f64 - 1.0,
    },
    PathOps::LineTo {
        x: 0.0,
        y: HEIGHT as f64 - 1.0,
    },
    PathOps::Close,
];

static FILL_STYLE: FillStyle = FillStyle::Plain(Color::dark_slate_blue());

const FILL_RULE: FillRule = FillRule::NonZero;

implement_test! {
    basic_test, canvas_description |
    PATH, FILL_STYLE, FILL_RULE, default_blending
}
