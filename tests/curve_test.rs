// This test draw quadratic Bézier curves.

use crate::common::default_blending;
use verg::{
    canvas::{Canvas, CanvasDescription, ViewBox},
    color::{Color, FillRule, FillStyle},
    geometry::{PathOps, Point},
    math::{rotate_around, scale_around, translate, Angle},
};

mod common;

const WIDTH: usize = 805;
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

const PATH: [PathOps; 4] = [
    PathOps::MoveTo { x: 20.0, y: 360.0 },
    PathOps::CubicTo {
        x1: 100.0,
        y1: 260.0,
        x2: 50.0,
        y2: 160.0,
        x3: 150.0,
        y3: 60.0,
    },
    PathOps::CubicTo {
        x1: 120.0,
        y1: 160.0,
        x2: 150.0,
        y2: 260.0,
        x3: 200.0,
        y3: 360.0,
    },
    PathOps::CubicTo {
        x1: 90.0,
        y1: 320.0,
        x2: 130.0,
        y2: 320.0,
        x3: 20.0,
        y3: 360.0,
    },
];

const DARK_SLATE_BLUE: FillStyle = FillStyle::Plain(Color::dark_slate_blue());
const YELLOW: FillStyle = FillStyle::Plain(Color::yellow());
const BLACK: FillStyle = FillStyle::Plain(Color::black());

const FILL_RULE: FillRule = FillRule::NonZero;

const MOON_WIDTH: f64 = 30.0;
const MOON_HEIGHT: f64 = 80.0;
const MOON_VERTICAL_OFFSET: f64 = 5.0;

const MOON: [PathOps; 3] = [
    PathOps::MoveTo { x: 0.0, y: 0.0 },
    PathOps::CubicTo {
        x1: MOON_WIDTH / 2.0,
        y1: MOON_HEIGHT / 2.0 - MOON_VERTICAL_OFFSET,
        x2: MOON_WIDTH / 2.0,
        y2: MOON_HEIGHT / 2.0 + MOON_VERTICAL_OFFSET,
        x3: 0.0,
        y3: MOON_HEIGHT,
    },
    PathOps::CubicTo {
        x1: MOON_WIDTH,
        y1: MOON_HEIGHT / 2.0 + MOON_VERTICAL_OFFSET,
        x2: MOON_WIDTH,
        y2: MOON_HEIGHT / 2.0 - MOON_VERTICAL_OFFSET,
        x3: 0.0,
        y3: 0.0,
    },
];

const EYE_WIDTH: f64 = 150.0;
const EYE_HEIGHT: f64 = 300.0;
const EYE_QUARTER_W: f64 = EYE_WIDTH / 4.0;
const EYE_QUARTER_H: f64 = EYE_HEIGHT / 4.0;
const EYE_INNER_OFFSET: f64 = 10.0;

const EYE: [PathOps; 10] = [
    PathOps::MoveTo {
        x: EYE_WIDTH / 2.0,
        y: EYE_HEIGHT,
    },
    PathOps::CubicTo {
        x1: EYE_WIDTH / 2.0 + EYE_QUARTER_W,
        y1: EYE_HEIGHT,
        x2: EYE_WIDTH,
        y2: EYE_HEIGHT - EYE_QUARTER_H,
        x3: EYE_WIDTH,
        y3: EYE_HEIGHT / 2.0,
    },
    PathOps::CubicTo {
        x1: EYE_WIDTH,
        y1: EYE_HEIGHT / 2.0 - EYE_QUARTER_H,
        x2: EYE_WIDTH - EYE_QUARTER_W,
        y2: 0.0,
        x3: EYE_WIDTH / 2.0,
        y3: 0.0,
    },
    PathOps::CubicTo {
        x1: EYE_QUARTER_W,
        y1: 0.0,
        x2: 0.0,
        y2: EYE_QUARTER_H,
        x3: 0.0,
        y3: EYE_HEIGHT / 2.0,
    },
    PathOps::CubicTo {
        x1: 0.0,
        y1: EYE_HEIGHT / 2.0 + EYE_QUARTER_H,
        x2: EYE_QUARTER_W,
        y2: EYE_HEIGHT,
        x3: EYE_WIDTH / 2.0,
        y3: EYE_HEIGHT,
    },
    PathOps::MoveTo {
        x: EYE_WIDTH / 2.0,
        y: EYE_HEIGHT - EYE_INNER_OFFSET,
    },
    PathOps::CubicTo {
        x1: EYE_QUARTER_W + EYE_INNER_OFFSET,
        y1: EYE_HEIGHT - EYE_INNER_OFFSET,
        x2: EYE_INNER_OFFSET,
        y2: EYE_HEIGHT - EYE_QUARTER_H - EYE_INNER_OFFSET,
        x3: EYE_INNER_OFFSET,
        y3: EYE_HEIGHT / 2.0,
    },
    PathOps::CubicTo {
        x1: EYE_INNER_OFFSET,
        y1: EYE_QUARTER_H + EYE_INNER_OFFSET,
        x2: EYE_QUARTER_W + EYE_INNER_OFFSET,
        y2: EYE_INNER_OFFSET,
        x3: EYE_WIDTH / 2.0,
        y3: EYE_INNER_OFFSET,
    },
    PathOps::CubicTo {
        x1: EYE_WIDTH / 2.0 + EYE_QUARTER_W - EYE_INNER_OFFSET,
        y1: EYE_INNER_OFFSET,
        x2: EYE_WIDTH - EYE_INNER_OFFSET,
        y2: EYE_QUARTER_H + EYE_INNER_OFFSET,
        x3: EYE_WIDTH - EYE_INNER_OFFSET,
        y3: EYE_HEIGHT / 2.0,
    },
    PathOps::CubicTo {
        x1: EYE_WIDTH - EYE_INNER_OFFSET,
        y1: EYE_HEIGHT / 2.0 + EYE_QUARTER_H - EYE_INNER_OFFSET,
        x2: EYE_WIDTH / 2.0 + EYE_QUARTER_W - EYE_INNER_OFFSET,
        y2: EYE_HEIGHT - EYE_INNER_OFFSET,
        x3: EYE_WIDTH / 2.0,
        y3: EYE_HEIGHT - EYE_INNER_OFFSET,
    },
];

