// This test should just write a gray 100x100 image into `basic_test.png`.

use verg::{
    canvas::{Canvas, CanvasDescription},
    color::{Color, FillRule, FillStyle},
    geometry::PathOps,
};

#[test]
fn line_test() {
    const WIDTH: usize = 500;
    const HEIGHT: usize = 500;

    let mut canvas = Canvas::new(CanvasDescription {
        width: WIDTH,
        height: HEIGHT,
        background_color: Color {
            r: 1.0_f64,
            g: 1.0_f64,
            b: 1.0_f64,
            a: 1.0_f64,
        },
        ..Default::default()
    });

    canvas.draw_shape(
        vec![
            PathOps::MoveTo {
                x: 120.0_f64,
                y: 120.0_f64,
            },
            PathOps::LineTo {
                x: 360.0_f64,
                y: 360.0_f64,
            },
        ],
        FillStyle::Plain(Color {
            r: 1.0_f64,
            ..Default::default()
        }),
        FillRule::NonZero,
    );

    canvas.draw_shape(
        vec![
            PathOps::MoveTo {
                x: 380.0_f64,
                y: 80.0_f64,
            },
            PathOps::LineTo {
                x: 200.0_f64,
                y: 60.0_f64,
            },
        ],
        FillStyle::Plain(Color {
            r: 1.0_f64,
            ..Default::default()
        }),
        FillRule::NonZero,
    );

    canvas.draw_shape(
        vec![
            PathOps::MoveTo {
                x: 60.0_f64,
                y: 20.0_f64,
            },
            PathOps::LineTo {
                x: 60.0_f64,
                y: 300.0_f64,
            },
        ],
        FillStyle::Plain(Color {
            r: 1.0_f64,
            ..Default::default()
        }),
        FillRule::NonZero,
    );

    canvas.draw_shape(
        vec![
            PathOps::MoveTo {
                x: 60.0_f64,
                y: 350.0_f64,
            },
            PathOps::LineTo {
                x: 360.0_f64,
                y: 350.0_f64,
            },
        ],
        FillStyle::Plain(Color {
            r: 1.0_f64,
            ..Default::default()
        }),
        FillRule::NonZero,
    );

    let u8_buffer = canvas.to_u8();

    image::save_buffer(
        "line_test.png",
        u8_buffer.as_slice(),
        canvas.desc.width as u32,
        canvas.desc.height as u32,
        image::ColorType::Rgba8,
    )
    .unwrap();
}
