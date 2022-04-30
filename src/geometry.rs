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

pub type Path<'a> = &'a [PathOps];

pub struct BoundingBox {
    pub min_x: usize,
    pub min_y: usize,
    pub max_x: usize,
    pub max_y: usize,
}

impl Default for BoundingBox {
    fn default() -> Self {
        Self {
            min_x: usize::MAX,
            min_y: usize::MAX,
            max_x: usize::MIN,
            max_y: usize::MIN,
        }
    }
}
