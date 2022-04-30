// This test draws a rectangle that's not filled inside.

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
        background_color: Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        },
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

const FILL_STYLE: FillStyle = FillStyle::Plain(Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
    a: 1.0,
});

const FILL_RULE: FillRule = FillRule::NonZero;

implement_test! { rect_test, canvas_description | PATH, FILL_STYLE, FILL_RULE }
