use std::collections::{HashSet, VecDeque};

use image::ColorType;
use ndarray::Array2;

use crate::{
    algorithms::zhang_suen_thinning::{
        self,
        zhang_suen_thinning_algorithm::{self, ZhangSuenThinningAlgorithm},
    },
    graph::graph::Graph,
    map::{
        grid::grid_map::{GridMap, GridMapCellState},
        topology::{
            topology_edge::TopologyEdge,
            topology_node::{TopologyNode, TopologyNodeType},
        },
    },
    math::numerics::{vector2d::Vector2D, vector2i::Vector2I},
};

type TopologyMap = Graph<TopologyNode, TopologyEdge>;

static GRID_OFFSETS_RIM: [[isize; 2]; 8] = [
    [0, -1],
    [1, -1],
    [1, 0],
    [1, 1],
    [0, 1],
    [-1, 1],
    [-1, 0],
    [-1, -1],
];

pub struct TopologyExtractor {}

impl TopologyExtractor {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn extract(&self, grid_map: &GridMap) -> TopologyMap {
        let mut thinning = ZhangSuenThinningAlgorithm::new();
        let occupancy_map: Array2<bool> =
            grid_map.map(|cell| *cell.state() == GridMapCellState::Vacant);
        let thinned_occupancy_map: Array2<bool> = thinning.run(&occupancy_map);
        let mut topology_map: TopologyMap = Graph::new(false, false);
        let mut bfs_queue: VecDeque<BfsData> = VecDeque::new();

        self.find_nodes(&thinned_occupancy_map, &mut topology_map, &mut bfs_queue);
        self.find_edges(&thinned_occupancy_map, &mut topology_map, &mut bfs_queue);
        return topology_map;
    }

    fn find_nodes(
        &self,
        thinned_occupancy_map: &Array2<bool>,
        topology_map: &mut TopologyMap,
        bfs_queue: &mut VecDeque<BfsData>,
    ) {
        let (map_height, map_width) = thinned_occupancy_map.dim();

        for x in 1..(map_width - 1) {
            for y in 1..(map_height - 1) {
                if let Some(false) = thinned_occupancy_map.get((y, x)) {
                    continue;
                }

                let score = TopologyExtractor::compute_pixel_score(thinned_occupancy_map, x, y);
                if score <= 1 {
                    let node_id = topology_map.add_node(TopologyNode {
                        node_type: TopologyNodeType::Endpoint,
                        position: Vector2D::from_xy(x as f64, y as f64),
                    });
                    println!("Node {}: ({}, {}) => Endpoint", node_id, x, y);
                    bfs_queue.push_back(BfsData {
                        pos: (x, y),
                        prev_pos: (x, y),
                        root_node: node_id,
                    });
                } else if score >= 3 {
                    let node_id = topology_map.add_node(TopologyNode {
                        node_type: TopologyNodeType::Intersection,
                        position: Vector2D::from_xy(x as f64, y as f64),
                    });
                    println!("Node {}: ({}, {}) => Intersection", node_id, x, y);
                    bfs_queue.push_back(BfsData {
                        pos: (x, y),
                        prev_pos: (x, y),
                        root_node: node_id,
                    })
                }
            }
        }
    }

