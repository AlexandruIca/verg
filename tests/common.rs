use sha2::{Digest, Sha256};

// We allow dead code because clippy gives a false positive.
// The constant is used in `implement_test!`.
#[allow(dead_code)]
pub const REFERENCE_HASHES: [(&str, &str); 5] = [
    (
        "basic_test",
        "95AEB28CB13578C558F745AD4DFCE5DF3BCAD3E11C0C9F15077ED3144C6D4D98",
    ),
    (
        "even_odd_fill_test",
        "EFF992CDB334A9EA152DF318A94BB6CFF42B0BC1412F7479DED6C23A0D78518D",
    ),
    (
        "line_test",
        "7A385C20B75FF0C71137820FDCC7899654774DC123517EB7844531381A2C042F",
    ),
    (
        "rect_test",
        "788DE6A896D30D52DE643180A3B0C084D7EA1F1940F79C41C084B607277CC3B2",
    ),
    (
        "triangle_test",
        "765A7406D707AF35C371B3505B0688C51FB11B42BBB5B46233EAA445A2F28675",
    ),
];

pub fn get_hash_for_color_buffer(buffer: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(buffer);
    format!("{:X}", hasher.finalize())
}

#[macro_export]
macro_rules! implement_test {
    ( $($name:ident, $canvas:ident)? | $($path:expr, $fill_style:expr, $fill_rule:expr),* ) => {
        #[test]
        fn $($name)?() {
            let mut canvas = Canvas::new($($canvas)?());
            $(
                canvas.draw_shape($path.to_vec(), $fill_style, $fill_rule);
            )*

            let u8_buffer = canvas.to_u8();

            image::save_buffer(
                format!("{}.png", stringify!($($name)?)),
                u8_buffer.as_slice(),
                canvas.desc.width as u32,
                canvas.desc.height as u32,
                image::ColorType::Rgba8,
            )
            .unwrap();

            {
                let hash = common::get_hash_for_color_buffer(&u8_buffer);
                let mut hash_found = false;

                println!("Hash for `{}`: {}", stringify!($($name)?), hash);

                for (ref_id, ref_hash) in common::REFERENCE_HASHES {
                    if ref_id == stringify!($($name)?) {
                        hash_found = true;
                        assert_eq!(ref_hash, hash);
                        break;
                    }
                }

                if !hash_found {
                    eprintln!(
                        "Hash for test id `{}` not found in `common::REFERENCE_HASHES`!",
                        stringify!($($name)?)
                    );
                    assert!(false);
                }
            }
        }
    }
}
