use crate::{
    graph::{edge::Edge, graph::Graph, node::Node},
    map::topology::{self, topology_edge::TopologyEdge, topology_node::TopologyNode},
    math::numerics::vector2d::Vector2D,
};

type TopologyMap = Graph<TopologyNode, TopologyEdge>;

pub struct TopologyCoordinateConverter {
    cell_size: f64,
    map_image_dim: (usize, usize),
}

impl TopologyCoordinateConverter {
    pub fn new(cell_size: f64, map_image_dim: (usize, usize)) -> Self {
        return Self {
            cell_size: cell_size,
            map_image_dim: map_image_dim,
        };
    }

    pub fn image_to_planar(&self, topology_map: &TopologyMap) -> TopologyMap {
        let mut ret: TopologyMap = TopologyMap::new(false, false);
        let mut nodes: Vec<&Node<TopologyNode>> =
            topology_map.get_nodes().iter().map(|(_, n)| n).collect();
        let mut edges: Vec<&Edge<TopologyEdge>> =
            topology_map.get_edges().iter().map(|(_, e)| e).collect();
        nodes.sort_by(|n1, n2| u32::cmp(&n1.get_id(), &n2.get_id()));
        edges.sort_by(|e1, e2| u32::cmp(&e1.get_id(), &e2.get_id()));

        for node in nodes {
            let new_node_id = ret.add_node(TopologyNode {
                node_type: node.node_info().node_type.clone(),
                position: self.convert_point_image_to_planar(&node.node_info().position),
            });
            assert_eq!(node.get_id(), new_node_id);
        }

        for edge in edges {
            let n1 = edge.node1();
            let n2 = edge.node2();
            let new_edge_id = ret.add_edge(
                n1,
                n2,
                TopologyEdge::from_waypoints(
                    edge.edge_info()
                        .get_waypoints()
                        .iter()
                        .map(|p| self.convert_point_image_to_planar(p))
                        .collect(),
                ),
            ).unwrap();
            assert_eq!(edge.get_id(), new_edge_id);
        }

        return ret;
    }

    fn convert_point_image_to_planar(&self, px: &Vector2D) -> Vector2D {
        let x = self.cell_size * (0.5 + px.x);
        let y = self.cell_size * (self.map_image_dim.1 as f64 - 0.5 - px.y);
        return Vector2D::from_xy(x, y);
    }
}
