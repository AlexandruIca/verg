// This test draws some open paths using only lines.

use crate::common::{default_blending, default_callback};
use verg::{
    canvas::{Canvas, CanvasDescription, ViewBox},
    color::{Color, FillRule, FillStyle},
    geometry::{PathOps, Point},
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
        background_color: Color::white(),
        ..Default::default()
    }
}

const LINES: [[PathOps; 2]; 6] = [
    [
        PathOps::MoveTo { x: 120.0, y: 120.0 },
        PathOps::LineTo { x: 360.0, y: 360.0 },
    ],
    [
        PathOps::MoveTo { x: 380.0, y: 80.0 },
        PathOps::LineTo { x: 200.0, y: 60.0 },
    ],
    [
        PathOps::MoveTo { x: 200.0, y: 115.0 },
        PathOps::LineTo { x: 380.0, y: 120.0 },
    ],
    [
        PathOps::MoveTo { x: 60.0, y: 20.0 },
        PathOps::LineTo { x: 60.0, y: 300.0 },
    ],
    [
        PathOps::MoveTo { x: 60.0, y: 350.0 },
        PathOps::LineTo { x: 360.0, y: 350.0 },
    ],
    [
        PathOps::MoveTo { x: 140.0, y: 400.0 },
        PathOps::LineTo { x: 141.0, y: 490.0 },
    ],
];

const FILL_STYLE: FillStyle = FillStyle::Plain(Color::red());

const FILL_RULE: FillRule = FillRule::NonZero;

implement_test! {
    line_test, canvas_description, default_callback |
    LINES[0], FILL_STYLE, FILL_RULE, default_blending,
    LINES[1], FILL_STYLE, FILL_RULE, default_blending,
    LINES[2], FILL_STYLE, FILL_RULE, default_blending,
    LINES[3], FILL_STYLE, FILL_RULE, default_blending,
    LINES[4], FILL_STYLE, FILL_RULE, default_blending,
    LINES[5], FILL_STYLE, FILL_RULE, default_blending
}
