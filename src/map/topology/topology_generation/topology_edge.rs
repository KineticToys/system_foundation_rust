use crate::math::numerics::{vector::Vector, vector2d::Vector2D};

pub struct TopologyEdge {
    waypoints: Vec<Vector2D>,
    length: f64,
}

impl TopologyEdge {
    pub fn from_waypoints(waypoints: Vec<Vector2D>) -> Self {
        let mut length = 0_f64;

        for i in 1..waypoints.len() {
            let p1 = waypoints.get(i - 1).unwrap().clone();
            let p2 = waypoints.get(i).unwrap().clone();
            let dist = (p2 - p1).magnitude();
            length += dist;
        }

        return Self {
            waypoints: waypoints,
            length: length,
        };
    }

    pub fn get_waypoints(&self) -> &Vec<(Vector2D)> {
        return &self.waypoints;
    }

    pub fn get_length(&self) -> f64 {
        return self.length;
    }
}
