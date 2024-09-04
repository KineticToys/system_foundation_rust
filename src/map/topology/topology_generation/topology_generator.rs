use std::collections::HashMap;

use ndarray::Array2;

use crate::graph::graph::Graph;

use super::{
    topology_edge::TopologyEdge, topology_extractor::TopologyExtractor,
    topology_node::TopologyNode, waypoint_simplifier::WaypointSimplifier,
};

type TopologyMap = Graph<TopologyNode, TopologyEdge>;

pub struct TopologyGenerator {
    topology_extractor: TopologyExtractor,
    waypoint_simplifier: WaypointSimplifier,
}

impl TopologyGenerator {
    pub fn new(max_deviation_pixels: f64) -> Self {
        return Self {
            topology_extractor: TopologyExtractor::new(),
            waypoint_simplifier: WaypointSimplifier::new(max_deviation_pixels),
        };
    }

    pub fn generate_topology(&mut self, occupancy_map: &Array2<bool>) -> TopologyMap {
        let temp_map = self.extract_topology(occupancy_map);
        let topology_map = self.simplify_topology(&temp_map);
        return topology_map;
    }

    fn extract_topology(&mut self, occupancy_map: &Array2<bool>) -> TopologyMap {
        return self.topology_extractor.extract(occupancy_map);
    }

    fn simplify_topology(&mut self, temp_map: &TopologyMap) -> TopologyMap {
        let mut old_to_new_id_map: HashMap<u32, u32> = HashMap::new();
        let mut new_to_old_id_map: HashMap<u32, u32> = HashMap::new();
        let mut topology_map: TopologyMap = TopologyMap::new(false, false);

        for (node_id, node) in temp_map.get_nodes().iter() {
            let new_id = topology_map.add_node(node.node_info().clone());
            old_to_new_id_map.insert(node_id.clone(), new_id);
            new_to_old_id_map.insert(new_id, node_id.clone());
        }

        for (edge_id, edge) in temp_map.get_edges().iter() {
            let node1 = temp_map.get_node_by_id(&edge.node1()).unwrap();
            let node2 = temp_map.get_node_by_id(&edge.node2()).unwrap();
            let simplified_waypoints = self
                .waypoint_simplifier
                .simplify(edge.edge_info().get_waypoints());
        }

        todo!();
    }
}
