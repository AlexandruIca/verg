use verg::{
    canvas::{Canvas, CanvasDescription, ViewBox},
    color::{Color, FillRule, FillStyle},
    geometry::{PathOps, Point},
    math::{translate, Angle},
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
        background_color: Color::dark_slate_blue(),
        tolerance: 1.0,
    }
}

const GAP: f64 = 100.0;

const SQUARE_SIZE: f64 = 200.0;
const SQUARE: [PathOps; 5] = [
    PathOps::MoveTo { x: 0.0, y: 0.0 },
    PathOps::LineTo {
        x: SQUARE_SIZE,
        y: 0.0,
    },
    PathOps::LineTo {
        x: SQUARE_SIZE,
        y: SQUARE_SIZE,
    },
    PathOps::LineTo {
        x: 0.0,
        y: SQUARE_SIZE,
    },
    PathOps::Close,
];

const HEX_WIDTH: f64 = 200.0;
const HEX_HEIGHT: f64 = 200.0;
const HEX: [PathOps; 7] = [
    PathOps::MoveTo {
        x: HEX_WIDTH / 2.0,
        y: 0.0,
    },
    PathOps::LineTo {
        x: HEX_WIDTH,
        y: HEX_HEIGHT / 3.0,
    },
    PathOps::LineTo {
        x: HEX_WIDTH,
        y: 2.0 * HEX_HEIGHT / 3.0,
    },
    PathOps::LineTo {
        x: HEX_WIDTH / 2.0,
        y: HEX_HEIGHT,
    },
    PathOps::LineTo {
        x: 0.0,
        y: 2.0 * HEX_HEIGHT / 3.0,
    },
    PathOps::LineTo {
        x: 0.0,
        y: HEX_HEIGHT / 3.0,
    },
    PathOps::Close,
];

const CURVED_TRIANGLE: [PathOps; 4] = [
    PathOps::MoveTo { x: 0.0, y: 180.0 },
    PathOps::CubicTo {
        x1: 45.0,
        y1: 85.0,
        x2: 95.0,
        y2: 65.0,
        x3: 105.0,
        y3: 0.0,
    },
    PathOps::CubicTo {
        x1: 110.0,
        y1: 140.0,
        x2: 160.0,
        y2: 150.0,
        x3: 200.0,
        y3: 200.0,
    },
    PathOps::Close,
];

fn callback(canvas: &mut Canvas) {
    let square_stops = [
        (Color::white(), 0.0),
        (Color::red(), 0.5),
        (Color::black(), 1.0),
    ];
    canvas.draw_shape(
        &SQUARE,
        FillStyle::LinearGradient {
            stops: &square_stops,
            angle: Angle::from_radians(0.0),
        },
        FillRule::NonZero,
        |p: &Point| translate(p, GAP, GAP),
    );

    let hex_stops = [
        (Color::crimson(), 0.0),
        (Color::dark_slate_blue(), 0.25),
        (Color::cyan(), 0.5),
        (Color::coral(), 0.75),
        (Color::forest_green(), 1.0),
    ];
    canvas.draw_shape(
        &HEX,
        FillStyle::LinearGradient {
            stops: &hex_stops,
            angle: Angle::from_degrees(45.0),
        },
        FillRule::NonZero,
        |p: &Point| translate(p, 2.0 * GAP + SQUARE_SIZE, GAP),
    );

    let triangle_stops = [
        (Color::yellow(), 0.0),
        (Color::steel_blue(), 0.25),
        (Color::white(), 0.5),
        (Color::forest_green(), 0.75),
        (Color::cyan(), 1.0),
    ];
    canvas.draw_shape(
        &CURVED_TRIANGLE,
        FillStyle::LinearGradient {
            stops: &triangle_stops,
            angle: Angle::from_degrees(285.0),
        },
        FillRule::NonZero,
        |p: &Point| translate(p, 3.0 * GAP + 2.0 * SQUARE_SIZE, GAP),
    );
}

implement_test! {
    gradients_test, canvas_description, callback |
}
