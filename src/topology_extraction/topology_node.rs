use crate::numerics::{vector2d::Vector2D, vector2i::Vector2I};

pub struct TopologyNode {
    pub node_type: TopologyNodeType,
    pub position: Vector2D,
}

pub enum TopologyNodeType {
    Island,
    Endpoint,
    Waypoint,
    Intersection,
}
