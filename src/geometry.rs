pub struct Point {
    x: f64,
    y: f64,
}

pub enum Primitive {
    Line { start: Point, end: Point },
}

pub type Shape = Vec<Primitive>;
