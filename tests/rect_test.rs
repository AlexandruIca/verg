// This test draws a rectangle that's not filled inside.

use crate::common::{default_blending, default_callback};
use verg::canvas::{Canvas, CanvasDescription, ViewBox};
use verg::color::{Color, FillRule, FillStyle};
use verg::geometry::{PathOps, Point};

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
        background_color: Color::black(),
        ..Default::default()
    }
}

const PATH: [PathOps; 15] = [
    PathOps::MoveTo { x: 80.0, y: 80.0 },
    PathOps::LineTo { x: 80.0, y: 420.0 },
    PathOps::LineTo { x: 420.0, y: 420.0 },
    PathOps::LineTo { x: 420.0, y: 80.0 },
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
    rect_test, canvas_description, default_callback |
    PATH, FILL_STYLE, FILL_RULE, default_blending
}
