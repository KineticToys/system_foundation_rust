use crate::math::numerics::{vector::Vector, vector2d::Vector2D, vector3d::Vector3D};

pub struct GeometrySolver {
    accuracy: f64,
}

impl GeometrySolver {
    pub fn new(accuracy: f64) -> Self {
        return Self {
            accuracy: accuracy,
        };
    }

    /// Compute distance from point to line.
    /// Returns pair of minimum distance and that position.
    pub fn point_to_line_distance(
        &self,
        point: &Vector3D,
        line_point_2: &Vector3D,
        line_point_1: &Vector3D,
        as_line_segment: bool,
    ) -> (f64, Vector3D) {
        let v12 = line_point_2 - line_point_1;
        let d12 = v12.magnitude();
        let u12 = v12.unit_vector();
        let v1p = point - line_point_1;
        let v1o = u12 * u12.dot(&v1p);
        let d1o = u12.dot(&v1o);

        if !as_line_segment || d1o >= 0_f64 && d1o <= d12 {
            let vpo = v1o - v1p;
            return (vpo.magnitude(), line_point_1 + v1o);
        } else if d1o < 0_f64 {
            return (v1p.magnitude(), line_point_1.clone());
        } else {
            return ((line_point_2 - point).magnitude(), line_point_2.clone());
        }
    }
}
