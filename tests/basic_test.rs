// This test should just write a gray 100x100 image into `basic_test.png`.

use verg::canvas::{Canvas, NUM_CHANNELS};

#[test]
fn basic_test() {
    const WIDTH: i64 = 100;
    const HEIGHT: i64 = 100;
    const BUFFER_SIZE: usize = (WIDTH * HEIGHT * NUM_CHANNELS) as usize;
    let mut buffer: [f64; BUFFER_SIZE] = [0.5f64; BUFFER_SIZE];

    let canvas = Canvas::from_buffer(&mut buffer, WIDTH, HEIGHT).unwrap();
    assert_eq!(canvas.width, WIDTH);
    assert_eq!(canvas.height, HEIGHT);

    let mut u8_buffer: [u8; BUFFER_SIZE] = [0u8; BUFFER_SIZE];
    canvas.to_u8(&mut u8_buffer).unwrap();

    image::save_buffer(
        "basic_test.png",
        &u8_buffer,
        WIDTH as u32,
        HEIGHT as u32,
        image::ColorType::Rgba8,
    )
    .unwrap();
}
