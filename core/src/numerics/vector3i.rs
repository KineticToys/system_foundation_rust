use std::ops::{Add, Div, Mul, Sub};

use super::{vector::Vector, vector2i::Vector2I};

#[derive(Clone, Copy, Debug)]
pub struct Vector3I {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Vector3I {
    pub fn from_xy(x: i64, y: i64) -> Self {
        return Self {
            x: x,
            y: y,
            z: 0_i64,
        };
    }

    pub fn zero() -> Self {
        return Self {
            x: 0_i64,
            y: 0_i64,
            z: 0_i64,
        };
    }

    pub fn dot(&self, v: &Self) -> i64 {
        return self.x * v.x + self.y + v.y + self.z * v.z;
    }

    pub fn cross(&self, v: &Self) -> Self {
        return Self {
            x: self.y * v.z - self.z * v.y,
            y: -self.x * v.z + self.z * v.x,
            z: self.x * v.y - self.y * v.x,
        };
    }
}

impl Vector for Vector3I {
    fn dimensions(&self) -> usize {
        return 3;
    }

    fn magnitude(&self) -> f64 {
        return ((self.x.pow(2) + self.y.pow(2) + self.z.pow(2)) as f64).sqrt();
    }
}

/// Vector3I + Vector3I
impl Add<Vector3I> for Vector3I {
    type Output = Vector3I;

    fn add(self, rhs: Vector3I) -> Self::Output {
        return Vector3I {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

/// Vector3I + &Vector3I
impl Add<&Vector3I> for Vector3I {
    type Output = Vector3I;

    fn add(self, rhs: &Vector3I) -> Self::Output {
        return Vector3I {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

/// &Vector3I + Vector3I
impl Add<Vector3I> for &Vector3I {
    type Output = Vector3I;

    fn add(self, rhs: Vector3I) -> Self::Output {
        return Vector3I {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

/// &Vector3I + &Vector3I
impl Add<&Vector3I> for &Vector3I {
    type Output = Vector3I;

    fn add(self, rhs: &Vector3I) -> Self::Output {
        return Vector3I {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

/// Vector3I - Vector3I
impl Sub<Vector3I> for Vector3I {
    type Output = Vector3I;

    fn sub(self, rhs: Vector3I) -> Self::Output {
        return Vector3I {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

/// Vector3I - &Vector3I
impl Sub<&Vector3I> for Vector3I {
    type Output = Vector3I;

    fn sub(self, rhs: &Vector3I) -> Self::Output {
        return Vector3I {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

/// &Vector3I - Vector3I
impl Sub<Vector3I> for &Vector3I {
    type Output = Vector3I;

    fn sub(self, rhs: Vector3I) -> Self::Output {
        return Vector3I {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

/// &Vector3I - &Vector3I
impl Sub<&Vector3I> for &Vector3I {
    type Output = Vector3I;

    fn sub(self, rhs: &Vector3I) -> Self::Output {
        return Vector3I {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

/// i64 * Vector3I
impl Mul<Vector3I> for i64 {
    type Output = Vector3I;

    fn mul(self, rhs: Vector3I) -> Self::Output {
        return Vector3I {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        };
    }
}

/// i64 * &Vector3I
impl Mul<&Vector3I> for i64 {
    type Output = Vector3I;

    fn mul(self, rhs: &Vector3I) -> Self::Output {
        return Vector3I {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        };
    }
}

/// Vector3I * i64
impl Mul<i64> for Vector3I {
    type Output = Vector3I;

    fn mul(self, rhs: i64) -> Self::Output {
        return Vector3I {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        };
    }
}

/// &Vector3I * i64
impl Mul<i64> for &Vector3I {
    type Output = Vector3I;

    fn mul(self, rhs: i64) -> Self::Output {
        return Vector3I {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        };
    }
}

/// i32 * Vector3I
impl Mul<Vector3I> for i32 {
    type Output = Vector3I;

    fn mul(self, rhs: Vector3I) -> Self::Output {
        return Vector3I {
            x: self as i64 * rhs.x,
            y: self as i64 * rhs.y,
            z: self as i64 * rhs.z,
        };
    }
}

/// i32 * &Vector3I
impl Mul<&Vector3I> for i32 {
    type Output = Vector3I;

    fn mul(self, rhs: &Vector3I) -> Self::Output {
        return Vector3I {
            x: self as i64 * rhs.x,
            y: self as i64 * rhs.y,
            z: self as i64 * rhs.z,
        };
    }
}

/// Vector3I * i32
impl Mul<i32> for Vector3I {
    type Output = Vector3I;

    fn mul(self, rhs: i32) -> Self::Output {
        return Vector3I {
            x: self.x * rhs as i64,
            y: self.y * rhs as i64,
            z: self.z * rhs as i64,
        };
    }
}

/// &Vector3I * i32
impl Mul<i32> for &Vector3I {
    type Output = Vector3I;

    fn mul(self, rhs: i32) -> Self::Output {
        return Vector3I {
            x: self.x * rhs as i64,
            y: self.y * rhs as i64,
            z: self.z * rhs as i64,
        };
    }
}

/// Vector3I / i64
impl Div<i64> for Vector3I {
    type Output = Vector3I;

    fn div(self, rhs: i64) -> Self::Output {
        return Vector3I {
            x: self.x / rhs as i64,
            y: self.y / rhs as i64,
            z: self.z / rhs as i64,
        };
    }
}

/// &Vector3I / i64
impl Div<i64> for &Vector3I {
    type Output = Vector3I;

    fn div(self, rhs: i64) -> Self::Output {
        return Vector3I {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        };
    }
}

/// Vector3I / i32
impl Div<i32> for Vector3I {
    type Output = Vector3I;

    fn div(self, rhs: i32) -> Self::Output {
        return Vector3I {
            x: self.x / rhs as i64,
            y: self.y / rhs as i64,
            z: self.z / rhs as i64,
        };
    }
}

/// &Vector3I / i32
impl Div<i32> for &Vector3I {
    type Output = Vector3I;

    fn div(self, rhs: i32) -> Self::Output {
        return Vector3I {
            x: self.x / rhs as i64,
            y: self.y / rhs as i64,
            z: self.z / rhs as i64,
        };
    }
}

impl From<Vector2I> for Vector3I {
    fn from(value: Vector2I) -> Self {
        return Vector3I::from_xy(value.x, value.y);
    }
}