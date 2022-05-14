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
    const NUM_VERTICES: f64 = 8.0;
    const RADIUS: f64 = SQUARE_SIZE / 2.0;
    const CX: f64 = SQUARE_SIZE / 2.0;
    const CY: f64 = SQUARE_SIZE / 2.0;
    let mut hex = Vec::<PathOps>::with_capacity(9);
    let mut alpha = 0.0_f64;

    while alpha < 360.0 {
        let x = RADIUS * f64::cos(alpha.to_radians()) + CX;
        let y = RADIUS * f64::sin(alpha.to_radians()) + CY;
        if hex.is_empty() {
            hex.push(PathOps::MoveTo { x, y });
        } else {
            hex.push(PathOps::LineTo { x, y });
        }
        alpha += 360.0 / NUM_VERTICES;
    }

    hex.push(PathOps::Close);

    // Linear gradients:
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
        hex.as_slice(),
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

    // Radial gradients:
    let square_stops = [
        (Color::black(), 0.0),
        (Color::red(), 0.5),
        (Color::white(), 1.0),
    ];
    canvas.draw_shape(
        &SQUARE,
        FillStyle::RadialGradient {
            stops: &square_stops,
            translation: Point { x: 0.0, y: 0.0 },
        },
        FillRule::NonZero,
        |p: &Point| translate(p, GAP, 2.0 * GAP + SQUARE_SIZE),
    );

    let hex_stops = [
        (Color::forest_green(), 0.0),
        (Color::steel_blue(), 0.25),
        (Color::cyan(), 0.5),
        (Color::forest_green(), 0.75),
        (Color::coral(), 1.0),
    ];
    canvas.draw_shape(
        hex.as_slice(),
        FillStyle::RadialGradient {
            stops: &hex_stops,
            translation: Point { x: 0.0, y: 0.0 },
        },
        FillRule::NonZero,
        |p: &Point| translate(p, 2.0 * GAP + SQUARE_SIZE, 2.0 * GAP + SQUARE_SIZE),
    );

    let triangle_stops = [
        (Color::yellow(), 0.0),
        (Color::forest_green(), 0.25),
        (Color::white(), 0.5),
        (Color::black(), 0.75),
        (Color::cyan(), 1.0),
    ];
    canvas.draw_shape(
        &CURVED_TRIANGLE,
        FillStyle::RadialGradient {
            stops: &triangle_stops,
            translation: Point { x: 10.0, y: -5.0 },
        },
        FillRule::NonZero,
        |p: &Point| translate(p, 3.0 * GAP + 2.0 * SQUARE_SIZE, 2.0 * GAP + SQUARE_SIZE),
    );

    // Conic gradients:
    let square_stops = [
        (Color::blue(), Angle::from_degrees(0.0)),
        (Color::yellow(), Angle::from_degrees(180.0)),
    ];
    canvas.draw_shape(
        &SQUARE,
        FillStyle::ConicGradient {
            stops: &square_stops,
            translation: Point { x: 0.0, y: 0.0 },
        },
        FillRule::NonZero,
        |p: &Point| translate(p, GAP, 3.0 * GAP + 2.0 * SQUARE_SIZE),
    );

    let hex_stops = [
        (Color::forest_green(), Angle::from_degrees(0.0)),
        (Color::steel_blue(), Angle::from_degrees(90.0)),
        (Color::cyan(), Angle::from_degrees(180.0)),
        (Color::forest_green(), Angle::from_degrees(270.0)),
    ];
    canvas.draw_shape(
        hex.as_slice(),
        FillStyle::ConicGradient {
            stops: &hex_stops,
            translation: Point { x: 0.0, y: 0.0 },
        },
        FillRule::NonZero,
        |p: &Point| translate(p, 2.0 * GAP + SQUARE_SIZE, 3.0 * GAP + 2.0 * SQUARE_SIZE),
    );

    let triangle_stops = [
        (Color::yellow(), Angle::from_degrees(0.0)),
        (Color::forest_green(), Angle::from_degrees(72.0)),
        (Color::white(), Angle::from_degrees(216.0)),
        (Color::black(), Angle::from_degrees(288.0)),
    ];
    canvas.draw_shape(
        &CURVED_TRIANGLE,
        FillStyle::ConicGradient {
            stops: &triangle_stops,
            translation: Point { x: 10.0, y: -5.0 },
        },
        FillRule::NonZero,
        |p: &Point| {
            translate(
                p,
                3.0 * GAP + 2.0 * SQUARE_SIZE,
                3.0 * GAP + 2.0 * SQUARE_SIZE,
            )
        },
    );
}

implement_test! {
    gradients_test, canvas_description, callback |
}
