// This tests the Porter-Duff blending operators.

use crate::common::default_blending;
use verg::{
    canvas::{Canvas, CanvasDescription, ViewBox},
    color::{Color, FillRule, FillStyle},
    geometry::PathOps,
    renderer::blend_func,
};

mod common;

const WIDTH: usize = 1400;
const HEIGHT: usize = 1020;

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
        background_color: Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
        },
        ..Default::default()
    }
}

const DESTINATION_TRIANGLES: [[PathOps; 4]; 12] = [
    [
        PathOps::MoveTo { x: 0.0, y: 20.0 },
        PathOps::LineTo { x: 200.0, y: 20.0 },
        PathOps::LineTo { x: 100.0, y: 220.0 },
        PathOps::Close,
    ],
    [
        PathOps::MoveTo { x: 400.0, y: 20.0 },
        PathOps::LineTo { x: 600.0, y: 20.0 },
        PathOps::LineTo { x: 500.0, y: 220.0 },
        PathOps::Close,
    ],
    [
        PathOps::MoveTo { x: 0.0, y: 420.0 },
        PathOps::LineTo { x: 200.0, y: 420.0 },
        PathOps::LineTo { x: 100.0, y: 620.0 },
        PathOps::Close,
    ],
    [
        PathOps::MoveTo { x: 400.0, y: 420.0 },
        PathOps::LineTo { x: 600.0, y: 420.0 },
        PathOps::LineTo { x: 500.0, y: 620.0 },
        PathOps::Close,
    ],
    [
        PathOps::MoveTo { x: 0.0, y: 820.0 },
        PathOps::LineTo { x: 200.0, y: 820.0 },
        PathOps::LineTo {
            x: 100.0,
            y: 1020.0,
        },
        PathOps::Close,
    ],
    [
        PathOps::MoveTo { x: 400.0, y: 820.0 },
        PathOps::LineTo { x: 600.0, y: 820.0 },
        PathOps::LineTo {
            x: 500.0,
            y: 1020.0,
        },
        PathOps::Close,
    ],
    [
        PathOps::MoveTo { x: 800.0, y: 20.0 },
        PathOps::LineTo { x: 1000.0, y: 20.0 },
        PathOps::LineTo { x: 900.0, y: 220.0 },
        PathOps::Close,
    ],
    [
        PathOps::MoveTo { x: 1200.0, y: 20.0 },
        PathOps::LineTo { x: 1400.0, y: 20.0 },
        PathOps::LineTo {
            x: 1300.0,
            y: 220.0,
        },
        PathOps::Close,
    ],
    [
        PathOps::MoveTo { x: 800.0, y: 420.0 },
        PathOps::LineTo {
            x: 1000.0,
            y: 420.0,
        },
        PathOps::LineTo { x: 900.0, y: 620.0 },
        PathOps::Close,
    ],
    [
        PathOps::MoveTo {
            x: 1200.0,
            y: 420.0,
        },
        PathOps::LineTo {
            x: 1400.0,
            y: 420.0,
        },
        PathOps::LineTo {
            x: 1300.0,
            y: 620.0,
        },
        PathOps::Close,
    ],
    [
        PathOps::MoveTo { x: 800.0, y: 820.0 },
        PathOps::LineTo {
            x: 1000.0,
            y: 820.0,
        },
        PathOps::LineTo {
            x: 900.0,
            y: 1020.0,
        },
        PathOps::Close,
    ],
    [
        PathOps::MoveTo {
            x: 1200.0,
            y: 820.0,
        },
        PathOps::LineTo {
            x: 1400.0,
            y: 820.0,
        },
        PathOps::LineTo {
            x: 1300.0,
            y: 1020.0,
        },
        PathOps::Close,
    ],
];

const SOURCE_TRIANGLES: [[PathOps; 4]; 12] = [
    [
        PathOps::MoveTo { x: 0.0, y: 200.0 },
        PathOps::LineTo { x: 100.0, y: 0.0 },
        PathOps::LineTo { x: 200.0, y: 200.0 },
        PathOps::Close,
    ],
    [
        PathOps::MoveTo { x: 400.0, y: 200.0 },
        PathOps::LineTo { x: 500.0, y: 0.0 },
        PathOps::LineTo { x: 600.0, y: 200.0 },
        PathOps::Close,
    ],
    [
        PathOps::MoveTo { x: 0.0, y: 600.0 },
        PathOps::LineTo { x: 100.0, y: 400.0 },
        PathOps::LineTo { x: 200.0, y: 600.0 },
        PathOps::Close,
    ],
    [
        PathOps::MoveTo { x: 400.0, y: 600.0 },
        PathOps::LineTo { x: 500.0, y: 400.0 },
        PathOps::LineTo { x: 600.0, y: 600.0 },
        PathOps::Close,
    ],
    [
        PathOps::MoveTo { x: 0.0, y: 1000.0 },
        PathOps::LineTo { x: 100.0, y: 800.0 },
        PathOps::LineTo {
            x: 200.0,
            y: 1000.0,
        },
        PathOps::Close,
    ],
    [
        PathOps::MoveTo {
            x: 400.0,
            y: 1000.0,
        },
        PathOps::LineTo { x: 500.0, y: 800.0 },
        PathOps::LineTo {
            x: 600.0,
            y: 1000.0,
        },
        PathOps::Close,
    ],
    [
        PathOps::MoveTo { x: 800.0, y: 200.0 },
        PathOps::LineTo { x: 900.0, y: 0.0 },
        PathOps::LineTo {
            x: 1000.0,
            y: 200.0,
        },
        PathOps::Close,
    ],
    [
        PathOps::MoveTo {
            x: 1200.0,
            y: 200.0,
        },
        PathOps::LineTo { x: 1300.0, y: 0.0 },
        PathOps::LineTo {
            x: 1400.0,
            y: 200.0,
        },
        PathOps::Close,
    ],
    [
        PathOps::MoveTo { x: 800.0, y: 600.0 },
        PathOps::LineTo { x: 900.0, y: 400.0 },
        PathOps::LineTo {
            x: 1000.0,
            y: 600.0,
        },
        PathOps::Close,
    ],
    [
        PathOps::MoveTo {
            x: 1200.0,
            y: 600.0,
        },
        PathOps::LineTo {
            x: 1300.0,
            y: 400.0,
        },
        PathOps::LineTo {
            x: 1400.0,
            y: 600.0,
        },
        PathOps::Close,
    ],
    [
        PathOps::MoveTo {
            x: 800.0,
            y: 1000.0,
        },
        PathOps::LineTo { x: 900.0, y: 800.0 },
        PathOps::LineTo {
            x: 1000.0,
            y: 1000.0,
        },
        PathOps::Close,
    ],
    [
        PathOps::MoveTo {
            x: 1200.0,
            y: 1000.0,
        },
        PathOps::LineTo {
            x: 1300.0,
            y: 800.0,
        },
        PathOps::LineTo {
            x: 1400.0,
            y: 1000.0,
        },
        PathOps::Close,
    ],
];

