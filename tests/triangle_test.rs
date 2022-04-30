// This test draws a bunch of triangles of different sizes, colors and shapes.

use verg::canvas::{Canvas, CanvasDescription};
use verg::color::{Color, FillRule, FillStyle};
use verg::geometry::PathOps;

#[test]
fn triangle_test() {
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

    // Big triangle
    canvas.draw_shape(
        vec![
            PathOps::MoveTo { x: 60.0, y: 240.0 },
            PathOps::LineTo { x: 360.0, y: 80.0 },
            PathOps::LineTo { x: 400.0, y: 280.0 },
            PathOps::Close,
        ],
        FillStyle::Plain(fill_color),
        FillRule::NonZero,
    );

    // Small white triangle
    canvas.draw_shape(
        vec![
            PathOps::MoveTo { x: 100.0, y: 150.0 },
            PathOps::LineTo { x: 100.0, y: 110.0 },
            PathOps::LineTo { x: 150.0, y: 110.0 },
            PathOps::Close,
        ],
        FillStyle::Plain(fill_color),
        FillRule::NonZero,
    );

    // Red triangles
    canvas.draw_shape(
        vec![
            PathOps::MoveTo { x: 100.0, y: 50.0 },
            PathOps::LineTo { x: 100.0, y: 10.0 },
            PathOps::LineTo { x: 150.0, y: 10.0 },
            PathOps::Close,
            PathOps::MoveTo { x: 250.0, y: 50.0 },
            PathOps::LineTo { x: 300.0, y: 50.0 },
            PathOps::LineTo { x: 300.0, y: 10.0 },
            PathOps::Close,
        ],
        FillStyle::Plain(Color {
            r: 0.8,
            g: 0.1,
            b: 0.3,
            a: 1.0,
        }),
        FillRule::NonZero,
    );

    // Blue rect
    canvas.draw_shape(
        vec![
            PathOps::MoveTo { x: 10.0, y: 10.0 },
            PathOps::LineTo { x: 10.0, y: 50.0 },
            PathOps::LineTo { x: 50.0, y: 10.0 },
            PathOps::Close,
            PathOps::MoveTo { x: 10.0, y: 50.0 },
            PathOps::LineTo { x: 50.0, y: 50.0 },
            PathOps::LineTo { x: 50.0, y: 10.0 },
            PathOps::Close,
        ],
        FillStyle::Plain(Color {
            r: 0.0,
            g: 0.0,
            b: 1.0,
            a: 1.0,
        }),
        FillRule::NonZero,
    );

    // Imperfect yellow rect
    canvas.draw_shape(
        vec![
            PathOps::MoveTo { x: 400.0, y: 10.0 },
            PathOps::LineTo { x: 400.0, y: 50.0 },
            PathOps::LineTo { x: 450.0, y: 10.0 },
            PathOps::Close,
        ],
        FillStyle::Plain(Color {
            r: 0.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }),
        FillRule::NonZero,
    );

    canvas.draw_shape(
        vec![
            PathOps::MoveTo { x: 400.0, y: 50.0 },
            PathOps::LineTo { x: 450.0, y: 50.0 },
            PathOps::LineTo { x: 450.0, y: 10.0 },
            PathOps::Close,
        ],
        FillStyle::Plain(Color {
            r: 0.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }),
        FillRule::NonZero,
    );

    // Diamond
    canvas.draw_shape(
        vec![
            PathOps::MoveTo { x: 50.0, y: 350.0 },
            PathOps::LineTo { x: 100.0, y: 300.0 },
            PathOps::LineTo { x: 100.0, y: 400.0 },
            PathOps::Close,
            PathOps::MoveTo { x: 150.0, y: 350.0 },
            PathOps::LineTo { x: 100.0, y: 300.0 },
            PathOps::LineTo { x: 100.0, y: 400.0 },
            PathOps::Close,
        ],
        FillStyle::Plain(Color {
            r: 0.1,
            g: 0.6,
            b: 0.1,
            a: 1.0,
        }),
        FillRule::NonZero,
    );

    // Triangle with an almost horizontal side
    canvas.draw_shape(
        vec![
            PathOps::MoveTo { x: 300.0, y: 400.0 },
            PathOps::LineToRel { x: 50.0, y: -50.0 },
            PathOps::LineToRel { x: -10.0, y: 51.0 },
            PathOps::Close,
        ],
        FillStyle::Plain(Color {
            r: 0.3,
            g: 0.1,
            b: 0.5,
            a: 1.0,
        }),
        FillRule::NonZero,
    );

    let u8_buffer = canvas.to_u8();

    image::save_buffer(
        "triangle_test.png",
        u8_buffer.as_slice(),
        canvas.desc.width as u32,
        canvas.desc.height as u32,
        image::ColorType::Rgba8,
    )
    .unwrap();
}