const SMALL_EYE_WIDTH: f64 = 80.0;
const SMALL_EYE_HEIGHT: f64 = 1.5 * SMALL_EYE_WIDTH;
const SMALL_EYE_QUARTER_W: f64 = SMALL_EYE_WIDTH / 4.0;
const SMALL_EYE_QUARTER_H: f64 = SMALL_EYE_HEIGHT / 4.0;
const SMALL_EYE_INNER_OFFSET: f64 = 10.0;
const SMALL_EYE_UPPER: f64 = 4.0;

const SMALL_EYE: [PathOps; 10] = [
    PathOps::MoveTo {
        x: SMALL_EYE_WIDTH / 2.0,
        y: SMALL_EYE_HEIGHT,
    },
    PathOps::CubicTo {
        x1: SMALL_EYE_WIDTH / 2.0 + SMALL_EYE_QUARTER_W,
        y1: SMALL_EYE_HEIGHT,
        x2: SMALL_EYE_WIDTH,
        y2: SMALL_EYE_HEIGHT - SMALL_EYE_QUARTER_H,
        x3: SMALL_EYE_WIDTH,
        y3: SMALL_EYE_HEIGHT / 2.0,
    },
    PathOps::CubicTo {
        x1: SMALL_EYE_WIDTH,
        y1: SMALL_EYE_HEIGHT / 2.0 - SMALL_EYE_QUARTER_H,
        x2: SMALL_EYE_WIDTH - SMALL_EYE_QUARTER_W,
        y2: 0.0,
        x3: SMALL_EYE_WIDTH / 2.0,
        y3: 0.0,
    },
    PathOps::CubicTo {
        x1: SMALL_EYE_QUARTER_W,
        y1: 0.0,
        x2: 0.0,
        y2: SMALL_EYE_QUARTER_H,
        x3: 0.0,
        y3: SMALL_EYE_HEIGHT / 2.0,
    },
    PathOps::CubicTo {
        x1: 0.0,
        y1: SMALL_EYE_HEIGHT / 2.0 + SMALL_EYE_QUARTER_H,
        x2: SMALL_EYE_QUARTER_W,
        y2: SMALL_EYE_HEIGHT,
        x3: SMALL_EYE_WIDTH / 2.0,
        y3: SMALL_EYE_HEIGHT,
    },
    PathOps::MoveTo {
        x: SMALL_EYE_WIDTH / 2.0,
        y: SMALL_EYE_HEIGHT - SMALL_EYE_INNER_OFFSET,
    },
    PathOps::CubicTo {
        x1: SMALL_EYE_QUARTER_W + SMALL_EYE_INNER_OFFSET,
        y1: SMALL_EYE_HEIGHT - SMALL_EYE_INNER_OFFSET,
        x2: SMALL_EYE_INNER_OFFSET,
        y2: SMALL_EYE_HEIGHT - SMALL_EYE_QUARTER_H - SMALL_EYE_INNER_OFFSET,
        x3: SMALL_EYE_INNER_OFFSET,
        y3: SMALL_EYE_HEIGHT / 2.0,
    },
    PathOps::CubicTo {
        x1: SMALL_EYE_INNER_OFFSET,
        y1: SMALL_EYE_QUARTER_H + SMALL_EYE_INNER_OFFSET,
        x2: SMALL_EYE_QUARTER_W + SMALL_EYE_INNER_OFFSET,
        y2: SMALL_EYE_UPPER * SMALL_EYE_INNER_OFFSET,
        x3: SMALL_EYE_WIDTH / 2.0,
        y3: SMALL_EYE_UPPER * SMALL_EYE_INNER_OFFSET,
    },
    PathOps::CubicTo {
        x1: SMALL_EYE_WIDTH / 2.0 + SMALL_EYE_QUARTER_W - SMALL_EYE_INNER_OFFSET,
        y1: SMALL_EYE_UPPER * SMALL_EYE_INNER_OFFSET,
        x2: SMALL_EYE_WIDTH - SMALL_EYE_INNER_OFFSET,
        y2: SMALL_EYE_QUARTER_H + SMALL_EYE_INNER_OFFSET,
        x3: SMALL_EYE_WIDTH - SMALL_EYE_INNER_OFFSET,
        y3: SMALL_EYE_HEIGHT / 2.0,
    },
    PathOps::CubicTo {
        x1: SMALL_EYE_WIDTH - SMALL_EYE_INNER_OFFSET,
        y1: SMALL_EYE_HEIGHT / 2.0 + SMALL_EYE_QUARTER_H - SMALL_EYE_INNER_OFFSET,
        x2: SMALL_EYE_WIDTH / 2.0 + SMALL_EYE_QUARTER_W - SMALL_EYE_INNER_OFFSET,
        y2: SMALL_EYE_HEIGHT - SMALL_EYE_INNER_OFFSET,
        x3: SMALL_EYE_WIDTH / 2.0,
        y3: SMALL_EYE_HEIGHT - SMALL_EYE_INNER_OFFSET,
    },
];

