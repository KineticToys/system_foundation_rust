use crate::numerics::{vector2d::Vector2D, vector2i::Vector2I};

use super::topology_node_type::TopologyNodeType;

pub struct TopologyNode {
    pub node_type: TopologyNodeType,
    pub position: Vector2D,
}