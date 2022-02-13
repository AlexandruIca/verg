// This test should just write a gray 100x100 image into `basic_test.png`.

use verg::{
    canvas::{Canvas, CanvasDescription},
    color::{Color, FillRule, FillStyle},
    geometry::{Point, Primitive},
};

#[test]
fn basic_test() {
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
        vec![Primitive::Line {
            start: Point {
                x: 120.0_f64,
                y: 120.0_f64,
            },
            end: Point {
                x: 360.0_f64,
                y: 360.0_f64,
            },
        }],
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
