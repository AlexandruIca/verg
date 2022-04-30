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
    let fill_color = Color {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };

    let mut canvas = Canvas::new(CanvasDescription {
        width: WIDTH,
        height: HEIGHT,
        background_color: Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        },
        ..Default::default()
    });

    canvas.draw_shape(
        vec![
            PathOps::MoveTo { x: 120.0, y: 120.0 },
            PathOps::LineTo { x: 360.0, y: 360.0 },
        ],
        FillStyle::Plain(fill_color),
        FillRule::NonZero,
    );

    canvas.draw_shape(
        vec![
            PathOps::MoveTo { x: 380.0, y: 80.0 },
            PathOps::LineTo { x: 200.0, y: 60.0 },
        ],
        FillStyle::Plain(fill_color),
        FillRule::NonZero,
    );
    canvas.draw_shape(
        vec![
            PathOps::MoveTo { x: 200.0, y: 115.0 },
            PathOps::LineTo { x: 380.0, y: 120.0 },
        ],
        FillStyle::Plain(fill_color),
        FillRule::NonZero,
    );

    canvas.draw_shape(
        vec![
            PathOps::MoveTo { x: 60.0, y: 20.0 },
            PathOps::LineTo { x: 60.0, y: 300.0 },
        ],
        FillStyle::Plain(fill_color),
        FillRule::NonZero,
    );

    canvas.draw_shape(
        vec![
            PathOps::MoveTo { x: 60.0, y: 350.0 },
            PathOps::LineTo { x: 360.0, y: 350.0 },
        ],
        FillStyle::Plain(fill_color),
        FillRule::NonZero,
    );

    canvas.draw_shape(
        vec![
            PathOps::MoveTo { x: 140.0, y: 400.0 },
            PathOps::LineTo { x: 141.0, y: 490.0 },
        ],
        FillStyle::Plain(fill_color),
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
