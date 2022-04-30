// This tests draws a triangle inside a square using the even-odd fill rule.

use verg::{
    canvas::{Canvas, CanvasDescription},
    color::{Color, FillStyle},
    geometry::PathOps,
};

#[test]
fn even_odd_fill_test() {
    const WIDTH: usize = 1000;
    const HEIGHT: usize = 1000;

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
            PathOps::MoveTo { x: 200.0, y: 200.0 },
            PathOps::LineTo { x: 800.0, y: 200.0 },
            PathOps::LineTo { x: 800.0, y: 800.0 },
            PathOps::LineTo { x: 200.0, y: 800.0 },
            PathOps::Close,
            PathOps::MoveTo { x: 300.0, y: 300.0 },
            PathOps::LineTo { x: 700.0, y: 300.0 },
            PathOps::LineTo { x: 700.0, y: 700.0 },
            PathOps::LineTo { x: 300.0, y: 700.0 },
            PathOps::Close,
            PathOps::MoveTo { x: 450.0, y: 500.0 },
            PathOps::LineTo { x: 550.0, y: 450.0 },
            PathOps::LineTo { x: 550.0, y: 550.0 },
            PathOps::Close,
            PathOps::MoveTo { x: 460.0, y: 500.0 },
            PathOps::LineTo { x: 540.0, y: 480.0 },
            PathOps::LineTo { x: 540.0, y: 540.0 },
            PathOps::Close,
        ],
        FillStyle::Plain(Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }),
        verg::color::FillRule::EvenOdd,
    );

    image::save_buffer(
        "even_odd_fill_test.png",
        canvas.to_u8().as_slice(),
        WIDTH as u32,
        HEIGHT as u32,
        image::ColorType::Rgba8,
    )
    .unwrap();
}
