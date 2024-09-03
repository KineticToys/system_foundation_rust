use crate::math::numerics::{vector2d::Vector2D, vector2i::Vector2I};

#[derive(Clone)]
pub struct TopologyNode {
    pub node_type: TopologyNodeType,
    pub position: Vector2D,
}

#[derive(Clone)]
pub enum TopologyNodeType {
    Island,
    Endpoint,
    Waypoint,
    Intersection,
}
