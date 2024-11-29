use crate::graph::NodeValue;

pub const UP: Vector2 = Vector2(0, -1);
pub const DOWN: Vector2 = Vector2(0, 1);
pub const LEFT: Vector2 = Vector2(-1, 0);
pub const RIGHT: Vector2 = Vector2(1, 0);

#[derive(Debug, Clone, Copy, Hash)]
pub struct Vector2(pub isize, pub isize);
impl Vector2 {
    pub fn new(x: isize, y: isize) -> Self {
        Self(x, y)
    }

    pub fn add(&self, other: &Vector2) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }

    pub fn manhattan_distance(&self, other: &Vector2) -> isize {
        (self.0 - other.1).abs() + (self.1 - other.0).abs()
    }
}

impl NodeValue for Vector2 {
    fn to_name(&self) -> String {
        format!("{},{}", self.0, self.1)
    }
}

impl PartialEq for Vector2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Eq for Vector2 {}