// This test draws a bunch of triangles of different sizes, colors and shapes.

use crate::common::default_blending;
use verg::canvas::{Canvas, CanvasDescription, ViewBox};
use verg::color::{Color, FillRule, FillStyle};
use verg::geometry::PathOps;

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

const BIG_TRIANGLE: [PathOps; 4] = [
    PathOps::MoveTo { x: 60.0, y: 240.0 },
    PathOps::LineTo { x: 360.0, y: 80.0 },
    PathOps::LineTo { x: 400.0, y: 280.0 },
    PathOps::Close,
];

const SMALL_WHITE_TRIANGLE: [PathOps; 4] = [
    PathOps::MoveTo { x: 100.0, y: 150.0 },
    PathOps::LineTo { x: 100.0, y: 110.0 },
    PathOps::LineTo { x: 150.0, y: 110.0 },
    PathOps::Close,
];

const FILL_WHITE: FillStyle = FillStyle::Plain(Color::white());

const RED_TRIANGLES: [PathOps; 8] = [
    PathOps::MoveTo { x: 100.0, y: 50.0 },
    PathOps::LineTo { x: 100.0, y: 10.0 },
    PathOps::LineTo { x: 150.0, y: 10.0 },
    PathOps::Close,
    PathOps::MoveTo { x: 250.0, y: 50.0 },
    PathOps::LineTo { x: 300.0, y: 50.0 },
    PathOps::LineTo { x: 300.0, y: 10.0 },
    PathOps::Close,
];

const FILL_RED: FillStyle = FillStyle::Plain(Color::crimson());

const BLUE_RECT: [PathOps; 8] = [
    PathOps::MoveTo { x: 10.0, y: 10.0 },
    PathOps::LineTo { x: 10.0, y: 50.0 },
    PathOps::LineTo { x: 50.0, y: 10.0 },
    PathOps::Close,
    PathOps::MoveTo { x: 10.0, y: 50.0 },
    PathOps::LineTo { x: 50.0, y: 50.0 },
    PathOps::LineTo { x: 50.0, y: 10.0 },
    PathOps::Close,
];

const FILL_BLUE: FillStyle = FillStyle::Plain(Color::blue());

const IMPERFECT_RECT_PART1: [PathOps; 4] = [
    PathOps::MoveTo { x: 400.0, y: 10.0 },
    PathOps::LineTo { x: 400.0, y: 50.0 },
    PathOps::LineTo { x: 450.0, y: 10.0 },
    PathOps::Close,
];

const IMPERFECT_RECT_PART2: [PathOps; 4] = [
    PathOps::MoveTo { x: 400.0, y: 50.0 },
    PathOps::LineTo { x: 450.0, y: 50.0 },
    PathOps::LineTo { x: 450.0, y: 10.0 },
    PathOps::Close,
];

const FILL_CYAN: FillStyle = FillStyle::Plain(Color::cyan());

const DIAMOND: [PathOps; 8] = [
    PathOps::MoveTo { x: 50.0, y: 350.0 },
    PathOps::LineTo { x: 100.0, y: 300.0 },
    PathOps::LineTo { x: 100.0, y: 400.0 },
    PathOps::Close,
    PathOps::MoveTo { x: 150.0, y: 350.0 },
    PathOps::LineTo { x: 100.0, y: 300.0 },
    PathOps::LineTo { x: 100.0, y: 400.0 },
    PathOps::Close,
];

const FILL_DIAMOND: FillStyle = FillStyle::Plain(Color::forest_green());

const SMALL_PURPLE_TRIANGLE: [PathOps; 4] = [
    PathOps::MoveTo { x: 300.0, y: 400.0 },
    PathOps::LineToRel { x: 50.0, y: -50.0 },
    PathOps::LineToRel { x: -10.0, y: 51.0 },
    PathOps::Close,
];

const FILL_PURPLE: FillStyle = FillStyle::Plain(Color::dark_slate_blue());

const FILL_RULE: FillRule = FillRule::NonZero;

implement_test! {
    triangle_test, canvas_description |
    BIG_TRIANGLE,               FILL_WHITE,    FILL_RULE, default_blending,
    SMALL_WHITE_TRIANGLE,       FILL_WHITE,    FILL_RULE, default_blending,
    RED_TRIANGLES,              FILL_RED,      FILL_RULE, default_blending,
    BLUE_RECT,                  FILL_BLUE,     FILL_RULE, default_blending,
    IMPERFECT_RECT_PART1,       FILL_CYAN,     FILL_RULE, default_blending,
    IMPERFECT_RECT_PART2,       FILL_CYAN,     FILL_RULE, default_blending,
    DIAMOND,                    FILL_DIAMOND,  FILL_RULE, default_blending,
    SMALL_PURPLE_TRIANGLE,      FILL_PURPLE,   FILL_RULE, default_blending
}
