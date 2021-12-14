// This test should just write a gray 100x100 image into `basic_test.png`.

use verg::canvas::{Canvas, CanvasDescription};

#[test]
fn basic_test() {
    const WIDTH: usize = 100;
    const HEIGHT: usize = 100;

    let canvas = Canvas::new(CanvasDescription {
        width: WIDTH,
        height: HEIGHT,
        ..Default::default()
    });

    assert_eq!(canvas.desc.width, WIDTH);
    assert_eq!(canvas.desc.height, HEIGHT);

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
