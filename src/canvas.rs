use crate::error_types::VergError;
use std::convert::TryFrom;

pub const NUM_CHANNELS: i64 = 4; // RGBA

#[derive(Debug)]
pub struct Canvas<'a> {
    pub buffer: &'a mut [f64],
    pub width: i64,
    pub height: i64,
}

impl<'a> Canvas<'a> {
    pub fn from_buffer(image: &mut [f64], width: i64, height: i64) -> Result<Canvas, VergError> {
        let image_isize = width
            .checked_mul(height)
            .ok_or(VergError::SizeTooBig { width, height })
            .map(|value: i64| NUM_CHANNELS * value)?;

        let image_size = usize::try_from(image_isize).map_err(|_| VergError::NegativeImageSize {
            image_size: image_isize,
        });

        if image.len() != image_size? {
            return Err(VergError::BadDimensions {
                expected_width: width,
                expected_height: height,
                actual_size: image.len() as i64,
            });
        }

        Ok(Canvas {
            buffer: image,
            width,
            height,
        })
    }

    pub fn to_u8(&self, output: &mut [u8]) -> Result<(), VergError> {
        if self.buffer.len() != output.len() {
            return Err(VergError::BadOutputDimensions {
                buffer_size: output.len() as i64,
                expected_size: self.buffer.len() as i64,
            });
        }

        self.buffer.iter().enumerate().for_each(|(index, value)| {
            // https://stackoverflow.com/a/56842762/8622014
            const FACTOR: f64 = (u8::MAX as f64) - f64::EPSILON * 128.0f64;
            output[index] = (*value * FACTOR) as u8;
        });

        Ok(())
    }
}
