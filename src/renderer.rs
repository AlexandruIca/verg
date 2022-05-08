use crate::{
    canvas::{AccumulationCell, CanvasDescription},
    color::{clamp, Color, FillRule, FillStyle},
    geometry::{BoundingBox, Path, PathOps, Point},
    math::map_viewbox,
};
use std::cmp::Ordering;

///
/// Parameters `src` and `dest`.
///
pub type BlendFunc = fn(&Color, &Color) -> Color;

pub mod blend_func {
    use crate::{color::clamp, renderer::Color};

    pub fn source_over(src: &Color, dest: &Color) -> Color {
        Color {
            r: src.r * src.a + dest.r * dest.a * (1.0 - src.a),
            g: src.g * src.a + dest.g * dest.a * (1.0 - src.a),
            b: src.b * src.a + dest.b * dest.a * (1.0 - src.a),
            a: src.a + dest.a * (1.0 - src.a),
        }
    }

    pub fn destination_over(src: &Color, dest: &Color) -> Color {
        Color {
            r: src.r * src.a * (1.0 - dest.a) + dest.r * dest.a,
            g: src.g * src.a * (1.0 - dest.a) + dest.g * dest.a,
            b: src.b * src.a * (1.0 - dest.a) + dest.b * dest.a,
            a: src.a * (1.0 - dest.a) + dest.a,
        }
    }

    pub fn source_out(src: &Color, dest: &Color) -> Color {
        Color {
            r: src.r * src.a * (1.0 - dest.a),
            g: src.g * src.a * (1.0 - dest.a),
            b: src.b * src.a * (1.0 - dest.a),
            a: src.a * (1.0 - dest.a),
        }
    }

    pub fn destination_out(src: &Color, dest: &Color) -> Color {
        Color {
            r: dest.r * dest.a * (1.0 - src.a),
            g: dest.g * dest.a * (1.0 - src.a),
            b: dest.b * dest.a * (1.0 - src.a),
            a: dest.a * (1.0 - src.a),
        }
    }

    pub fn source_in(src: &Color, dest: &Color) -> Color {
        Color {
            r: src.r * src.a * dest.a,
            g: src.g * src.a * dest.a,
            b: src.b * src.a * dest.a,
            a: src.a * dest.a,
        }
    }

    pub fn destination_in(src: &Color, dest: &Color) -> Color {
        Color {
            r: dest.r * dest.a * src.a,
            g: dest.g * dest.a * src.a,
            b: dest.b * dest.a * src.a,
            a: dest.a * src.a,
        }
    }

    pub fn source_atop(src: &Color, dest: &Color) -> Color {
        Color {
            r: src.r * src.a * dest.a + dest.r * dest.a * (1.0 - src.a),
            g: src.g * src.a * dest.a + dest.g * dest.a * (1.0 - src.a),
            b: src.b * src.a * dest.a + dest.b * dest.a * (1.0 - src.a),
            a: src.a * dest.a + dest.a * (1.0 - src.a),
        }
    }

    pub fn destination_atop(src: &Color, dest: &Color) -> Color {
        Color {
            r: src.r * src.a * (1.0 - dest.a) + dest.r * dest.a * src.a,
            g: src.g * src.a * (1.0 - dest.a) + dest.g * dest.a * src.a,
            b: src.b * src.a * (1.0 - dest.a) + dest.b * dest.a * src.a,
            a: src.a * (1.0 - dest.a) + dest.a * src.a,
        }
    }

    pub fn xor(src: &Color, dest: &Color) -> Color {
        Color {
            r: src.r * src.a * (1.0 - dest.a) + dest.r * dest.a * (1.0 - src.a),
            g: src.g * src.a * (1.0 - dest.a) + dest.g * dest.a * (1.0 - src.a),
            b: src.b * src.a * (1.0 - dest.a) + dest.b * dest.a * (1.0 - src.a),
            a: src.a * (1.0 - dest.a) + dest.a * (1.0 - src.a),
        }
    }

    pub fn clear(_src: &Color, _dest: &Color) -> Color {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
        }
    }

    pub fn source(src: &Color, _dest: &Color) -> Color {
        Color {
            r: src.r,
            g: src.g,
            b: src.b,
            a: src.a,
        }
    }

    pub fn destination(_src: &Color, dest: &Color) -> Color {
        Color {
            r: dest.r,
            g: dest.g,
            b: dest.b,
            a: dest.a,
        }
    }

    pub fn additive(src: &Color, dest: &Color) -> Color {
        Color {
            r: clamp(src.r * src.a + dest.r * dest.a, 0.0, 1.0),
            g: clamp(src.g * src.a + dest.g * dest.a, 0.0, 1.0),
            b: clamp(src.b * src.a + dest.b * dest.a, 0.0, 1.0),
            a: clamp(src.a + dest.a, 0.0, 1.0),
        }
    }
}

pub const NUM_CHANNELS: usize = 4;

fn update_cell(area: f32, cell: &mut AccumulationCell, id: i32) {
    cell.area += area;
    cell.id = id;
}

