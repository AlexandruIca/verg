pub struct Point {
    pub x: f64,
    pub y: f64,
}

///
/// (position, cell_index)
///
#[derive(Clone)]
pub struct GridPoint {
    pub x: (i64, i64),
    pub y: (i64, i64),
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

pub enum Primitive {
    Line { start: Point, end: Point },
}

pub type Shape = Vec<Primitive>;

// For line intersections: https://www.petercollingridge.co.uk/tutorials/computational-geometry/line-line-intersections/
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

    let (start_y, end_y) = if y_dir < 0 {
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

#[test]
fn test_intersections_with_grid() {}
