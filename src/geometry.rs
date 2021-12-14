pub struct Point {
    pub x: f64,
    pub y: f64,
}

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
