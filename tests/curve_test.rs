// This test draw quadratic Bézier curves.

use crate::common::{default_blending, default_callback};
use verg::{
    canvas::{Canvas, CanvasDescription, ViewBox},
    color::{Color, FillRule, FillStyle},
    geometry::{PathOps, Point},
};

mod common;

const WIDTH: usize = 205;
const HEIGHT: usize = 405;

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
        tolerance: 0.25,
    }
}

const PATH: [PathOps; 3] = [
    PathOps::MoveTo { x: 0.0, y: 0.0 },
    PathOps::QuadTo {
        x1: 200.0,
        y1: 200.0,
        x2: 0.0,
        y2: 400.0,
    },
    PathOps::Close,
];

const FILL_STYLE: FillStyle = FillStyle::Plain(Color::black());

const FILL_RULE: FillRule = FillRule::NonZero;

implement_test! {
    curve_test, canvas_description, default_callback |
    PATH, FILL_STYLE, FILL_RULE, default_blending
}
