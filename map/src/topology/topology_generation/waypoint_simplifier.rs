use std::collections::VecDeque;

use crate::math::{
    geometry::geometry_solver::GeometrySolver,
    numerics::{vector::Vector, vector2d::Vector2D, vector3d::Vector3D},
};

pub struct WaypointSimplifier {
    allowed_deviation: f64,
}

impl WaypointSimplifier {
    pub fn new(max_deviation: f64) -> Self {
        return Self {
            allowed_deviation: max_deviation,
        };
    }

    pub fn simplify(&self, waypoints: &Vec<Vector2D>) -> Vec<Vector2D> {
        if waypoints.len() < 3 {
            return waypoints.iter().map(|p| p.clone()).collect();
        }

        let mut divisions: Vec<bool> = vec![false; waypoints.len()];
        *divisions.first_mut().unwrap() = true;
        *divisions.last_mut().unwrap() = true;
        self.simplify_dfs(waypoints, &mut divisions, 0, waypoints.len() - 1);

        let simplified: Vec<Vector2D> = waypoints
            .iter()
            .enumerate()
            .filter(|(i, _)| *divisions.get(*i).unwrap())
            .map(|(i, p)| p.clone())
            .collect();
        return simplified;
    }

    fn simplify_dfs(
        &self,
        waypoints: &Vec<Vector2D>,
        divisions: &mut Vec<bool>,
        start_index: usize,
        end_index: usize,
    ) -> Option<usize> {
        if end_index - start_index < 2 {
            return None;
        }

        let (max_deviation_index, max_deviation) = match self.find_max_deviation(waypoints, start_index, end_index) {
            Some(p) => p,
            None => return None,
        };

        // Find the deviation point on the left side.
        let left_dev = self.simplify_dfs(
            waypoints,
            divisions,
            start_index,
            max_deviation_index,
        );

        // Find the deviation point on the left side.
        let right_dev = self.simplify_dfs(
            waypoints,
            divisions,
            max_deviation_index,
            end_index,
        );

        if left_dev.is_some() && right_dev.is_some() {}

        *divisions.get_mut(max_deviation_index).unwrap() = true;
        return Some(max_deviation_index);
    }

    fn find_max_deviation(
        &self,
        points: &Vec<Vector2D>,
        start_index: usize,
        end_index: usize,
    ) -> Option<(usize, f64)> {
        let solver = GeometrySolver::new(1e-9);
        let mut max_deviation_pair: Option<(usize, f64)> = None;
        let mut max_deviation = 0_f64;
        let mut max_deviation_index: Option<usize> = None;

        for i in (start_index + 1)..=(end_index - 1) {
            let point = points.get(i).unwrap().clone();
            let start_point = points.get(start_index).unwrap().clone();
            let end_point = points.get(end_index).unwrap().clone();

            let distance = match &start_point == &end_point {
                true => (point - start_point).magnitude(),
                false => {
                    let (dist, _) = solver.point_to_line_distance(
                        &point.into(),
                        &points.get(start_index).unwrap().clone().into(),
                        &points.get(end_index).unwrap().clone().into(),
                        true,
                    );
                    dist
                }
            };

            if distance > max_deviation && distance > self.allowed_deviation {
                max_deviation = distance;
                max_deviation_index = Some(i);
            }
        }

        if max_deviation_index.is_some() {
            return Some((max_deviation_index.unwrap(), max_deviation));
        } else {
            return None;
        }
    }
}
