// This tests draws a triangle inside a square using the even-odd fill rule.

use crate::common::{default_blending, default_callback};
use verg::{
    canvas::{Canvas, CanvasDescription, ViewBox},
    color::{Color, FillRule, FillStyle},
    geometry::{PathOps, Point},
};

mod common;

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;

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

const PATH: [PathOps; 18] = [
    PathOps::MoveTo { x: 200.0, y: 200.0 },
    PathOps::LineTo { x: 800.0, y: 200.0 },
    PathOps::LineTo { x: 800.0, y: 800.0 },
    PathOps::LineTo { x: 200.0, y: 800.0 },
    PathOps::Close,
    PathOps::MoveTo { x: 300.0, y: 300.0 },
    PathOps::LineTo { x: 700.0, y: 300.0 },
    PathOps::LineTo { x: 700.0, y: 700.0 },
    PathOps::LineTo { x: 300.0, y: 700.0 },
    PathOps::Close,
    PathOps::MoveTo { x: 450.0, y: 500.0 },
    PathOps::LineTo { x: 550.0, y: 450.0 },
    PathOps::LineTo { x: 550.0, y: 550.0 },
    PathOps::Close,
    PathOps::MoveTo { x: 460.0, y: 500.0 },
    PathOps::LineTo { x: 540.0, y: 480.0 },
    PathOps::LineTo { x: 540.0, y: 540.0 },
    PathOps::Close,
];

const FILL_STYLE: FillStyle = FillStyle::Plain(Color::white());

const FILL_RULE: FillRule = FillRule::EvenOdd;

implement_test! {
    even_odd_fill_test, canvas_description, default_callback |
    PATH, FILL_STYLE, FILL_RULE, default_blending
}
