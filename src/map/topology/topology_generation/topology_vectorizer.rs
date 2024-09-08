use std::collections::HashMap;

use crate::{
    graph::graph::Graph,
    map::topology::{
        topology_edge::TopologyEdge,
        topology_generation::waypoint_simplifier::WaypointSimplifier,
        topology_node::{TopologyNode, TopologyNodeType},
    },
};

pub struct TopologyVectorizer;

type TopologyMap = Graph<TopologyNode, TopologyEdge>;

impl TopologyVectorizer {
    pub fn vectorizer(topology_map_temp: &TopologyMap) -> (TopologyMap, Vec<Vec<u32>>) {
        let mut ret: TopologyMap = TopologyMap::new(false, false);
        let mut old_to_new_id_map: HashMap<u32, u32> = HashMap::new();
        let mut new_to_old_id_map: HashMap<u32, u32> = HashMap::new();
        let mut node_groups: Vec<Vec<u32>> = Vec::new();
        let simplifier = WaypointSimplifier::new(1_f64.sqrt());

        for (node_id, node) in topology_map_temp.get_nodes() {
            let new_node_id = ret.add_node(TopologyNode {
                node_type: node.node_info().node_type.clone(),
                position: node.node_info().position.clone(),
            });
            old_to_new_id_map.insert(*node_id, new_node_id);
            new_to_old_id_map.insert(new_node_id, *node_id);
        }

        for (edge_id, edge) in topology_map_temp.get_edges() {
            let node1 = edge.node1();
            let node2 = edge.node2();
            let node1_new = old_to_new_id_map.get(&node1).unwrap().clone();
            let node2_new = old_to_new_id_map.get(&node2).unwrap().clone();
            let mut nodes: Vec<u32> = vec![node1_new];

            let waypoints = edge.edge_info().get_waypoints();
            let simplified_waypoints = simplifier.simplify(&waypoints);

            if simplified_waypoints.len() >= 3 {
                for i in 1..(simplified_waypoints.len() - 1) {
                    let new_node_id = ret.add_node(TopologyNode {
                        node_type: TopologyNodeType::Waypoint,
                        position: simplified_waypoints.get(i).unwrap().clone(),
                    });
                    nodes.push(new_node_id);
                }
            }

            nodes.push(node2_new);

            for i in 1..(nodes.len()) {
                let n1 = nodes.get(i - 1).unwrap().clone();
                let n2 = nodes.get(i).unwrap().clone();
                ret.add_edge(n1, n2, TopologyEdge::from_waypoints(Vec::new()));
            }

            node_groups.push(nodes);
        }

        return (ret, node_groups);
    }
}