///
/// # Note
///
/// Expects `start` and `end` to be lexicographically sorted (by `x` and `y`).
///
/// Line drawing algorithm taken from here:
/// - https://medium.com/@raphlinus/inside-the-fastest-font-renderer-in-the-world-75ae5270c445
///
/// id: A number that should differentiate dfferent segments that are part of the same `Path`.
///
pub fn draw_line(
    accumulation_buffer: &mut [AccumulationCell],
    desc: &CanvasDescription,
    start: &Point,
    end: &Point,
    id: i32,
) {
    let p0 = start;
    let p1 = end;

    if (p0.y - p1.y).abs() <= f64::EPSILON {
        return;
    }
    let (dir, p0, p1) = if p0.y < p1.y {
        (1.0, p0, p1)
    } else {
        (-1.0, p1, p0)
    };
    let dxdy = (p1.x - p0.x) / (p1.y - p0.y);
    let mut x = p0.x;
    let y0 = p0.y as usize;

    for y in y0..desc.height.min(p1.y.ceil() as usize) {
        let linestart = y * desc.width;
        let dy = ((y + 1) as f64).min(p1.y) - (y as f64).max(p0.y);
        let xnext = x + dxdy * dy;
        let d = dy * dir;
        let (x0, x1) = if x < xnext { (x, xnext) } else { (xnext, x) };
        let x0floor = x0.floor();
        let x0i = x0floor as i32;
        let x1ceil = x1.ceil();
        let x1i = x1ceil as i32;
        if x1i <= x0i + 1 {
            let xmf = 0.5 * (x + xnext) - x0floor;
            let linestart_x0i = linestart as isize + x0i as isize;
            if linestart_x0i < 0 {
                continue; // oob index
            }
            update_cell(
                (d - d * xmf) as f32,
                &mut accumulation_buffer[linestart_x0i as usize],
                id,
            );
            update_cell(
                (d * xmf) as f32,
                &mut accumulation_buffer[linestart_x0i as usize + 1],
                id,
            );
        } else {
            let s = (x1 - x0).recip();
            let x0f = x0 - x0floor;
            let a0 = 0.5 * s * (1.0 - x0f) * (1.0 - x0f);
            let x1f = x1 - x1ceil + 1.0;
            let am = 0.5 * s * x1f * x1f;
            let linestart_x0i = linestart as isize + x0i as isize;
            if linestart_x0i < 0 {
                continue; // oob index
            }
            update_cell(
                (d * a0) as f32,
                &mut accumulation_buffer[linestart_x0i as usize],
                id,
            );

            if x1i == x0i + 2 {
                update_cell(
                    (d * (1.0 - a0 - am)) as f32,
                    &mut accumulation_buffer[linestart_x0i as usize + 1],
                    id,
                );
            } else {
                let a1 = s * (1.5 - x0f);
                update_cell(
                    (d * (a1 - a0)) as f32,
                    &mut accumulation_buffer[linestart_x0i as usize + 1],
                    id,
                );

                for xi in x0i + 2..x1i - 1 {
                    update_cell(
                        (d * s) as f32,
                        &mut accumulation_buffer[linestart + xi as usize],
                        id,
                    );
                }
                let a2 = a1 + (x1i - x0i - 3) as f64 * s;
                update_cell(
                    (d * (1.0 - a2 - am)) as f32,
                    &mut accumulation_buffer[linestart + (x1i - 1) as usize],
                    id,
                );
            }
            update_cell(
                (d * am) as f32,
                &mut accumulation_buffer[linestart + x1i as usize],
                id,
            );
        }
        x = xnext;
    }
}

