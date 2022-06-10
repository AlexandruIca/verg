// This test draw quadratic Bézier curves.

use verg::{
    canvas::{Canvas, CanvasDescription, ViewBox},
    color::{Color, FillRule, FillStyle},
    geometry::{PathOps, Point},
};

mod common;

const WIDTH: usize = 200;
const HEIGHT: usize = 200;

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
        tolerance: 0.5,
    }
}

const PATH: [PathOps; 2] = [
    PathOps::MoveTo { x: 20.0, y: 180.0 },
    PathOps::QuadTo {
        x1: 180.0,
        y1: 180.0,
        x2: 180.0,
        y2: 20.0,
    },
];

const BLACK: FillStyle = FillStyle::Plain(Color::black());

const FILL_RULE: FillRule = FillRule::NonZero;

fn callback(canvas: &mut Canvas) {
    canvas.draw_shape(&PATH, BLACK, FILL_RULE, |p| *p);
}

implement_test! {
    quadbezier_test, canvas_description, callback |
}
