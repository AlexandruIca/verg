// This test draws using affine transformations applied on paths.

use verg::{
    canvas::{Canvas, CanvasDescription, ViewBox},
    color::{Color, FillRule, FillStyle},
    geometry::{PathOps, Point},
    math::{rotate_around, scale_around, skew_around, translate, Angle},
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
            width: 100.0,
            height: 100.0,
        },
        background_color: Color::white(),
        ..Default::default()
    }
}

const PATH: [PathOps; 5] = [
    PathOps::MoveTo { x: 5.0, y: 5.0 },
    PathOps::LineTo { x: 15.0, y: 5.0 },
    PathOps::LineTo { x: 15.0, y: 15.0 },
    PathOps::LineTo { x: 5.0, y: 15.0 },
    PathOps::Close,
];

const FILL_RULE: FillRule = FillRule::NonZero;

static TRANSLATIONS: [Point; 25] = [
    Point { x: 0.0, y: 0.0 },
    Point { x: 20.0, y: 0.0 },
    Point { x: 40.0, y: 0.0 },
    Point { x: 60.0, y: 0.0 },
    Point { x: 80.0, y: 0.0 },
    Point { x: 0.0, y: 20.0 },
    Point { x: 20.0, y: 20.0 },
    Point { x: 40.0, y: 20.0 },
    Point { x: 60.0, y: 20.0 },
    Point { x: 80.0, y: 20.0 },
    Point { x: 0.0, y: 40.0 },
    Point { x: 20.0, y: 40.0 },
    Point { x: 40.0, y: 40.0 },
    Point { x: 60.0, y: 40.0 },
    Point { x: 80.0, y: 40.0 },
    Point { x: 0.0, y: 60.0 },
    Point { x: 20.0, y: 60.0 },
    Point { x: 40.0, y: 60.0 },
    Point { x: 60.0, y: 60.0 },
    Point { x: 80.0, y: 60.0 },
    Point { x: 0.0, y: 80.0 },
    Point { x: 20.0, y: 80.0 },
    Point { x: 40.0, y: 80.0 },
    Point { x: 60.0, y: 80.0 },
    Point { x: 80.0, y: 80.0 },
];

