use std::ops::{Add, Div, Mul, Sub};

use super::{vector::Vector, vector2i::Vector2I};

#[derive(Clone, Copy, Debug)]
pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}

impl Vector2D {
    pub fn from_xy(x: f64, y: f64) -> Self {
        return Self { x: x, y: y };
    }

    pub fn zero() -> Self {
        return Self { x: 0_f64, y: 0_f64 };
    }

    pub fn dot(&self, v: &Vector2D) -> f64 {
        return self.x * v.x + self.y + v.y;
    }

    pub fn unit_vector(&self) -> Self {
        return self / self.magnitude();
    }
}

impl Vector for Vector2D {
    fn dimensions(&self) -> usize {
        return 2;
    }

    fn magnitude(&self) -> f64 {
        return (self.x.powi(2) + self.y.powi(2)).sqrt();
    }
}

/// Vector2D + Vector2D
impl Add<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn add(self, rhs: Vector2D) -> Self::Output {
        return Vector2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

/// Vector2D + &Vector2D
impl Add<&Vector2D> for Vector2D {
    type Output = Vector2D;

    fn add(self, rhs: &Vector2D) -> Self::Output {
        return Vector2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

/// &Vector2D + Vector2D
impl Add<Vector2D> for &Vector2D {
    type Output = Vector2D;

    fn add(self, rhs: Vector2D) -> Self::Output {
        return Vector2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

/// &Vector2D + &Vector2D
impl Add<&Vector2D> for &Vector2D {
    type Output = Vector2D;

    fn add(self, rhs: &Vector2D) -> Self::Output {
        return Vector2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

/// Vector2D - Vector2D
impl Sub<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn sub(self, rhs: Vector2D) -> Self::Output {
        return Vector2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

/// Vector2D - &Vector2D
impl Sub<&Vector2D> for Vector2D {
    type Output = Vector2D;

    fn sub(self, rhs: &Vector2D) -> Self::Output {
        return Vector2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

/// &Vector2D - Vector2D
impl Sub<Vector2D> for &Vector2D {
    type Output = Vector2D;

    fn sub(self, rhs: Vector2D) -> Self::Output {
        return Vector2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

/// &Vector2D - &Vector2D
impl Sub<&Vector2D> for &Vector2D {
    type Output = Vector2D;

    fn sub(self, rhs: &Vector2D) -> Self::Output {
        return Vector2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

/// f64 * Vector2D
impl Mul<Vector2D> for f64 {
    type Output = Vector2D;

    fn mul(self, rhs: Vector2D) -> Self::Output {
        return Vector2D {
            x: self * rhs.x,
            y: self * rhs.y,
        };
    }
}

/// f64 * &Vector2D
impl Mul<&Vector2D> for f64 {
    type Output = Vector2D;

    fn mul(self, rhs: &Vector2D) -> Self::Output {
        return Vector2D {
            x: self * rhs.x,
            y: self * rhs.y,
        };
    }
}

/// Vector2D * f64
impl Mul<f64> for Vector2D {
    type Output = Vector2D;

    fn mul(self, rhs: f64) -> Self::Output {
        return Vector2D {
            x: self.x * rhs,
            y: self.y * rhs,
        };
    }
}

/// &Vector2D * f64
impl Mul<f64> for &Vector2D {
    type Output = Vector2D;

    fn mul(self, rhs: f64) -> Self::Output {
        return Vector2D {
            x: self.x * rhs,
            y: self.y * rhs,
        };
    }
}

/// Vector2D / f64
impl Div<f64> for Vector2D {
    type Output = Vector2D;

    fn div(self, rhs: f64) -> Self::Output {
        return Vector2D {
            x: self.x / rhs,
            y: self.y / rhs,
        };
    }
}

/// &Vector2D / f64
impl Div<f64> for &Vector2D {
    type Output = Vector2D;

    fn div(self, rhs: f64) -> Self::Output {
        return Vector2D {
            x: self.x / rhs,
            y: self.y / rhs,
        };
    }
}

impl Into<Vector2I> for Vector2D {
    fn into(self) -> Vector2I {
        return Vector2I::from_xy(self.x as i64, self.y as i64);
    }
}

impl Into<Vector2I> for &Vector2D {
    fn into(self) -> Vector2I {
        return Vector2I::from_xy(self.x as i64, self.y as i64);
    }
}