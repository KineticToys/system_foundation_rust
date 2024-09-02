pub struct TopologyEdge {
    waypoints: Vec<(f64, f64)>,
    length: f64,
}

impl TopologyEdge {
    pub fn from_waypoints(waypoints: Vec<(f64, f64)>) -> Self {
        let mut length = 0_f64;

        for i in 1..waypoints.len() {
            let (x1, y1) = waypoints.get(i - 1).unwrap().clone();
            let (x2, y2) = waypoints.get(i).unwrap().clone();
            let dist = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
            length += dist;
        }

        return Self {
            waypoints: waypoints,
            length: length,
        };
    }

    pub fn get_waypoints(&self) -> &Vec<(f64, f64)> {
        return &self.waypoints;
    }

    pub fn get_length(&self) -> f64 {
        return self.length;
    }
}