pub fn render_path(
    accumulation_buffer: &mut [AccumulationCell],
    desc: &CanvasDescription,
    path: Path,
    transform: impl Fn(&Point) -> Point,
) -> BoundingBox {
    let mut result = BoundingBox::default();
    let mut update_bounds = |x: f64, y: f64| {
        let x = x as usize;
        let y = y as usize;

        result.min_x = usize::min(result.min_x, x);
        result.min_y = usize::min(result.min_y, y);

        result.max_x = usize::max(result.max_x, x);
        result.max_y = usize::max(result.max_y, y);
    };

    let mut start_point = Point { x: 0.0, y: 0.0 };
    let mut start_point_unmaped = Point { x: 0.0, y: 0.0 };
    let mut currently_at = Point { x: 0.0, y: 0.0 };
    let mut currently_at_unmaped = Point { x: 0.0, y: 0.0 };
    let mut line_id = 0;

    for op in path.iter() {
        match op {
            PathOps::MoveTo { x, y } => {
                let p = transform(&Point { x: *x, y: *y });
                let p = map_viewbox(desc, &p);

                currently_at.x = p.x;
                currently_at.y = p.y;

                currently_at_unmaped.x = *x;
                currently_at_unmaped.y = *y;

                start_point.x = p.x;
                start_point.y = p.y;

                start_point_unmaped.x = *x;
                start_point_unmaped.y = *y;

                update_bounds(p.x, p.y);
            }
            PathOps::MoveToRel { x, y } => {
                let p = transform(&Point {
                    x: currently_at_unmaped.x + *x,
                    y: currently_at_unmaped.y + *y,
                });
                let p = map_viewbox(desc, &p);

                currently_at.x = p.x;
                currently_at.y = p.y;

                currently_at_unmaped.x += *x;
                currently_at_unmaped.y += *y;

                start_point.x = currently_at.x;
                start_point.y = currently_at.y;

                start_point_unmaped.x += *x;
                start_point_unmaped.y += *y;

                update_bounds(currently_at.x, currently_at.y);
            }
            PathOps::LineTo { x, y } => {
                let p = transform(&Point { x: *x, y: *y });
                let p = map_viewbox(desc, &p);

                line_id += 1;
                draw_line(accumulation_buffer, desc, &currently_at, &p, line_id);

                currently_at.x = p.x;
                currently_at.y = p.y;

                currently_at_unmaped.x = *x;
                currently_at_unmaped.y = *y;

                update_bounds(p.x, p.y);
            }
            PathOps::LineToRel { x, y } => {
                let p = transform(&Point {
                    x: currently_at_unmaped.x + *x,
                    y: currently_at_unmaped.y + *y,
                });
                let p = map_viewbox(desc, &p);

                line_id += 1;
                draw_line(accumulation_buffer, desc, &currently_at, &p, line_id);

                currently_at.x = p.x;
                currently_at.y = p.y;

                currently_at_unmaped.x += *x;
                currently_at_unmaped.y += *y;

                update_bounds(currently_at.x, currently_at.y);
            }
            PathOps::Close => {
                line_id += 1;
                draw_line(
                    accumulation_buffer,
                    desc,
                    &currently_at,
                    &start_point,
                    line_id,
                );

                currently_at.x = start_point.x;
                currently_at.y = start_point.y;

                currently_at_unmaped.x = start_point_unmaped.x;
                currently_at_unmaped.y = start_point_unmaped.y;
            }
        }
    }

    result
}

fn alpha_fill_even_odd(
    cell: &AccumulationCell,
    prev_cell: &mut AccumulationCell,
    acc: &mut f32,
    filling: &mut f32,
) -> f32 {
    if cell.id > 0 && cell.id != prev_cell.id {
        prev_cell.id = cell.id;
        *filling = -(*filling);
    }

    if cell.id == prev_cell.id {
        *acc += *filling * cell.area.abs();

        if *acc < 0.0 || *acc > 1.0 {
            let is_filling = filling.partial_cmp(&&mut 0.0_f32) != Some(Ordering::Greater);
            *acc = (is_filling as i32) as f32;
        }
    } else {
        let is_filling = filling.partial_cmp(&&mut 0.0_f32) == Some(Ordering::Greater);
        *acc = (is_filling as i32) as f32;
    }

    clamp(acc.abs(), 0.0, 1.0)
}

fn alpha_fill_non_zero(
    cell: &AccumulationCell,
    _prev_cell: &mut AccumulationCell,
    acc: &mut f32,
    _filling: &mut f32,
) -> f32 {
    *acc += cell.area;
    acc.abs()
}

pub fn fill_path(
    accumulation_buffer: &mut [AccumulationCell],
    color_buffer: &mut [f64],
    desc: &CanvasDescription,
    fill_style: FillStyle,
    fill_rule: FillRule,
    bounds: &BoundingBox,
    blend: BlendFunc,
) {
    for y in bounds.min_y..bounds.max_y {
        let mut acc = 0.0_f32;
        let mut filling = -1.0_f32;
        let mut prev_cell = AccumulationCell { area: 0.0, id: 0 };
        let get_alpha = match fill_rule {
            FillRule::NonZero => alpha_fill_non_zero,
            FillRule::EvenOdd => alpha_fill_even_odd,
        };

        for x in bounds.min_x..=bounds.max_x {
            let cell = &mut accumulation_buffer[y * desc.width + x];
            let alpha = get_alpha(cell, &mut prev_cell, &mut acc, &mut filling);
            cell.area = 0.0;
            let pixel_offset: usize = y * desc.width * NUM_CHANNELS + x * NUM_CHANNELS;
            let dest = Color {
                r: color_buffer[pixel_offset],
                g: color_buffer[pixel_offset + 1],
                b: color_buffer[pixel_offset + 2],
                a: color_buffer[pixel_offset + 3],
            };
            let src = match fill_style {
                FillStyle::Plain(Color { r, g, b, a }) => Color {
                    r,
                    g,
                    b,
                    a: f64::min(alpha as f64, a),
                },
            };

            let resulting_color = blend(&src, &dest);

            color_buffer[pixel_offset] = resulting_color.r;
            color_buffer[pixel_offset + 1] = resulting_color.g;
            color_buffer[pixel_offset + 2] = resulting_color.b;
            color_buffer[pixel_offset + 3] = resulting_color.a;
        }
    }
}
