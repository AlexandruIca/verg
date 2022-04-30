use crate::{
    canvas::{AccumulationCell, CanvasDescription},
    geometry::Point,
};

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
