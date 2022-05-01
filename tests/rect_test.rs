// This test draws a rectangle that's not filled inside.

use crate::common::default_blending;
use verg::canvas::{Canvas, CanvasDescription};
use verg::color::{Color, FillRule, FillStyle};
use verg::geometry::PathOps;

mod common;

const WIDTH: usize = 500;
const HEIGHT: usize = 500;

fn canvas_description() -> CanvasDescription {
    CanvasDescription {
        width: WIDTH,
        height: HEIGHT,
        background_color: Color::black(),
        ..Default::default()
    }
}

const PATH: [PathOps; 15] = [
    PathOps::MoveTo { x: 80.0, y: 80.0 },
    PathOps::LineTo { x: 80.0, y: 420.0 },
    PathOps::LineTo { x: 420.0, y: 420.0 },
    PathOps::LineTo {
        x: 420.0_f64,
        y: 80.0_f64,
    },
    PathOps::Close,
    PathOps::MoveTo { x: 10.0, y: 10.0 },
    PathOps::LineTo { x: 490.0, y: 10.0 },
    PathOps::LineTo { x: 490.0, y: 490.0 },
    PathOps::LineTo { x: 10.0, y: 490.0 },
    PathOps::Close,
    PathOps::MoveTo { x: 300.0, y: 200.0 },
    PathOps::LineTo { x: 300.0, y: 300.0 },
    PathOps::LineTo { x: 200.0, y: 300.0 },
    PathOps::LineTo { x: 200.0, y: 200.0 },
    PathOps::Close,
];

const FILL_STYLE: FillStyle = FillStyle::Plain(Color::white());

const FILL_RULE: FillRule = FillRule::NonZero;

implement_test! {
    rect_test, canvas_description |
    PATH, FILL_STYLE, FILL_RULE, default_blending
}
