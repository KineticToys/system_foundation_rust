use std::{cmp::Ordering, collections::LinkedList, iter::zip};

use crate::math::numerics::{vector2d::Vector2D, vector3d::Vector3D};

pub fn get_convex_hull(points: &[Vector2D]) -> Result<Vec<Vector2D>, ()> {
    if points.len() < 3 {
        return Err(());
    }

    let mut pivot_point = points.get(0).unwrap();
    let mut pivot_point_index: usize = 0;

    for (i, point) in zip(0..points.len(), points.iter()) {
        if point.y < pivot_point.y {
            pivot_point = point;
            pivot_point_index = i;
        }
    }

    // Compute the angle of the line connecting each point with the pivot.
    // The resultant list does NOT contain the pivot.
    let mut point_angles =
        Vec::from_iter(zip(0..points.len(), points.iter()).filter_map(|(idx, p)| {
            if idx == pivot_point_index {
                None
            } else {
                Some((
                    f64::atan2(p.y - pivot_point.y, p.x - pivot_point.x),
                    Vector3D::from_xy(p.x, p.y),
                ))
            }
        }));
    point_angles.sort_by(|a, b| {
        if a.0 < b.0 {
            return Ordering::Less;
        } else if a.0 > b.0 {
            return Ordering::Greater;
        } else {
            return Ordering::Equal;
        }
    });
    let sorted_points = Vec::from_iter(point_angles.into_iter().map(|(_, p)| p));

    let mut stack: LinkedList<Vector3D> = LinkedList::new();
    stack.push_back(Vector3D {
        x: pivot_point.x,
        y: pivot_point.y,
        z: 0_f64,
    });

    for (it, point) in zip(0..sorted_points.len(), sorted_points.into_iter()) {
        if it == 0 {
            stack.push_back(point);
            continue;
        }

        loop {
            let p1 = stack.iter().nth_back(1).unwrap();
            let p2 = stack.iter().nth_back(0).unwrap();
            let p3 = &point;
            let u12 = (p2 - p1).unit_vector();
            let u13 = (p3 - p1).unit_vector();
            let cross = Vector3D::cross(&u12, &u13).z;

            if cross > 0_f64 {
                stack.push_back(point);
                break;
            } else {
                stack.pop_back();
            }
        }
    }

    return Ok(Vec::from_iter(
        stack.into_iter().map(|v| Vector2D { x: v.x, y: v.y }),
    ));
}