pub fn callback(canvas: &mut Canvas) {
    let black = FillStyle::Plain(Color::black());
    let dark_slate_blue = FillStyle::Plain(Color::dark_slate_blue());
    let forest_green = FillStyle::Plain(Color::forest_green());
    let blue = FillStyle::Plain(Color::blue());
    let crimson = FillStyle::Plain(Color::crimson());
    let coral = FillStyle::Plain(Color::coral());
    let cyan = FillStyle::Plain(Color::cyan());

    {
        let translate = |i: usize| {
            return move |p: &Point| translate(p, TRANSLATIONS[i].x, TRANSLATIONS[i].y);
        };

        canvas.draw_shape(&PATH, black, FILL_RULE, translate(0));
        canvas.draw_shape(&PATH, dark_slate_blue, FILL_RULE, translate(1));
        canvas.draw_shape(&PATH, blue, FILL_RULE, translate(2));
        canvas.draw_shape(&PATH, forest_green, FILL_RULE, translate(3));
        canvas.draw_shape(&PATH, black, FILL_RULE, translate(4));
    }

    {
        let rotate = |i: usize, angle: f64| {
            return move |p: &Point| {
                let p = translate(p, TRANSLATIONS[i].x, TRANSLATIONS[i].y);
                let around = Point {
                    x: TRANSLATIONS[i].x + 10.0,
                    y: TRANSLATIONS[i].y + 10.0,
                };

                rotate_around(&p, &around, Angle::from_degrees(angle))
            };
        };

        canvas.draw_shape(&PATH, dark_slate_blue, FILL_RULE, rotate(5, 25.0));
        canvas.draw_shape(&PATH, crimson, FILL_RULE, rotate(6, -45.0));
        canvas.draw_shape(&PATH, blue, FILL_RULE, rotate(7, 270.0));
        canvas.draw_shape(&PATH, forest_green, FILL_RULE, rotate(8, 170.0));
        canvas.draw_shape(&PATH, coral, FILL_RULE, rotate(9, 330.0));
    }

    {
        let scale = |i: usize, sx: f64, sy: f64| {
            return move |p: &Point| {
                let p = translate(p, TRANSLATIONS[i].x, TRANSLATIONS[i].y);
                let center = Point {
                    x: TRANSLATIONS[i].x + 10.0,
                    y: TRANSLATIONS[i].y + 10.0,
                };

                scale_around(&p, &center, sx, sy)
            };
        };

        canvas.draw_shape(&PATH, dark_slate_blue, FILL_RULE, scale(10, 0.5, 0.5));
        canvas.draw_shape(&PATH, crimson, FILL_RULE, scale(11, 0.5, 1.0));
        canvas.draw_shape(&PATH, blue, FILL_RULE, scale(12, 1.2, 1.2));
        canvas.draw_shape(&PATH, forest_green, FILL_RULE, scale(13, 1.0, 2.5));
        canvas.draw_shape(&PATH, coral, FILL_RULE, scale(14, 1.5, 2.5));
    }

    {
        let skew = |i: usize, x: f64, y: f64| {
            return move |p: &Point| {
                let p = translate(p, TRANSLATIONS[i].x, TRANSLATIONS[i].y);
                let center = Point {
                    x: TRANSLATIONS[i].x + 10.0,
                    y: TRANSLATIONS[i].y + 10.0,
                };

                skew_around(&p, &center, Angle::from_degrees(x), Angle::from_degrees(y))
            };
        };

        canvas.draw_shape(&PATH, dark_slate_blue, FILL_RULE, skew(15, 15.0, 15.0));
        canvas.draw_shape(&PATH, cyan, FILL_RULE, skew(16, -15.0, -15.0));
        canvas.draw_shape(&PATH, coral, FILL_RULE, skew(17, 45.0, -15.0));
        canvas.draw_shape(&PATH, forest_green, FILL_RULE, skew(18, 0.0, 0.0));
        canvas.draw_shape(&PATH, crimson, FILL_RULE, skew(19, 0.0, 20.0));
    }

    {
        let transform = |i: usize, sx: f64, sy: f64, skew_x: f64, skew_y: f64, rotate: f64| {
            return move |p: &Point| {
                let p = translate(p, TRANSLATIONS[i].x, TRANSLATIONS[i].y);
                let center = Point {
                    x: TRANSLATIONS[i].x + 10.0,
                    y: TRANSLATIONS[i].y + 10.0,
                };

                let p = scale_around(&p, &center, sx, sy);
                let p = skew_around(
                    &p,
                    &center,
                    Angle::from_degrees(skew_x),
                    Angle::from_degrees(skew_y),
                );

                rotate_around(&p, &center, Angle::from_degrees(rotate))
            };
        };

        canvas.draw_shape(
            &PATH,
            blue,
            FILL_RULE,
            transform(20, 0.8, 0.5, 10.0, 10.0, 36.0),
        );
        canvas.draw_shape(
            &PATH,
            crimson,
            FILL_RULE,
            transform(21, 1.8, 1.5, 15.0, 15.0, 127.0),
        );
        canvas.draw_shape(
            &PATH,
            black,
            FILL_RULE,
            transform(22, -0.6, -0.6, 30.0, 30.0, 90.0),
        );
        canvas.draw_shape(
            &PATH,
            dark_slate_blue,
            FILL_RULE,
            transform(23, -1.4, -1.4, 20.0, 20.0, 270.0),
        );
        canvas.draw_shape(
            &PATH,
            forest_green,
            FILL_RULE,
            transform(24, 1.4, -0.4, 20.0, 20.0, 120.0),
        );
    }
}

implement_test! {
    affine_transforms_test, canvas_description, callback |
}
