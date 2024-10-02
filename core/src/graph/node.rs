use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Node<TNodeInfo> {
    id: u32,
    adjacent_nodes: HashMap<u32, HashSet<u32>>,
    connected_edges: HashMap<u32, u32>,
    node_info: TNodeInfo,
}

impl<TNodeInfo> Node<TNodeInfo> {
    pub fn new(id: u32, node_info: TNodeInfo) -> Self {
        return Self {
            id: id,
            adjacent_nodes: HashMap::new(),
            connected_edges: HashMap::new(),
            node_info: node_info,
        };
    }

    pub fn get_id(&self) -> u32 {
        return self.id;
    }

    pub fn degree(&self) -> usize {
        return self.connected_edges.len();
    }

    pub fn adjacent_nodes(&self) -> &HashMap<u32, HashSet<u32>> {
        return &self.adjacent_nodes;
    }

    pub fn connected_edges(&self) -> &HashMap<u32, u32> {
        return &self.connected_edges;
    }

    pub fn node_info(&self) -> &TNodeInfo {
        return &self.node_info;
    }

    pub fn node_info_mut(&mut self) -> &mut TNodeInfo {
        return &mut self.node_info;
    }

    pub fn add_connection(&mut self, node_id: u32, edge_id: u32) -> bool {
        if self.connected_edges.contains_key(&edge_id) {
            return false;
        }

        let mut edge_set: Option<&mut HashSet<u32>> = self.adjacent_nodes.get_mut(&node_id);
        if edge_set.is_none() {
            self.adjacent_nodes.insert(node_id, HashSet::new());
            edge_set = self.adjacent_nodes.get_mut(&node_id);
        }
        edge_set.unwrap().insert(edge_id);

        self.connected_edges.insert(edge_id, node_id);

        return true;
    }

    pub fn remove_connection(&mut self, edge_id: u32) -> bool {
        let node_id = match self.connected_edges.get(&edge_id) {
            Some(id) => *id,
            None => return false,
        };
        self.connected_edges.remove(&edge_id);

        let edge_set = self
            .adjacent_nodes
            .get_mut(&node_id)
            .expect("Node found in connected edges map does not exist in adjacent nodes map.");

        edge_set.remove(&edge_id);
        if edge_set.is_empty() {
            self.adjacent_nodes.remove(&node_id);
        }

        return true;
    }
}
