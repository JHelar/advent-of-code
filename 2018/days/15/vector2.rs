use crate::graph::NodeValue;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Vector2(pub isize, pub isize);
impl Vector2 {
    pub fn new(x: isize, y: isize) -> Self {
        Self(x, y)
    }

    pub fn add(&self, other: &Vector2) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl NodeValue for Vector2 {
    fn to_name(&self) -> String {
        format!("{},{}", self.0, self.1)
    }
}