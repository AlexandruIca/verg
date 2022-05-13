use ttf_parser as ttf;
use verg::{
    canvas::{Canvas, CanvasDescription, ViewBox},
    color::{Color, FillRule, FillStyle},
    geometry::{PathOps, Point},
    math::translate,
};

mod common;

///
/// Only used to compute the bounding box of a glyph.
///
#[derive(Debug, Clone, Copy)]
struct BboxOutlineBuilder {
    min_x: f32,
    min_y: f32,
    max_x: f32,
    max_y: f32,
    num_paths: usize,
}

fn fmin3(a: f32, b: f32, c: f32) -> f32 {
    f32::min(f32::min(a, b), c)
}

fn fmax3(a: f32, b: f32, c: f32) -> f32 {
    f32::max(f32::max(a, b), c)
}

impl ttf::OutlineBuilder for BboxOutlineBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        self.min_x = f32::min(self.min_x, x);
        self.min_y = f32::min(self.min_y, y);
        self.max_x = f32::max(self.max_x, x);
        self.max_y = f32::max(self.max_y, y);
        self.num_paths += 1;
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.min_x = f32::min(self.min_x, x);
        self.min_y = f32::min(self.min_y, y);
        self.max_x = f32::max(self.max_x, x);
        self.max_y = f32::max(self.max_y, y);
        self.num_paths += 1;
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.min_x = fmin3(self.min_x, x1, x);
        self.min_y = fmin3(self.min_y, y1, y);
        self.max_x = fmax3(self.max_x, x1, x);
        self.max_y = fmax3(self.max_y, y1, y);
        self.num_paths += 1;
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.min_x = f32::min(fmin3(self.min_x, x1, x2), x);
        self.min_y = f32::min(fmin3(self.min_y, y1, y2), y);
        self.max_x = f32::max(fmax3(self.max_x, x1, x2), x);
        self.max_y = f32::max(fmax3(self.max_y, y1, y2), y);
        self.num_paths += 1;
    }

    fn close(&mut self) {}
}

#[allow(dead_code)]
struct OutlineBuilder {
    width: usize,
    height: usize,
    path_ops: Vec<PathOps>,
    prev_point: Point,
    min_x: f32,
    min_y: f32,
    starting_point: Point,
}

impl OutlineBuilder {
    fn new(bbox: &BboxOutlineBuilder) -> Self {
        let width = (bbox.max_x - bbox.min_x).ceil() as usize + 1;
        let height = (bbox.max_y - bbox.min_y).ceil() as usize + 1;

        Self {
            width,
            height,
            path_ops: Vec::<PathOps>::with_capacity(bbox.num_paths),
            prev_point: Point { x: 0.0, y: 0.0 },
            min_x: bbox.min_x,
            min_y: bbox.min_y,
            starting_point: Point {
                x: f64::MIN,
                y: f64::MIN,
            },
        }
    }
}

impl ttf::OutlineBuilder for OutlineBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        let new_point = Point {
            x: (x - self.min_x) as f64,
            y: self.height as f64 - (y - self.min_y) as f64,
        };
        self.prev_point = new_point;
        self.starting_point = new_point;
        self.path_ops.push(PathOps::MoveTo {
            x: new_point.x,
            y: new_point.y,
        });

        println!("Move to: (x={}, y={})", new_point.x, new_point.y);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        let new_point = Point {
            x: (x - self.min_x) as f64,
            y: self.height as f64 - (y - self.min_y) as f64,
        };
        self.path_ops.push(PathOps::LineTo {
            x: new_point.x,
            y: new_point.y,
        });
        self.prev_point = new_point;

        println!("Line to: (x={}, y={})", new_point.x, new_point.y);
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        let p1 = Point {
            x: (x1 - self.min_x) as f64,
            y: self.height as f64 - (y1 - self.min_y) as f64,
        };
        let p = Point {
            x: (x - self.min_x) as f64,
            y: self.height as f64 - (y - self.min_y) as f64,
        };
        let cp3 = p;
        let cp1 = Point {
            x: self.prev_point.x + (2.0 / 3.0) * (p1.x - self.prev_point.x),
            y: self.prev_point.y + (2.0 / 3.0) * (p1.y - self.prev_point.y),
        };
        let cp2 = Point {
            x: p.x + (2.0 / 3.0) * (p1.x - p.x),
            y: p.y + (2.0 / 3.0) * (p1.y - p.y),
        };
        self.path_ops.push(PathOps::CubicTo {
            x1: cp1.x,
            y1: cp1.y,
            x2: cp2.x,
            y2: cp2.y,
            x3: cp3.x,
            y3: cp3.y,
        });
        self.prev_point = p;

        println!(
            "Quad to: (x1={}, y1={}), (x={}, y={})",
            p1.x, p1.y, p.x, p.y
        );
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        let p1 = Point {
            x: (x1 - self.min_x) as f64,
            y: self.height as f64 - (y1 - self.min_y) as f64,
        };
        let p2 = Point {
            x: (x2 - self.min_x) as f64,
            y: self.height as f64 - (y2 - self.min_y) as f64,
        };
        let p = Point {
            x: (x - self.min_x) as f64,
            y: self.height as f64 - (y - self.min_y) as f64,
        };
        self.path_ops.push(PathOps::CubicTo {
            x1: p1.x,
            y1: p1.y,
            x2: p2.x,
            y2: p2.y,
            x3: p.x,
            y3: p.y,
        });
        self.prev_point = p;

        println!(
            "Cubic to: (x1={}, y1={}), (x2={}, y2={}), (x={}, y={})",
            x1, y1, x2, y2, x, y
        );
    }

    fn close(&mut self) {
        self.path_ops.push(PathOps::LineTo {
            x: self.starting_point.x,
            y: self.starting_point.y,
        });
        self.prev_point = self.starting_point;

        println!("CLOSE");
    }
}

