use std::fmt::Display;

pub const UP: Vector2 = Vector2(0, -1);
pub const RIGHT: Vector2 = Vector2(1, 0);
pub const DOWN: Vector2 = Vector2(0, 1);
pub const LEFT: Vector2 = Vector2(-1, 0);

#[derive(Debug, Clone, Copy, Hash)]
pub struct Vector2(pub isize, pub isize);
impl Vector2 {
    pub fn zero() -> Self {
        Self(0, 0)
    }

    pub fn new(x: isize, y: isize) -> Self {
        Self(x, y)
    }

    pub fn add(&self, other: &Vector2) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }

    pub fn sub(&self, other: &Vector2) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }

    pub fn add_scalar(&self, scalar: isize) -> Self {
        Self(self.0 + scalar, self.1 + scalar)
    }

    pub fn mul(&self, other: &Vector2) -> Self {
        Self(self.0 * other.0, self.1 * other.1)
    }

    pub fn mul_scalar(&self, scalar: isize) -> Self {
        Self(self.0 * scalar, self.1 * scalar)
    }

    pub fn div_scalar(&self, scalar: isize) -> Self {
        Self(self.0 / scalar, self.1 / scalar)
    }

    pub fn manhattan_distance(&self, other: &Vector2) -> isize {
        (self.0 - other.1).abs() + (self.1 - other.0).abs()
    }

    pub fn chebyshev_distance(&self, other: &Self) -> isize {
        (other.0 - self.0).max(other.1 - self.1)
    }

    pub fn rot_right_90(&self) -> Self {
        Self(self.1 * -1, self.0)
    }

    fn gcd(&self) -> isize {
        let mut a = self.0.abs();
        let mut b = self.1.abs();
        loop {
            let res = a % b;
            if res == 0 {
                break;
            } else {
                a = b;
                b = res
            }
        }
    
        b
    }

    pub fn get_direction(&self, other: &Self) -> Self {
        Self(other.0 - self.0, other.1 - self.1)
    }

    pub fn normalize(&self) -> Self {
        let g = self.gcd();
        self.div_scalar(g)
    }
}

impl Display for Vector2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

impl PartialEq for Vector2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Eq for Vector2 {
    
}