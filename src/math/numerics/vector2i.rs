use std::ops::{Add, Div, Mul, Sub};

use super::vector::Vector;

#[derive(Clone, Copy, Debug)]
pub struct Vector2I {
    pub x: i64,
    pub y: i64,
}

impl Vector2I {
    pub fn from_xy(x: i64, y: i64) -> Self {
        return Self {
            x: x,
            y: y,
        };
    }

    pub fn zero() -> Self {
        return Self {
            x: 0_i64,
            y: 0_i64,
        };
    }
}

impl Vector for Vector2I {
    fn dimensions(&self) -> usize {
        return 2;
    }

    fn magnitude(&self) -> f64 {
        return ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt();
    }
}

/// Vector2I + Vector2I
impl Add<Vector2I> for Vector2I {
    type Output = Vector2I;

    fn add(self, rhs: Vector2I) -> Self::Output {
        return Vector2I {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

/// Vector2I + &Vector2I
impl Add<&Vector2I> for Vector2I {
    type Output = Vector2I;

    fn add(self, rhs: &Vector2I) -> Self::Output {
        return Vector2I {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

/// &Vector2I + Vector2I
impl Add<Vector2I> for &Vector2I {
    type Output = Vector2I;

    fn add(self, rhs: Vector2I) -> Self::Output {
        return Vector2I {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

/// &Vector2I + &Vector2I
impl Add<&Vector2I> for &Vector2I {
    type Output = Vector2I;

    fn add(self, rhs: &Vector2I) -> Self::Output {
        return Vector2I {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

/// Vector2I - Vector2I
impl Sub<Vector2I> for Vector2I {
    type Output = Vector2I;

    fn sub(self, rhs: Vector2I) -> Self::Output {
        return Vector2I {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

/// Vector2I - &Vector2I
impl Sub<&Vector2I> for Vector2I {
    type Output = Vector2I;

    fn sub(self, rhs: &Vector2I) -> Self::Output {
        return Vector2I {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

/// &Vector2I - Vector2I
impl Sub<Vector2I> for &Vector2I {
    type Output = Vector2I;

    fn sub(self, rhs: Vector2I) -> Self::Output {
        return Vector2I {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

/// &Vector2I - &Vector2I
impl Sub<&Vector2I> for &Vector2I {
    type Output = Vector2I;

    fn sub(self, rhs: &Vector2I) -> Self::Output {
        return Vector2I {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

/// i64 * Vector2I
impl Mul<Vector2I> for i64 {
    type Output = Vector2I;

    fn mul(self, rhs: Vector2I) -> Self::Output {
        return Vector2I {
            x: self * rhs.x,
            y: self * rhs.y,
        };
    }
}

/// i64 * &Vector2I
impl Mul<&Vector2I> for i64 {
    type Output = Vector2I;

    fn mul(self, rhs: &Vector2I) -> Self::Output {
        return Vector2I {
            x: self * rhs.x,
            y: self * rhs.y,
        };
    }
}

/// Vector2I * i64
impl Mul<i64> for Vector2I {
    type Output = Vector2I;

    fn mul(self, rhs: i64) -> Self::Output {
        return Vector2I {
            x: self.x * rhs,
            y: self.y * rhs,
        };
    }
}

/// &Vector2I * i64
impl Mul<i64> for &Vector2I {
    type Output = Vector2I;

    fn mul(self, rhs: i64) -> Self::Output {
        return Vector2I {
            x: self.x * rhs,
            y: self.y * rhs,
        };
    }
}

/// i32 * Vector2I
impl Mul<Vector2I> for i32 {
    type Output = Vector2I;

    fn mul(self, rhs: Vector2I) -> Self::Output {
        return Vector2I {
            x: self as i64 * rhs.x,
            y: self as i64 * rhs.y,
        };
    }
}

/// i32 * &Vector2I
impl Mul<&Vector2I> for i32 {
    type Output = Vector2I;

    fn mul(self, rhs: &Vector2I) -> Self::Output {
        return Vector2I {
            x: self as i64 * rhs.x,
            y: self as i64 * rhs.y,
        };
    }
}

/// Vector2I * i32
impl Mul<i32> for Vector2I {
    type Output = Vector2I;

    fn mul(self, rhs: i32) -> Self::Output {
        return Vector2I {
            x: self.x * rhs as i64,
            y: self.y * rhs as i64,
        };
    }
}

/// &Vector2I * i32
impl Mul<i32> for &Vector2I {
    type Output = Vector2I;

    fn mul(self, rhs: i32) -> Self::Output {
        return Vector2I {
            x: self.x * rhs as i64,
            y: self.y * rhs as i64,
        };
    }
}

/// Vector2I / i64
impl Div<i64> for Vector2I {
    type Output = Vector2I;

    fn div(self, rhs: i64) -> Self::Output {
        return Vector2I {
            x: self.x / rhs as i64,
            y: self.y / rhs as i64,
        };
    }
}

/// &Vector2I / i64
impl Div<i64> for &Vector2I {
    type Output = Vector2I;

    fn div(self, rhs: i64) -> Self::Output {
        return Vector2I {
            x: self.x / rhs,
            y: self.y / rhs,
        };
    }
}

/// Vector2I / i32
impl Div<i32> for Vector2I {
    type Output = Vector2I;

    fn div(self, rhs: i32) -> Self::Output {
        return Vector2I {
            x: self.x / rhs as i64,
            y: self.y / rhs as i64,
        };
    }
}

/// &Vector2I / i32
impl Div<i32> for &Vector2I {
    type Output = Vector2I;

    fn div(self, rhs: i32) -> Self::Output {
        return Vector2I {
            x: self.x / rhs as i64,
            y: self.y / rhs as i64,
        };
    }
}
