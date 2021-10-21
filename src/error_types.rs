use crate::canvas::NUM_CHANNELS;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum VergError {
    BadDimensions {
        expected_width: i64,
        expected_height: i64,
        actual_size: i64,
    },
    SizeTooBig {
        width: i64,
        height: i64,
    },
    NegativeImageSize {
        image_size: i64,
    },
    BadOutputDimensions {
        buffer_size: i64,
        expected_size: i64,
    },
}

impl fmt::Display for VergError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VergError::BadDimensions {
                expected_width,
                expected_height,
                actual_size,
            } => write!(
                f,
                "Got buffer of size {}, expected it to be large enough to hold {} * w={} * h={} ({})",
                actual_size,
                NUM_CHANNELS,
                expected_width,
                expected_height,
                (expected_width * NUM_CHANNELS).checked_mul(*expected_height).unwrap_or(-1)
            ),
            VergError::SizeTooBig { width, height } => write!(
                f,
                "Multiplying num_channels={} by width={} by height={} results in overflow!",
                NUM_CHANNELS, width, height
            ),
            VergError::NegativeImageSize { image_size } => {
                write!(f, "Image size is negative: {}", image_size)
            }
            VergError::BadOutputDimensions {
                buffer_size,
                expected_size,
            } => write!(
                f,
                "Output buffer is of size {}, it should be of size {}",
                buffer_size, expected_size
            ),
        }
    }
}
