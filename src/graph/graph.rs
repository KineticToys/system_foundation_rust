use std::collections::{HashMap, HashSet, LinkedList};

use super::{edge::Edge, graph_error::GraphError, node::Node};

#[derive(Debug)]
pub struct Graph<TNodeInfo, TEdgeInfo> {
    nodes: HashMap<u32, Node<TNodeInfo>>,
    edges: HashMap<u32, Edge<TEdgeInfo>>,

    allow_cyclic_edges: bool,
    allow_duplicate_edges: bool,

    node_id_alloc: u32,
    edge_id_alloc: u32,
}

impl<TNodeInfo, TEdgeInfo> Graph<TNodeInfo, TEdgeInfo> {
    pub fn new(allow_cyclic_edges: bool, allow_duplicate_edges: bool) -> Self {
        return Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            allow_cyclic_edges: allow_cyclic_edges,
            allow_duplicate_edges: allow_duplicate_edges,
            node_id_alloc: 1_u32,
            edge_id_alloc: 1_u32,
        };
    }

    pub fn from_entities(
        nodes: Vec<(u32, TNodeInfo)>,
        edges: Vec<(u32, (u32, u32), TEdgeInfo)>,
        assume_bidirectional: bool,
    ) -> Result<Self, GraphError> {
        // Check if there are any nodes with duplicate IDs.
        let mut unique_node_ids: HashSet<u32> = HashSet::new();
        let mut max_node_id: u32 = 0;
        for (node_id, _) in nodes.iter() {
            if !unique_node_ids.insert(*node_id) {
                return Err(GraphError::DuplicateNodeId);
            }

            max_node_id = u32::max(max_node_id, *node_id);
        }

        // Check if there are any edges with duplicate IDs.
        let mut unique_edge_ids: HashSet<u32> = HashSet::new();
        let mut max_edge_id: u32 = 0;
        for (edge_id, (n1, n2), _) in edges.iter() {
            if !unique_edge_ids.insert(*edge_id) {
                return Err(GraphError::DuplicateEdgeId);
            }

            max_edge_id = u32::max(max_edge_id, *edge_id);
        }

        // Build list of nodes.
        let mut _nodes: HashMap<u32, Node<TNodeInfo>> = HashMap::new();
        for (node_id, node_info) in nodes {
            _nodes.insert(node_id, Node::new(node_id, node_info));
        }

        // Build list of edges.
        let mut _edges: HashMap<u32, Edge<TEdgeInfo>> = HashMap::new();
        for (edge_id, (n1, n2), edge_info) in edges {
            _edges.insert(
                edge_id,
                Edge::new(edge_id, n1, n2, true, assume_bidirectional, edge_info),
            );
            _nodes.get_mut(&n1).unwrap().add_connection(n2, edge_id);
            _nodes.get_mut(&n2).unwrap().add_connection(n1, edge_id);
        }

        return Ok(Self {
            nodes: _nodes,
            edges: _edges,
            allow_cyclic_edges: false,
            allow_duplicate_edges: false,
            node_id_alloc: max_node_id + 1,
            edge_id_alloc: max_edge_id + 1,
        });
    }

    pub fn from_entities_list(
        nodes: LinkedList<(u32, TNodeInfo)>,
        edges: LinkedList<(u32, (u32, u32), TEdgeInfo)>,
        assume_bidirectional: bool,
    ) -> Result<Self, GraphError> {
        let mut _nodes: Vec<(u32, TNodeInfo)> = Vec::with_capacity(nodes.len());
        let mut _edges: Vec<(u32, (u32, u32), TEdgeInfo)> = Vec::with_capacity(edges.len());

        for (node_id, node_info) in nodes {
            _nodes.push((node_id, node_info));
        }

        for (edge_id, (n1, n2), edge_info) in edges {
            _edges.push((edge_id, (n1, n2), edge_info));
        }

        return Graph::from_entities(_nodes, _edges, assume_bidirectional);
    }

    /// Add node and return its ID.
    pub fn add_node(&mut self, node_info: TNodeInfo) -> u32 {
        let node_id = self.node_id_alloc;
        self.node_id_alloc += 1;
        let node = Node::new(node_id, node_info);
        self.nodes.insert(node_id, node);
        return node_id;
    }

    /// Add edge and return its ID.
    pub fn add_edge(
        &mut self,
        node1_id: u32,
        node2_id: u32,
        edge_info: TEdgeInfo,
    ) -> Result<u32, GraphError> {
        return self.add_directed_edge(node1_id, node2_id, true, true, edge_info);
    }

    pub fn add_directed_edge(
        &mut self,
        node1_id: u32,
        node2_id: u32,
        can_move_forward: bool,
        can_move_backward: bool,
        edge_info: TEdgeInfo,
    ) -> Result<u32, GraphError> {
        if !self.nodes.contains_key(&node1_id) || !self.nodes.contains_key(&node2_id) {
            return Err(GraphError::NoSuchNode);
        }

        let edge_id = self.edge_id_alloc;
        self.edge_id_alloc += 1;
        let edge = Edge::new(
            edge_id,
            node1_id,
            node2_id,
            can_move_forward,
            can_move_backward,
            edge_info,
        );

        self.edges.insert(edge.get_id(), edge);
        self.nodes
            .get_mut(&node1_id)
            .unwrap()
            .add_connection(node2_id, edge_id);
        self.nodes
            .get_mut(&node2_id)
            .unwrap()
            .add_connection(node1_id, edge_id);

        return Ok(edge_id);
    }

    pub fn get_node_count(&self) -> usize {
        return self.nodes.len();
    }

    pub fn get_edge_count(&self) -> usize {
        return self.edges.len();
    }

    pub fn get_nodes(&self) -> &HashMap<u32, Node<TNodeInfo>> {
        return &self.nodes;
    }

    pub fn get_edges(&self) -> &HashMap<u32, Edge<TEdgeInfo>> {
        return &self.edges;
    }

    pub fn get_node_by_id(&self, node_id: &u32) -> Option<&Node<TNodeInfo>> {
        return self.nodes.get(node_id);
    }

    pub fn get_edge_by_id(&self, edge_id: &u32) -> Option<&Edge<TEdgeInfo>> {
        return self.edges.get(edge_id);
    }

    pub fn remove_node(&mut self, node_id: &u32) -> Result<u32, GraphError> {
        // Remove corresponding node.
        let removed_node = match self.nodes.remove(node_id) {
            Some(n) => n,
            None => return Err(GraphError::NoSuchNode),
        };

        // Build list of entities adjacent to removed node.
        let mut rm_list: LinkedList<(u32, u32)> = LinkedList::new();
        for (rm_edge, rm_node) in removed_node.connected_edges().iter() {
            rm_list.push_back((*rm_edge, *rm_node));
        }

        for (rm_edge, rm_node) in rm_list {
            let node = match self.nodes.get_mut(&rm_node) {
                Some(n) => n,
                None => continue,
            };

            node.remove_connection(rm_edge);
            self.edges.remove(&rm_edge);
        }

        return Ok(removed_node.get_id());
    }

    pub fn remove_edge(&mut self, edge_id: &u32) -> Result<u32, GraphError> {
        // Remove corresponding edge.
        let removed_edge = match self.edges.remove(edge_id) {
            Some(e) => e,
            None => return Err(GraphError::NoSuchEdge),
        };

        let n1 = match self.nodes.get_mut(&removed_edge.node1()) {
            Some(n) => n,
            None => return Err(GraphError::NoSuchNode),
        };
        n1.remove_connection(*edge_id);

        let n2 = match self.nodes.get_mut(&removed_edge.node2()) {
            Some(n) => n,
            None => return Err(GraphError::NoSuchNode),
        };
        n2.remove_connection(*edge_id);

        return Ok(removed_edge.get_id());
    }
}
