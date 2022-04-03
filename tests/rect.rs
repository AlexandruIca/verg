use verg::canvas::{Canvas, CanvasDescription};
use verg::color::{Color, FillRule, FillStyle};
use verg::geometry::{Path, PathOps};

#[test]
fn rect_test() {
    const WIDTH: usize = 500;
    const HEIGHT: usize = 500;
    let fill_color = Color {
        r: 1.0_f64,
        g: 1.0_f64,
        b: 1.0_f64,
        a: 1.0_f64,
    };

    let mut canvas = Canvas::new(CanvasDescription {
        width: WIDTH,
        height: HEIGHT,
        background_color: Color {
            r: 0.0_f64,
            g: 0.0_f64,
            b: 0.0_f64,
            a: 1.0_f64,
        },
        ..Default::default()
    });

    canvas.draw_shape(
        vec![
            PathOps::MoveTo {
                x: 80.0_f64,
                y: 80.0_f64,
            },
            PathOps::LineTo {
                x: 80.0_f64,
                y: 420.0_f64,
            },
            PathOps::LineTo {
                x: 420.0_f64,
                y: 420.0_f64,
            },
            PathOps::LineTo {
                x: 420.0_f64,
                y: 80.0_f64,
            },
            PathOps::Close,
            PathOps::MoveTo {
                x: 10.0_f64,
                y: 10.0_f64,
            },
            PathOps::LineTo {
                x: 490.0_f64,
                y: 10.0_f64,
            },
            PathOps::LineTo {
                x: 490.0_f64,
                y: 490.0_f64,
            },
            PathOps::LineTo {
                x: 10.0_f64,
                y: 490.0_f64,
            },
            PathOps::Close,
            PathOps::MoveTo {
                x: 300.0_f64,
                y: 200.0_f64,
            },
            PathOps::LineTo {
                x: 300.0_f64,
                y: 300.0_f64,
            },
            PathOps::LineTo {
                x: 200.0_f64,
                y: 300.0_f64,
            },
            PathOps::LineTo {
                x: 200.0_f64,
                y: 200.0_f64,
            },
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
