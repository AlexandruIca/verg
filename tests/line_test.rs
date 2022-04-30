// This test draws some open paths using only lines.

use verg::{
    canvas::{Canvas, CanvasDescription},
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
    line_test, canvas_description |
    LINES[0], FILL_STYLE, FILL_RULE,
    LINES[1], FILL_STYLE, FILL_RULE,
    LINES[2], FILL_STYLE, FILL_RULE,
    LINES[3], FILL_STYLE, FILL_RULE,
    LINES[4], FILL_STYLE, FILL_RULE,
    LINES[5], FILL_STYLE, FILL_RULE
}
