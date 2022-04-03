// This test should just write a gray 100x100 image into `basic_test.png`.

use verg::{
    canvas::{Canvas, CanvasDescription},
    color::{Color, FillRule, FillStyle},
    geometry::PathOps,
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
            g: 0.5_f64,
            b: 0.5_f64,
            a: 1.0_f64,
        },
        ..Default::default()
    });

    assert_eq!(canvas.desc.width, WIDTH);
    assert_eq!(canvas.desc.height, HEIGHT);

    canvas.draw_shape(
        vec![
            PathOps::MoveTo { x: 0.0, y: 0.0 },
            PathOps::LineTo {
                x: WIDTH as f64 - 1.0_f64,
                y: 0.0,
            },
            PathOps::LineTo {
                x: WIDTH as f64 - 1.0_f64,
                y: HEIGHT as f64 - 1.0_f64,
            },
            PathOps::LineTo {
                x: 0.0,
                y: HEIGHT as f64 - 1.0_f64,
            },
            PathOps::Close,
        ],
        FillStyle::Plain(Color {
            r: 0.3,
            g: 0.2,
            b: 0.3,
            a: 1.0,
        }),
        FillRule::NonZero,
    );

    let u8_buffer = canvas.to_u8();

    image::save_buffer(
        "basic_test.png",
        u8_buffer.as_slice(),
        canvas.desc.width as u32,
        canvas.desc.height as u32,
        image::ColorType::Rgba8,
    )
    .unwrap();
}
