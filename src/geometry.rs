use crate::canvas::CanvasDescription;

pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn round_to_grid(&self, pixel_size: usize) -> GridPoint {
        let round_on_axis = |axis: f64| -> (i64, i64) {
            let pixel_length = 1_f64 / (pixel_size as f64);
            let f = axis.fract();
            let cell_index = f / pixel_length;

            return (axis.trunc() as i64, cell_index as i64);
        };

        return GridPoint {
            x: round_on_axis(self.x),
            y: round_on_axis(self.y),
        };
    }
}

///
/// (position, cell_index)
///
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

        return (x + fx / pixel_size, y + fy / pixel_size);
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

pub enum Primitive {
    Line { start: Point, end: Point },
}

pub type Shape = Vec<Primitive>;

pub fn get_line_intersections_with_grid(start: &GridPoint, end: &GridPoint) -> Vec<GridPoint> {
    let mut result = vec![start.clone()];

    let x_dir = (end.x.0 - start.x.0).signum();
    let y_dir = (end.y.0 - start.x.0).signum();

    if x_dir == 0 || y_dir == 0 {
        unimplemented!("Can't handle vertical/horizontal lines yet...");
    }

    let (start_x, end_x) = if x_dir < 0 {
        (start.x.0 + 1, end.x.0 - 1)
    } else {
        (start.x.0 - 1, end.x.0 + 1)
    };

    let (_start_y, _end_y) = if y_dir < 0 {
        (start.y.0 + 1, end.y.0 - 1)
    } else {
        (start.y.0 - 1, end.y.0 + 1)
    };

    let mut x = start_x;
    while x != end_x {
        x += x_dir;
    }

    result.push(end.clone());
    return result;
}

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

    return ((start_x, start_y), (end_x, end_y));
}

/// https://www.petercollingridge.co.uk/tutorials/computational-geometry/line-line-intersections/
///
/// \returns (t, s), `t` is for `start`, `s` is for `end`
///
pub fn intersect_two_lines(
    start: (&GridPoint, &GridPoint),
    end: (&GridPoint, &GridPoint),
    pixel_size: usize,
) -> (f64, f64) {
    let x0 = start.0.x.0;
    let x1 = start.1.x.0;

    let (ax, ay) = start.0.to_float(pixel_size);
    let (bx, by) = start.1.to_float(pixel_size);
    let (cx, cy) = end.0.to_float(pixel_size);
    let (dx, dy) = end.1.to_float(pixel_size);

    let bax = bx - ax;
    let bay = by - ay;
    let dcx = dx - cx;
    let dcy = dy - cy;
    let cax = cx - ax;
    let cay = cy - ay;

    let gradient_a = bay / bax;
    let gradient_b = dcy / dcx;

    let s = if (gradient_a - gradient_b).abs() < f64::EPSILON {
        -1.0f64
    } else {
        (bax * cay - bay * cax) / (bay * dcx - bax * dcy)
    };

    let t = if x0 == x1 {
        (s * dcy + cy - ay) / bay
    } else {
        (s * dcx + cx - ax) / bax
    };

    return (t, s);
}

fn line_parameter_is_ok(t: f64) -> bool {
    t >= 0f64 && t <= 1f64
}

pub fn intersect_line_with_grid(
    a: &GridPoint,
    b: &GridPoint,
    canvas: &CanvasDescription,
) -> Vec<GridPoint> {
    let (x0, _fx0) = a.x;
    let (y0, _fy0) = a.y;
    let (x1, _fx1) = b.x;
    let (y1, _fy1) = b.y;

    let dir_x = (x1 - x0).signum();
    let dir_y = (y1 - y0).signum();

    let ((_, start_y), (_, end_y)) = get_bounding_coordinates((x0, y0), (x1, y1), (dir_x, dir_y));

    let (ax, ay) = a.to_float(canvas.pixel_size);
    let (bx, by) = b.to_float(canvas.pixel_size);

    let mut points_on_horizontals = Vec::<GridPoint>::new();

    // for each horizontal line
    let mut i = start_y;
    while i != end_y {
        if dir_y == 0 {
            break;
        }

        let (c, d) = (
            GridPoint {
                x: (0, 0),
                y: (i, 0),
            },
            GridPoint {
                x: (canvas.width as i64, 0),
                y: (i, 0),
            },
        );

        let (t, s) = intersect_two_lines((a, b), (&c, &d), canvas.pixel_size);

        if line_parameter_is_ok(t) && line_parameter_is_ok(s) {
            points_on_horizontals.push(
                Point {
                    x: (ax + (bx - ax) * t),
                    y: (ay + (by - ay) * t),
                }
                .round_to_grid(canvas.pixel_size),
            );
        }
        i += dir_y;
    }

    if let Some(point) = points_on_horizontals.first() {
        if point != a {
            points_on_horizontals.insert(0, a.clone());
        }
    }
    if let Some(point) = points_on_horizontals.last() {
        if point != b {
            points_on_horizontals.push(b.clone());
        }
    }

    let mut intersections = vec![a.clone()];

    let accumulate_intersections = |acc: &mut Vec<GridPoint>, end: &GridPoint| {
        let start = acc.last().unwrap();
        let ((start_x, end_x), _) =
            get_bounding_coordinates((start.x.0, end.x.0), (start.y.0, end.y.0), (dir_x, dir_y));
        let mut vertical_intersections = Vec::<GridPoint>::new();

        // for each vertical line
        let mut i = start_x;
        while i != end_x {
            if dir_x == 0 {
                break;
            }

            let accumulate_verticals =
                |verticals: &mut Vec<GridPoint>, (v1, v2): (&GridPoint, &GridPoint)| {
                    let (t, s) = intersect_two_lines((start, end), (v1, v2), canvas.pixel_size);
                    let (ax, ay) = start.to_float(canvas.pixel_size);
                    let (bx, by) = end.to_float(canvas.pixel_size);

                    if line_parameter_is_ok(s) && line_parameter_is_ok(t) {
                        let p = Point {
                            x: (ax + (bx - ax) * t),
                            y: (ay + (by - ay) * t),
                        }
                        .round_to_grid(canvas.pixel_size);

                        if p != *start && p != *end {
                            verticals.push(p);
                        }
                    }
                };

            let (v1, v2) = (
                GridPoint {
                    x: (i, 0),
                    y: (0, 0),
                },
                GridPoint {
                    x: (i, 0),
                    y: (canvas.height as i64, 0),
                },
            );

            accumulate_verticals(&mut vertical_intersections, (&v1, &v2));

            i += dir_x;
        }

        if !vertical_intersections.is_empty() {
            if vertical_intersections.last().unwrap() == end {
                acc.extend(vertical_intersections);
            } else {
                vertical_intersections.push(end.clone());
                acc.extend(vertical_intersections);
            }
        } else {
            acc.push(end.clone());
        }
    };

    if points_on_horizontals.is_empty() {
        accumulate_intersections(&mut intersections, b);
    } else {
        for p in points_on_horizontals[1..].into_iter() {
            accumulate_intersections(&mut intersections, p);
        }
    }

    return intersections;
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

        println!("");
        i += 1;
    }
}
