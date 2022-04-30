// This test draws a rectangle that's not filled inside.

use verg::canvas::{Canvas, CanvasDescription};
use verg::color::{Color, FillRule, FillStyle};
use verg::geometry::PathOps;

#[test]
fn rect_test() {
    const WIDTH: usize = 500;
    const HEIGHT: usize = 500;
    let fill_color = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };

    let mut canvas = Canvas::new(CanvasDescription {
        width: WIDTH,
        height: HEIGHT,
        background_color: Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        },
        ..Default::default()
    });

    canvas.draw_shape(
        vec![
            PathOps::MoveTo { x: 80.0, y: 80.0 },
            PathOps::LineTo { x: 80.0, y: 420.0 },
            PathOps::LineTo { x: 420.0, y: 420.0 },
            PathOps::LineTo {
                x: 420.0_f64,
                y: 80.0_f64,
            },
            PathOps::Close,
            PathOps::MoveTo { x: 10.0, y: 10.0 },
            PathOps::LineTo { x: 490.0, y: 10.0 },
            PathOps::LineTo { x: 490.0, y: 490.0 },
            PathOps::LineTo { x: 10.0, y: 490.0 },
            PathOps::Close,
            PathOps::MoveTo { x: 300.0, y: 200.0 },
            PathOps::LineTo { x: 300.0, y: 300.0 },
            PathOps::LineTo { x: 200.0, y: 300.0 },
            PathOps::LineTo { x: 200.0, y: 200.0 },
            PathOps::Close,
        ],
        FillStyle::Plain(fill_color),
        FillRule::NonZero,
    );

    let u8_buffer = canvas.to_u8();

    image::save_buffer(
        "rect_test.png",
        u8_buffer.as_slice(),
        canvas.desc.width as u32,
        canvas.desc.height as u32,
        image::ColorType::Rgba8,
    )
    .unwrap();
}
