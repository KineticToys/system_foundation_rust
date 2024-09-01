use std::ops::{Add, Div, Mul, Sub};

use super::vector::Vector;

#[derive(Clone, Copy, Debug)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3D {
    pub fn from_xy(x: f64, y: f64) -> Self {
        return Self {
            x: x,
            y: y,
            z: 0_f64,
        };
    }

    pub fn zero() -> Self {
        return Self {
            x: 0_f64,
            y: 0_f64,
            z: 0_f64,
        };
    }

    pub fn dot(&self, v: &Vector3D) -> f64 {
        return self.x * v.x + self.y + v.y + self.z * v.z;
    }

    pub fn cross(&self, v: &Vector3D) -> Self {
        return Self {
            x: self.y * v.z - self.z * v.y,
            y: -self.x * v.z + self.z * v.x,
            z: self.x * v.y - self.y * v.x,
        };
    }

    pub fn unit_vector(&self) -> Self {
        return self / self.magnitude();
    }
}

impl Vector for Vector3D {
    fn dimensions(&self) -> usize {
        return 3;
    }

    fn magnitude(&self) -> f64 {
        return (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt();
    }
}

/// Vector3D + Vector3D
impl Add<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn add(self, rhs: Vector3D) -> Self::Output {
        return Vector3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

/// Vector3D + &Vector3D
impl Add<&Vector3D> for Vector3D {
    type Output = Vector3D;

    fn add(self, rhs: &Vector3D) -> Self::Output {
        return Vector3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

/// &Vector3D + Vector3D
impl Add<Vector3D> for &Vector3D {
    type Output = Vector3D;

    fn add(self, rhs: Vector3D) -> Self::Output {
        return Vector3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

/// &Vector3D + &Vector3D
impl Add<&Vector3D> for &Vector3D {
    type Output = Vector3D;

    fn add(self, rhs: &Vector3D) -> Self::Output {
        return Vector3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

/// Vector3D - Vector3D
impl Sub<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn sub(self, rhs: Vector3D) -> Self::Output {
        return Vector3D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

/// Vector3D - &Vector3D
impl Sub<&Vector3D> for Vector3D {
    type Output = Vector3D;

    fn sub(self, rhs: &Vector3D) -> Self::Output {
        return Vector3D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

/// &Vector3D - Vector3D
impl Sub<Vector3D> for &Vector3D {
    type Output = Vector3D;

    fn sub(self, rhs: Vector3D) -> Self::Output {
        return Vector3D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

/// &Vector3D - &Vector3D
impl Sub<&Vector3D> for &Vector3D {
    type Output = Vector3D;

    fn sub(self, rhs: &Vector3D) -> Self::Output {
        return Vector3D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

/// f64 * Vector3D
impl Mul<Vector3D> for f64 {
    type Output = Vector3D;

    fn mul(self, rhs: Vector3D) -> Self::Output {
        return Vector3D {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        };
    }
}

/// f64 * &Vector3D
impl Mul<&Vector3D> for f64 {
    type Output = Vector3D;

    fn mul(self, rhs: &Vector3D) -> Self::Output {
        return Vector3D {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        };
    }
}

/// Vector3D * f64
impl Mul<f64> for Vector3D {
    type Output = Vector3D;

    fn mul(self, rhs: f64) -> Self::Output {
        return Vector3D {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        };
    }
}

/// &Vector3D * f64
impl Mul<f64> for &Vector3D {
    type Output = Vector3D;

    fn mul(self, rhs: f64) -> Self::Output {
        return Vector3D {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        };
    }
}

/// Vector3D / f64
impl Div<f64> for Vector3D {
    type Output = Vector3D;

    fn div(self, rhs: f64) -> Self::Output {
        return Vector3D {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        };
    }
}

/// &Vector3D / f64
impl Div<f64> for &Vector3D {
    type Output = Vector3D;

    fn div(self, rhs: f64) -> Self::Output {
        return Vector3D {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        };
    }
}
