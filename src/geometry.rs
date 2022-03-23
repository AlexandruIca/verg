use crate::canvas::CanvasDescription;

pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    ///
    /// A whole pixel is considered a `pixel_size` by `pixel_size` matrix.
    ///
    /// This function takes a normal coordinate and places it in a grid of pixels with `pixel_size` precision.
    ///
    pub fn round_to_grid(&self, pixel_size: usize) -> GridPoint {
        let round_on_axis = |axis: f64| -> (i64, i64) {
            let pixel_length = 1_f64 / (pixel_size as f64);
            let cell_index = axis.fract() / pixel_length;

            (axis.trunc() as i64, cell_index as i64)
        };

        GridPoint {
            x: round_on_axis(self.x),
            y: round_on_axis(self.y),
        }
    }
}

#[derive(Clone, Debug)]
pub struct GridPoint {
    pub x: (i64, i64),
    pub y: (i64, i64),
}

impl GridPoint {
    pub fn to_float(&self, pixel_size: usize) -> (f64, f64) {
        let (ix, ifx) = self.x;
        let (iy, ify) = self.y;
        let (x, fx) = (ix as f64, ifx as f64);
        let (y, fy) = (iy as f64, ify as f64);
        let pixel_size = pixel_size as f64;

        (x + fx / pixel_size, y + fy / pixel_size)
    }
}

impl std::cmp::PartialEq for GridPoint {
    fn eq(&self, other: &Self) -> bool {
        self.x.0 == other.x.0 && self.y.0 == other.y.0
    }
}

impl std::fmt::Display for GridPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "(x={},{}; y={},{})",
            self.x.0, self.x.1, self.y.0, self.y.1
        )
    }
}

pub enum PathOps {
    MoveTo { x: f64, y: f64 },
    MoveToRel { x: f64, y: f64 },
    LineTo { x: f64, y: f64 },
    LineToRel { x: f64, y: f64 },
}

pub type Path = Vec<PathOps>;

///
/// Returns the starting and ending coordinates between which we need to iterate, so that
/// we don't miss any lines with which we want to intersect with.
///
pub fn get_bounding_coordinates(
    (x0, y0): (i64, i64),
    (x1, y1): (i64, i64),
    (dir_x, dir_y): (i64, i64),
) -> ((i64, i64), (i64, i64)) {
    let (start_x, end_x) = if dir_x < 0 {
        (x0 + 1, x1 - 1)
    } else {
        (x0 - 1, x1 + 1)
    };

    let (start_y, end_y) = if dir_y < 0 {
        (y0 + 1, y1 - 1)
    } else {
        (y0 - 1, y1 + 1)
    };

    ((start_x, start_y), (end_x, end_y))
}

///
/// https://www.petercollingridge.co.uk/tutorials/computational-geometry/line-line-intersections/
///
/// \returns Line parameters `(t, s)`, `t` is for `start`, `s` is for `end`.
///
pub fn intersect_two_lines(
    start: (&GridPoint, &GridPoint),
    end: (&GridPoint, &GridPoint),
    pixel_size: usize,
) -> (f64, f64) {
    let (x0, x1) = (start.0.x.0, start.1.x.0);

    let (ax, ay) = start.0.to_float(pixel_size);
    let (bx, by) = start.1.to_float(pixel_size);
    let (cx, cy) = end.0.to_float(pixel_size);
    let (dx, dy) = end.1.to_float(pixel_size);

    let (bax, bay) = (bx - ax, by - ay);
    let (dcx, dcy) = (dx - cx, dy - cy);
    let (cax, cay) = (cx - ax, cy - ay);

    let (gradient_a, gradient_b) = (bay / bax, dcy / dcx);

    let s = if (gradient_a - gradient_b).abs() < f64::EPSILON {
        -1.0_f64
    } else {
        (bax * cay - bay * cax) / (bay * dcx - bax * dcy)
    };

    let t = if x0 == x1 {
        (s * dcy + cy - ay) / bay
    } else {
        (s * dcx + cx - ax) / bax
    };

    (t, s)
}

fn line_parameter_is_ok(t: f64) -> bool {
    (0_f64..=1_f64).contains(&t)
}

#[derive(Eq, PartialEq)]
enum IntersectionWith {
    Horizontals,
    Verticals,
}

fn push_new_point(acc: &mut Vec<GridPoint>, point: GridPoint) {
    if let Some(prev) = acc.last() {
        if prev != &point {
            acc.push(point);
        }
    } else {
        acc.push(point);
    }
}

fn index_to_line(
    i: i64,
    dir: &IntersectionWith,
    canvas: &CanvasDescription,
) -> (GridPoint, GridPoint) {
    match dir {
        IntersectionWith::Horizontals => (
            GridPoint {
                x: (0, 0),
                y: (i, 0),
            },
            GridPoint {
                x: (canvas.width as i64, 0),
                y: (i, 0),
            },
        ),
        IntersectionWith::Verticals => (
            GridPoint {
                x: (i, 0),
                y: (0, 0),
            },
            GridPoint {
                x: (i, 0),
                y: (canvas.height as i64, 0),
            },
        ),
    }
}

