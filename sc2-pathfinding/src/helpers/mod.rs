/// A 2D point with floating-point coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2 {
    pub x: f32,
    pub y: f32,
}

impl Point2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Returns the midpoint between `self` and `other`.
    pub fn midpoint(self, other: Self) -> Self {
        Self {
            x: (self.x + other.x) / 2.0,
            y: (self.y + other.y) / 2.0,
        }
    }
}

impl From<(f32, f32)> for Point2 {
    fn from((x, y): (f32, f32)) -> Self {
        Self { x, y }
    }
}

impl From<Point2> for (f32, f32) {
    fn from(p: Point2) -> Self {
        (p.x, p.y)
    }
}

pub fn round_point2(point: (f32, f32)) -> (usize, usize) {
    let x = point.0.round() as usize;
    let y = point.1.round() as usize;
    (x, y)
}

pub fn point2_f32(point: (usize, usize)) -> (f32, f32) {
    let x = point.0 as f32;
    let y = point.1 as f32;
    (x, y)
}
