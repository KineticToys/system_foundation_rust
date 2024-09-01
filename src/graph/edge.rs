#[derive(Debug)]
pub struct Edge<TEdgeInfo> {
    id: u32,
    node1: u32,
    node2: u32,
    can_move_forward: bool,
    can_move_backward: bool,
    edge_info: TEdgeInfo,
}

impl<TEdgeInfo> Edge<TEdgeInfo> {
    pub fn new(
        id: u32,
        node1: u32,
        node2: u32,
        can_move_forward: bool,
        can_move_backward: bool,
        edge_info: TEdgeInfo,
    ) -> Self {
        return Self {
            id: id,
            node1: node1,
            node2: node2,
            can_move_forward: can_move_forward,
            can_move_backward: can_move_backward,
            edge_info: edge_info,
        };
    }

    pub fn get_id(&self) -> u32 {
        return self.id;
    }

    pub fn node1(&self) -> u32 {
        return self.node1;
    }

    pub fn node2(&self) -> u32 {
        return self.node2;
    }

    pub fn can_move_forward(&self) -> bool {
        return self.can_move_forward;
    }

    pub fn can_move_backward(&self) -> bool {
        return self.can_move_backward;
    }

    pub fn edge_info(&self) -> &TEdgeInfo {
        return &self.edge_info;
    }

    pub fn edge_info_mut(&mut self) -> &mut TEdgeInfo {
        return &mut self.edge_info;
    }
}