const IRIS_WIDTH: f64 = 40.0;
const IRIS_HEIGHT: f64 = 1.5 * IRIS_WIDTH;
const IRIS_QUARTER_W: f64 = IRIS_WIDTH / 4.0;
const IRIS_QUARTER_H: f64 = IRIS_HEIGHT / 4.0;

const IRIS: [PathOps; 5] = [
    PathOps::MoveTo {
        x: IRIS_WIDTH / 2.0,
        y: IRIS_HEIGHT,
    },
    PathOps::CubicTo {
        x1: IRIS_WIDTH / 2.0 + IRIS_QUARTER_W,
        y1: IRIS_HEIGHT,
        x2: IRIS_WIDTH,
        y2: IRIS_HEIGHT - IRIS_QUARTER_H,
        x3: IRIS_WIDTH,
        y3: IRIS_HEIGHT / 2.0,
    },
    PathOps::CubicTo {
        x1: IRIS_WIDTH,
        y1: IRIS_HEIGHT / 2.0 - IRIS_QUARTER_H,
        x2: IRIS_WIDTH - IRIS_QUARTER_W,
        y2: 0.0,
        x3: IRIS_WIDTH / 2.0,
        y3: 0.0,
    },
    PathOps::CubicTo {
        x1: IRIS_QUARTER_W,
        y1: 0.0,
        x2: 0.0,
        y2: IRIS_QUARTER_H,
        x3: 0.0,
        y3: IRIS_HEIGHT / 2.0,
    },
    PathOps::CubicTo {
        x1: 0.0,
        y1: IRIS_HEIGHT / 2.0 + IRIS_QUARTER_H,
        x2: IRIS_QUARTER_W,
        y2: IRIS_HEIGHT,
        x3: IRIS_WIDTH / 2.0,
        y3: IRIS_HEIGHT,
    },
];

fn callback(canvas: &mut Canvas) {
    let transform = |p: &Point| {
        let center = Point {
            x: MOON_WIDTH / 2.0,
            y: MOON_HEIGHT / 2.0,
        };
        let p = rotate_around(&p, &center, Angle::from_degrees(65.0));
        let p = scale_around(&p, &center, 1.5, 1.5);

        translate(&p, 280.0, 180.0)
    };
    canvas.draw_shape(&MOON, YELLOW, FILL_RULE, transform);

    let transform = |x: f64, y: f64| {
        return move |p: &Point| translate(&p, x, y);
    };
    canvas.draw_shape(&EYE, BLACK, FILL_RULE, transform(400.0, 50.0));
    canvas.draw_shape(&EYE, BLACK, FILL_RULE, transform(400.0 + EYE_WIDTH, 50.0));
    canvas.draw_shape(&SMALL_EYE, BLACK, FILL_RULE, transform(433.0, 220.0));
    canvas.draw_shape(
        &SMALL_EYE,
        BLACK,
        FILL_RULE,
        transform(433.0 + EYE_WIDTH, 220.0),
    );
    canvas.draw_shape(&IRIS, BLACK, FILL_RULE, transform(448.0, 267.0));
    canvas.draw_shape(&IRIS, BLACK, FILL_RULE, transform(448.0 + EYE_WIDTH, 267.0));
}

implement_test! {
    curve_test, canvas_description, callback |
    PATH, DARK_SLATE_BLUE, FILL_RULE, default_blending
}
