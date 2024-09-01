pub struct TopologyEdge {
    waypoints: Vec<(f64, f64)>,
    length: f64,
}

impl TopologyEdge {
    pub fn from_waypoints(waypoints: Vec<(f64, f64)>) -> Self {
        let mut length = 0_f64;

        for i in 1..waypoints.len() {
            let (x_prev, y_prev) = waypoints.get(i - 1).unwrap();
            let (x_curr, y_curr) = waypoints.get(i).unwrap();
            let _x_prev = *x_prev as f64;
            let _y_prev = *y_prev as f64;
            let _x_curr = *x_curr as f64;
            let _y_curr = *y_curr as f64;
            let dist = ((_x_curr - _x_prev).powi(2) + (_y_curr - _y_prev).powi(2)).sqrt();
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