struct TextDescriptor<'a, const N: usize> {
    font_path: &'a str,
    glyphs: [u16; N],
    foreground_color: Color,
    background_color: Color,
}

#[allow(dead_code)]
const FMI: TextDescriptor<3> = TextDescriptor::<3> {
    font_path: "media/JFWilwod.ttf",
    glyphs: [42, 49, 45],
    foreground_color: Color::white(),
    background_color: Color::steel_blue(),
};

#[allow(dead_code)]
const MALAXO: TextDescriptor<6> = TextDescriptor::<6> {
    font_path: "media/JFWilwod.ttf",
    glyphs: [49, 37, 48, 37, 60, 51],
    foreground_color: Color::white(),
    background_color: Color::steel_blue(),
};

#[allow(dead_code)]
const VERG: TextDescriptor<4> = TextDescriptor::<4> {
    font_path: "media/Roboto-MediumItalic.ttf",
    glyphs: [58, 41, 54, 43],
    foreground_color: Color::black(),
    background_color: Color::white(),
};

#[allow(dead_code)]
const BLESS: TextDescriptor<5> = TextDescriptor::<5> {
    font_path: "media/JFWilwod.ttf",
    glyphs: [38, 48, 41, 55, 55],
    foreground_color: Color::white(),
    background_color: Color::crimson(),
};

#[test]
fn font_test() {
    let test = VERG;
    let font_data = std::fs::read(test.font_path).unwrap();
    let face = ttf::Face::from_slice(&font_data, 0).unwrap();

    let mut path_ops = test
        .glyphs
        .iter()
        .map(|_| Vec::<PathOps>::new())
        .collect::<Vec<Vec<PathOps>>>();
    let mut translations = test
        .glyphs
        .iter()
        .map(|_| Point { x: 0.0, y: 0.0 })
        .collect::<Vec<Point>>();
    let mut total_width: f64 = 0.0;
    let mut total_height: f64 = 0.0;

    for (i, &glyph_index) in test.glyphs.iter().enumerate() {
        let glyph_id = ttf::GlyphId(glyph_index);
        let mut bbox_builder = BboxOutlineBuilder {
            min_x: 0.0,
            max_x: 0.0,
            min_y: 0.0,
            max_y: 0.0,
            num_paths: 0,
        };
        let _ = match face.outline_glyph(glyph_id, &mut bbox_builder) {
            Some(v) => v,
            _ => return,
        };
        let mut builder = OutlineBuilder::new(&bbox_builder);
        let _ = match face.outline_glyph(glyph_id, &mut builder) {
            Some(v) => v,
            None => return,
        };

        path_ops[i] = builder.path_ops;
        translations[i].x = total_width;
        translations[i].y = 10.0;
        total_width += bbox_builder.max_x as f64;
        total_height = f64::max(total_height, bbox_builder.max_y as f64);
    }
    total_width += 100.0;
    total_height += 100.0;
    let canvas_desc = CanvasDescription {
        width: total_width as usize,
        height: total_height as usize,
        viewbox: ViewBox {
            x: 0.0,
            y: 0.0,
            width: total_width,
            height: total_height,
        },
        tolerance: 1.0,
        background_color: test.background_color,
    };
    let mut canvas = Canvas::new(canvas_desc);

    for (i, path) in path_ops.iter().enumerate() {
        canvas.draw_shape(
            path.as_slice(),
            FillStyle::Plain(test.foreground_color),
            FillRule::NonZero,
            |p: &Point| translate(p, translations[i].x, translations[i].y),
        );
    }

    let u8_buffer = canvas.to_u8();

    image::save_buffer(
        "font_test.png",
        u8_buffer.as_slice(),
        canvas_desc.width as u32,
        canvas_desc.height as u32,
        image::ColorType::Rgba8,
    )
    .unwrap();

    {
        let hash = common::get_hash_for_color_buffer(&u8_buffer);
        let mut hash_found = false;

        println!("Hash for `{}`: {}", "font_test", hash);

        for (ref_id, ref_hash) in common::REFERENCE_HASHES {
            if ref_id == "font_test" {
                hash_found = true;
                assert_eq!(ref_hash, hash);
                break;
            }
        }

        if !hash_found {
            eprintln!(
                "Hash for test id `{}` not found in `common::REFERENCE_HASHES`!",
                "font_test"
            );
            assert!(false);
        }
    }
}