static FILL_SOURCE: FillStyle = FillStyle::Plain(Color::coral());
static FILL_DESTINATION: FillStyle = FillStyle::Plain(Color::dark_slate_blue());

const FILL_RULE: FillRule = FillRule::NonZero;

#[allow(dead_code)]
fn destination_over(src: &Color, dest: &Color) -> Color {
    blend_func::destination_over(src, dest)
}

#[allow(dead_code)]
fn source_out(src: &Color, dest: &Color) -> Color {
    blend_func::source_out(src, dest)
}

#[allow(dead_code)]
fn destination_out(src: &Color, dest: &Color) -> Color {
    blend_func::destination_out(src, dest)
}

#[allow(dead_code)]
fn source_in(src: &Color, dest: &Color) -> Color {
    blend_func::source_in(src, dest)
}

#[allow(dead_code)]
fn destination_in(src: &Color, dest: &Color) -> Color {
    blend_func::destination_in(src, dest)
}

#[allow(dead_code)]
fn source_atop(src: &Color, dest: &Color) -> Color {
    blend_func::source_atop(src, dest)
}

#[allow(dead_code)]
fn destination_atop(src: &Color, dest: &Color) -> Color {
    blend_func::destination_atop(src, dest)
}

#[allow(dead_code)]
fn xor(src: &Color, dest: &Color) -> Color {
    blend_func::xor(src, dest)
}

#[allow(dead_code)]
fn source(src: &Color, dest: &Color) -> Color {
    blend_func::source(src, dest)
}

#[allow(dead_code)]
fn destination(src: &Color, dest: &Color) -> Color {
    blend_func::destination(src, dest)
}

#[allow(dead_code)]
fn additive(src: &Color, dest: &Color) -> Color {
    blend_func::additive(src, dest)
}

implement_test! {
    alpha_blending_test, canvas_description |
    DESTINATION_TRIANGLES[0],  FILL_DESTINATION, FILL_RULE, default_blending,
    SOURCE_TRIANGLES[0],       FILL_SOURCE,      FILL_RULE, default_blending,
    DESTINATION_TRIANGLES[1],  FILL_DESTINATION, FILL_RULE, default_blending,
    SOURCE_TRIANGLES[1],       FILL_SOURCE,      FILL_RULE, destination_over,
    DESTINATION_TRIANGLES[2],  FILL_DESTINATION, FILL_RULE, default_blending,
    SOURCE_TRIANGLES[2],       FILL_SOURCE,      FILL_RULE, source_out,
    DESTINATION_TRIANGLES[3],  FILL_DESTINATION, FILL_RULE, default_blending,
    SOURCE_TRIANGLES[3],       FILL_SOURCE,      FILL_RULE, destination_out,
    DESTINATION_TRIANGLES[4],  FILL_DESTINATION, FILL_RULE, default_blending,
    SOURCE_TRIANGLES[4],       FILL_SOURCE,      FILL_RULE, source_in,
    DESTINATION_TRIANGLES[5],  FILL_DESTINATION, FILL_RULE, default_blending,
    SOURCE_TRIANGLES[5],       FILL_SOURCE,      FILL_RULE, destination_in,
    DESTINATION_TRIANGLES[6],  FILL_DESTINATION, FILL_RULE, default_blending,
    SOURCE_TRIANGLES[6],       FILL_SOURCE,      FILL_RULE, source_atop,
    DESTINATION_TRIANGLES[7],  FILL_DESTINATION, FILL_RULE, default_blending,
    SOURCE_TRIANGLES[7],       FILL_SOURCE,      FILL_RULE, destination_atop,
    DESTINATION_TRIANGLES[8],  FILL_DESTINATION, FILL_RULE, default_blending,
    SOURCE_TRIANGLES[8],       FILL_SOURCE,      FILL_RULE, xor,
    DESTINATION_TRIANGLES[9],  FILL_DESTINATION, FILL_RULE, default_blending,
    SOURCE_TRIANGLES[9],       FILL_SOURCE,      FILL_RULE, source,
    DESTINATION_TRIANGLES[10], FILL_DESTINATION, FILL_RULE, default_blending,
    SOURCE_TRIANGLES[10],      FILL_SOURCE,      FILL_RULE, destination,
    DESTINATION_TRIANGLES[11], FILL_DESTINATION, FILL_RULE, default_blending,
    SOURCE_TRIANGLES[11],      FILL_SOURCE,      FILL_RULE, additive
}
