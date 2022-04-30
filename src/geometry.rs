pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Copy)]
pub enum PathOps {
    MoveTo { x: f64, y: f64 },
    MoveToRel { x: f64, y: f64 },
    LineTo { x: f64, y: f64 },
    LineToRel { x: f64, y: f64 },
    Close,
}

pub type Path = Vec<PathOps>;
