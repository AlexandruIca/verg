use sha2::{Digest, Sha256};
use verg::canvas::Canvas;
use verg::color::Color;
use verg::renderer::blend_func;

// We allow dead code because clippy gives a false positive.
// The constant is used in `implement_test!`.
#[allow(dead_code)]
pub const REFERENCE_HASHES: [(&str, &str); 10] = [
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
        "B63BD4212971997150E7FE594444F58C539111AB72363D7C584C25DA7DE692CE",
    ),
    (
        "rect_test",
        "788DE6A896D30D52DE643180A3B0C084D7EA1F1940F79C41C084B607277CC3B2",
    ),
    (
        "triangle_test",
        "DB57D40754C6871D66B23D1CA2A750791303347136EE35BE680A0FD2684289D1",
    ),
    (
        "alpha_blending_test",
        "6C9B6E7943B889530E2CDC9BAA64FC70551AFD233A2E85C4E448DCD261AA0832",
    ),
    (
        "affine_transforms_test",
        "72D232FA2940A3ED66F3465073088EC35989131664B18888D5DBAB8C725226EE",
    ),
    (
        "curve_test",
        "7D79ED1BE7F16175DF64AC8DB90A35F8ADEE807A2E5493BA20D270ADBAA47704",
    ),
    (
        "font_test",
        "F2B2DC85EB145FD18896CED3F73F07AC851CDF267464ACD8390D3E974A2FC727",
    ),
    (
        "gradients_test",
        "5BAEE5895D31B42D8919ACC944915D3A087235F1DD520522D6AF6FFBE44DF6BE",
    ),
];

pub fn get_hash_for_color_buffer(buffer: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(buffer);
    format!("{:X}", hasher.finalize())
}

// Another false positive, this function is used in a lot of tests.
#[allow(dead_code)]
pub fn default_blending(src: &Color, dest: &Color) -> Color {
    blend_func::source_over(src, dest)
}

#[allow(dead_code)]
pub fn default_callback(_canvas: &mut Canvas) {}

#[macro_export]
macro_rules! implement_test {
    ( $($name:ident, $canvas:ident, $custom:ident)? | $($path:expr, $fill_style:expr, $fill_rule:expr, $blend:ident),* ) => {
        #[test]
        fn $($name)?() {
            let _transform = |p: &Point| -> Point { *p };
            let mut canvas = Canvas::new($($canvas)?());
            $(
                canvas.set_blending_function($blend);
                canvas.draw_shape(&($path), $fill_style, $fill_rule, _transform);
            )*

            $(
                $custom(&mut canvas);
            )?

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