    fn find_edges(
        &self,
        thinned_occupancy_map: &Array2<bool>,
        topology_map: &mut TopologyMap,
        bfs_queue: &mut VecDeque<BfsData>,
    ) {
        let (map_height, map_width) = thinned_occupancy_map.dim();
        let mut exploration_map: Array2<ExplorationData> =
            Array2::from_shape_fn((map_height, map_width), |(y, x)| ExplorationData {
                cell_state: CellState::Unvisited,
                root_node: None,
                pos: (x, y),
                prev_pos: (x, y),
            });

        while !bfs_queue.is_empty() {
            let data = bfs_queue.pop_front().unwrap();
            let pos = data.pos;

            match exploration_map.get((pos.1, pos.0)) {
                Some(_) => {}
                None => println!("{:?}", pos),
            };

            match exploration_map.get((pos.1, pos.0)).unwrap().cell_state {
                CellState::Merged => continue,
                CellState::Visited => {
                    let this_prev_pos = data.prev_pos;
                    let other_prev_pos = exploration_map.get((pos.1, pos.0)).unwrap().prev_pos;
                    TopologyExtractor::merge_and_add_edge(
                        topology_map,
                        &mut exploration_map,
                        this_prev_pos,
                        other_prev_pos,
                    );
                    continue;
                }
                CellState::Unvisited => {
                    let cell = exploration_map.get_mut((pos.1, pos.0)).unwrap();
                    cell.cell_state = CellState::Visited;
                    cell.prev_pos = data.prev_pos;
                    cell.root_node = Some(data.root_node);
                    exploration_map.get_mut((pos.1, pos.0)).unwrap().cell_state =
                        CellState::Visited;
                }
            };

            let mut visit_mask = vec![false; 8];

            for i in 0..8 {
                match TopologyExtractor::get_neighboring_pos(data.pos, (map_width, map_height), i) {
                    Some((x, y)) => {
                        if *thinned_occupancy_map.get((y, x)).unwrap() {
                            *visit_mask.get_mut(i).unwrap() = true;
                        }
                    }
                    None => {}
                }
            };

            for i in 0..4 {
                match TopologyExtractor::get_neighboring_pos(data.pos, (map_width, map_height), 2 * i) {
                    Some((x, y)) => {
                        if *thinned_occupancy_map.get((y, x)).unwrap() {
                            *visit_mask.get_mut((8 + 2 * i - 1) % 8).unwrap() = false;
                            *visit_mask.get_mut((8 + 2 * i + 1) % 8).unwrap() = false;
                        }
                    }
                    None => {}
                };
            }

            for neighbor in 0..GRID_OFFSETS_RIM.len() {
                if !*visit_mask.get(neighbor).unwrap() {
                    continue;
                }
                
                let dx = GRID_OFFSETS_RIM[neighbor][0];
                let dy = GRID_OFFSETS_RIM[neighbor][1];
                let x: isize = (data.pos.0 as isize + dx);
                let y: isize = (data.pos.1 as isize + dy);

                if x < 0 || x >= map_width as isize || y < 0 || y >= map_height as isize {
                    continue;
                }

                let neighbor_pos = (x as usize, y as usize);

                if neighbor_pos == data.prev_pos {
                    continue;
                }

                if exploration_map
                    .get((neighbor_pos.1, neighbor_pos.0))
                    .unwrap()
                    .cell_state
                    != CellState::Unvisited
                {
                    continue;
                }

                bfs_queue.push_back(BfsData {
                    root_node: data.root_node,
                    pos: neighbor_pos,
                    prev_pos: data.pos,
                });
            }
        }
    }

    fn compute_pixel_score(thinned_occupancy_map: &Array2<bool>, x: usize, y: usize) -> i32 {
        let mut adjacent_pixels = 0;
        let mut contiguous_intervals = 0;

        for [dx, dy] in GRID_OFFSETS_RIM.iter() {
            if let Some(true) =
                thinned_occupancy_map.get(((y as isize + dy) as usize, (x as isize + dx) as usize))
            {
                adjacent_pixels += 1;
            }
        }

        for i in 0..GRID_OFFSETS_RIM.len() {
            let i1 = (i + 0) % GRID_OFFSETS_RIM.len();
            let i2 = (i + 1) % GRID_OFFSETS_RIM.len();

            if let Some(false) = thinned_occupancy_map.get((
                (y as isize + GRID_OFFSETS_RIM[i1][1]) as usize,
                (x as isize + GRID_OFFSETS_RIM[i1][0]) as usize,
            )) {
                continue;
            }

            if let Some(false) = thinned_occupancy_map.get((
                (y as isize + GRID_OFFSETS_RIM[i2][1]) as usize,
                (x as isize + GRID_OFFSETS_RIM[i2][0]) as usize,
            )) {
                continue;
            }

            contiguous_intervals += 1;
        }

        let score = adjacent_pixels - contiguous_intervals;
        return score;
    }

