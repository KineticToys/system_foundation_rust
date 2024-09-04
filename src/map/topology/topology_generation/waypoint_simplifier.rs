use std::collections::VecDeque;

use crate::math::{
    geometry::geometry_solver::GeometrySolver,
    numerics::{vector2d::Vector2D, vector3d::Vector3D},
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
    ) {
        if end_index - start_index < 2 {
            return;
        }

        let solver = GeometrySolver::new(1e-9);
        let mut max_deviation = 0_f64;
        let mut max_deviation_index: Option<usize> = None;

        // Find the index of point where has maximum deviation on this level.
        for i in (start_index + 1)..=(end_index + 1) {
            let (distance, _) = solver.point_to_line_distance(
                &waypoints.get(i).unwrap().clone().into(),
                &waypoints.get(start_index).unwrap().clone().into(),
                &waypoints.get(end_index).unwrap().clone().into(),
                true,
            );

            if distance > max_deviation && distance > self.allowed_deviation {
                max_deviation = distance;
                max_deviation_index = Some(i);
            }
        }

        // If no deviation point is found, return None.
        // Given interval is approximately straight.
        if max_deviation_index.is_none() {
            return;
        }

        // Find the deviation point on the left side.
        self.simplify_dfs(
            waypoints,
            divisions,
            start_index,
            max_deviation_index.unwrap(),
        );

        // Find the deviation point on the left side.
        self.simplify_dfs(
            waypoints,
            divisions,
            max_deviation_index.unwrap(),
            end_index,
        );

        *divisions.get_mut(max_deviation_index.unwrap()).unwrap() = true;
    }
}
