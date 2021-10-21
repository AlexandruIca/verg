// Tests in this file should make sure that `VergError` values are returned appropiately.

use verg::{
    canvas::{Canvas, NUM_CHANNELS},
    error_types::VergError,
};

// Not taking into account `NUM_CHANNELS` when creating the canvas
#[test]
fn bad_dimensions() {
    let mut buffer: [f64; 100 * 100] = [0.0f64; 100 * 100];
    let canvas_err = Canvas::from_buffer(&mut buffer, 100, 100).err().unwrap();

    println!("{}", canvas_err);
    assert_eq!(
        canvas_err,
        VergError::BadDimensions {
            expected_width: 100,
            expected_height: 100,
            actual_size: 100 * 100
        }
    );
}

// WIDTH * HEIGHT * NUM_CHANNELS would overflow
#[test]
fn size_too_big() {
    let mut buffer: [f64; 10] = [1.0f64; 10];
    let canvas_err = Canvas::from_buffer(&mut buffer, i64::MAX, i64::MAX)
        .err()
        .unwrap();

    println!("{}", canvas_err);
    assert_eq!(
        canvas_err,
        VergError::SizeTooBig {
            width: i64::MAX,
            height: i64::MAX
        }
    );
}

// WIDTH * HEIGHT * NUM_CHANNELS would result in a negative value
#[test]
fn negative_image_size() {
    let mut buffer: [f64; 10] = [1.0f64; 10];
    let canvas_err = Canvas::from_buffer(&mut buffer, -NUM_CHANNELS, NUM_CHANNELS)
        .err()
        .unwrap();

    assert_eq!(
        canvas_err,
        VergError::NegativeImageSize {
            image_size: -NUM_CHANNELS * NUM_CHANNELS * NUM_CHANNELS
        }
    );
}

// Output buffer does not have the appropiate size
#[test]
fn bad_output_buffer() {
    const BUFFER_SIZE: usize = 10 * 10 * NUM_CHANNELS as usize;
    let mut buffer: [f64; BUFFER_SIZE] = [1.0f64; BUFFER_SIZE];
    let mut output_buffer: [u8; BUFFER_SIZE - 1] = [0u8; BUFFER_SIZE - 1];
    let canvas = Canvas::from_buffer(&mut buffer, 10, 10).unwrap();

    assert_eq!(
        canvas.to_u8(&mut output_buffer).err().unwrap(),
        VergError::BadOutputDimensions {
            buffer_size: output_buffer.len() as i64,
            expected_size: buffer.len() as i64
        }
    );
}