fn is_horizontal_line(start: &GridPoint, end: &GridPoint) -> bool {
    start.y.0 == end.y.0
}

fn is_vertical_line(start: &GridPoint, end: &GridPoint) -> bool {
    start.x.0 == end.x.0
}

// This includes `b` in `acc` (it's meant to be used iteratively between many points for reuse).
#[allow(clippy::many_single_char_names)]
fn get_intersections_between_two_points(
    a: &GridPoint,
    b: &GridPoint,
    dir: IntersectionWith,
    acc: &mut Vec<GridPoint>,
    canvas: &CanvasDescription,
) {
    let (is_horizontal, is_vertical) = (is_horizontal_line(a, b), is_vertical_line(a, b));

    if is_horizontal && dir == IntersectionWith::Horizontals {
        return acc.push(b.clone());
    }
    if is_vertical && dir == IntersectionWith::Verticals {
        return acc.push(b.clone());
    }

    let (x0, y0) = (a.x.0, a.y.0);
    let (x1, y1) = (b.x.0, b.y.0);

    let dir_x = (x1 - x0).signum();
    let dir_y = (y1 - y0).signum();

    let ((start_x, start_y), (end_x, end_y)) =
        get_bounding_coordinates((x0, y0), (x1, y1), (dir_x, dir_y));

    let (ax, ay) = a.to_float(canvas.pixel_size);
    let (bx, by) = b.to_float(canvas.pixel_size);

    let (start, end, dir_offset) = match dir {
        IntersectionWith::Horizontals => (start_y, end_y, dir_y),
        IntersectionWith::Verticals => (start_x, end_x, dir_x),
    };

    if dir_offset != 0 {
        let mut i = start;
        while i != end {
            let (c, d) = index_to_line(i, &dir, canvas);
            let (t, s) = intersect_two_lines((a, b), (&c, &d), canvas.pixel_size);

            if line_parameter_is_ok(t) && line_parameter_is_ok(s) {
                push_new_point(
                    acc,
                    Point {
                        x: (ax + (bx - ax) * t),
                        y: (ay + (by - ay) * t),
                    }
                    .round_to_grid(canvas.pixel_size),
                );
            }

            i += dir_offset;
        }
    }

    push_new_point(acc, b.clone());
}

pub fn intersect_line_with_grid(
    a: &GridPoint,
    b: &GridPoint,
    canvas: &CanvasDescription,
) -> Vec<GridPoint> {
    let mut on_horizontals = vec![a.clone()];
    let mut on_verticals = vec![a.clone()];

    get_intersections_between_two_points(
        a,
        b,
        IntersectionWith::Horizontals,
        &mut on_horizontals,
        canvas,
    );

    if on_horizontals.is_empty() {
        get_intersections_between_two_points(
            a,
            b,
            IntersectionWith::Verticals,
            &mut on_verticals,
            canvas,
        );
    } else {
        on_horizontals.windows(2).for_each(|points: &[GridPoint]| {
            get_intersections_between_two_points(
                &points[0],
                &points[1],
                IntersectionWith::Verticals,
                &mut on_verticals,
                canvas,
            )
        });
    }

    on_verticals
}

#[test]
fn test_intersections_with_grid() {
    let canvas_desc = CanvasDescription {
        pixel_size: 4,
        width: 600,
        height: 600,
        ..Default::default()
    };

    let data = vec![
        (
            GridPoint {
                x: (25, 1),
                y: (20, 0),
            },
            GridPoint {
                x: (23, 0),
                y: (20, 0),
            },
        ),
        (
            GridPoint {
                x: (25, 1),
                y: (20, 0),
            },
            GridPoint {
                x: (25, 1),
                y: (25, 0),
            },
        ),
        (
            GridPoint {
                x: (25, 1),
                y: (20, 3),
            },
            GridPoint {
                x: (20, 3),
                y: (25, 1),
            },
        ),
        (
            GridPoint {
                x: (20, 3),
                y: (25, 1),
            },
            GridPoint {
                x: (25, 1),
                y: (20, 3),
            },
        ),
        (
            GridPoint {
                x: (20, 1),
                y: (30, 3),
            },
            GridPoint {
                x: (27, 2),
                y: (33, 1),
            },
        ),
        (
            GridPoint {
                x: (27, 2),
                y: (33, 1),
            },
            GridPoint {
                x: (20, 1),
                y: (30, 3),
            },
        ),
    ];

    let mut i = 0;
    for (a, b) in data.iter() {
        let result = intersect_line_with_grid(a, b, &canvas_desc);

        println!("Test {} A={}, B={}:\n", i, a, b);

        for p in result {
            println!("{}", p);
        }

        println!();
        i += 1;
    }
}