    fn merge_and_add_edge(
        topology_map: &mut TopologyMap,
        exploration_map: &mut Array2<ExplorationData>,
        this_side_prev_pos: (usize, usize),
        other_side_pos: (usize, usize),
    ) {
        let mut waypoints_temp: VecDeque<(usize, usize)> = VecDeque::new();

        let mut this_side_pos = this_side_prev_pos;
        let this_side_root = exploration_map
            .get((this_side_pos.1, this_side_pos.0))
            .unwrap()
            .root_node
            .unwrap();

        loop {
            waypoints_temp.push_front(this_side_pos);
            let prev_pos = exploration_map
                .get((this_side_pos.1, this_side_pos.0))
                .unwrap()
                .prev_pos;

            if prev_pos == this_side_pos {
                break;
            }

            exploration_map
                .get_mut((this_side_pos.1, this_side_pos.0))
                .unwrap()
                .cell_state = CellState::Merged;
            this_side_pos = prev_pos;
        }

        let mut _other_side_pos = other_side_pos;
        let other_side_root = exploration_map
            .get((_other_side_pos.1, _other_side_pos.0))
            .unwrap()
            .root_node
            .unwrap();

        loop {
            waypoints_temp.push_back(_other_side_pos);
            let prev_pos = exploration_map
                .get((_other_side_pos.1, _other_side_pos.0))
                .unwrap()
                .prev_pos;

            if prev_pos == _other_side_pos {
                break;
            }

            exploration_map
                .get_mut((_other_side_pos.1, _other_side_pos.0))
                .unwrap()
                .cell_state = CellState::Merged;
            _other_side_pos = prev_pos;
        }

        let mut waypoints: Vec<Vector2D> = Vec::new();
        let lower_group: u32;
        let upper_group: u32;

        if this_side_root < other_side_root {
            lower_group = this_side_root;
            upper_group = other_side_root;

            for (x, y) in waypoints_temp.iter() {
                waypoints.push(Vector2D::from_xy(*x as f64, *y as f64));
            }
        } else {
            lower_group = other_side_root;
            upper_group = this_side_root;

            for (x, y) in waypoints_temp.iter().rev() {
                waypoints.push(Vector2D::from_xy(*x as f64, *y as f64));
            }
        }

        topology_map
            .add_edge(
                lower_group,
                upper_group,
                TopologyEdge::from_waypoints(waypoints),
            )
            .expect("Error while adding edge to topology map.");
    }

    fn get_neighboring_pos(
        pos: (usize, usize),
        dim: (usize, usize),
        offset_index: usize,
    ) -> Option<(usize, usize)> {
        let x = pos.0 as isize + GRID_OFFSETS_RIM[offset_index][0];
        let y = pos.1 as isize + GRID_OFFSETS_RIM[offset_index][1];

        if x < 0 || x >= dim.0 as isize || y < 0 || y >= dim.1 as isize {
            return None;
        }

        return Some((x as usize, y as usize));
    }
}

#[derive(Clone)]
struct BfsData {
    /// ID of root node.
    pub root_node: u32,

    /// Position of cell in (x, y).
    pub pos: (usize, usize),

    /// Position of previous cell in (x, y).
    pub prev_pos: (usize, usize),
}

#[derive(Clone, PartialEq)]
enum CellState {
    Unvisited,
    Visited,
    Merged,
}

#[derive(Clone)]
struct ExplorationData {
    pub cell_state: CellState,
    pub root_node: Option<u32>,
    pub pos: (usize, usize),
    pub prev_pos: (usize, usize),
}
