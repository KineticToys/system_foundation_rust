use std::collections::{HashSet, VecDeque};

use ndarray::Array2;

use crate::{graph::graph::Graph, map::topology::{topology_edge::TopologyEdge, topology_node::{TopologyNode, TopologyNodeType}}, math::numerics::vector2d::Vector2D};

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

pub struct TopologyExtractor {
    occupancy_map: Array2<bool>,
    bfs_queue: VecDeque<BfsData>,
}

impl TopologyExtractor {
    pub fn new() -> Self {
        return Self {
            occupancy_map: ndarray::array![[]],
            bfs_queue: VecDeque::new(),
        };
    }

    pub fn extract(&mut self, occupancy_map: &Array2<bool>) -> TopologyMap {
        self.occupancy_map = occupancy_map.clone();
        let mut topology_map: TopologyMap = Graph::new(false, false);
        self.find_nodes(&mut topology_map);
        self.find_edges(&mut topology_map);
        return topology_map;
    }

    fn find_nodes(&mut self, topology_map: &mut TopologyMap) {
        let (map_height, map_width) = self.occupancy_map.dim();

        for x in 1..(map_width - 1) {
            for y in 0..(map_height - 1) {
                if let Some(false) = self.occupancy_map.get((y, x)) {
                    continue;
                }

                let mut adjacent_pixels = 0;
                let mut contiguous_intervals = 0;

                for [dx, dy] in GRID_OFFSETS_RIM.iter() {
                    if let Some(true) = self
                        .occupancy_map
                        .get(((y as isize + dy) as usize, (x as isize + dx) as usize))
                    {
                        adjacent_pixels += 1;
                    }
                }

                for i in 0..GRID_OFFSETS_RIM.len() {
                    let i1 = (i + 0) % GRID_OFFSETS_RIM.len();
                    let i2 = (i + 1) % GRID_OFFSETS_RIM.len();

                    if let Some(false) = self.occupancy_map.get((
                        (y as isize + GRID_OFFSETS_RIM[i1][1]) as usize,
                        (x as isize + GRID_OFFSETS_RIM[i1][0]) as usize,
                    )) {
                        continue;
                    }

                    if let Some(false) = self.occupancy_map.get((
                        (y as isize + GRID_OFFSETS_RIM[i2][1]) as usize,
                        (x as isize + GRID_OFFSETS_RIM[i2][0]) as usize,
                    )) {
                        continue;
                    }

                    contiguous_intervals += 1;
                }

                let score = adjacent_pixels - contiguous_intervals;
                if score <= 1 {
                    let node_id = topology_map.add_node(TopologyNode {
                        node_type: TopologyNodeType::Endpoint,
                        position: Vector2D::from_xy(x as f64, y as f64),
                    });
                    self.bfs_queue.push_back(BfsData {
                        pos: (x, y),
                        prev_pos: (x, y),
                        root_node: node_id,
                    });
                } else if score >= 3 {
                    let node_id = topology_map.add_node(TopologyNode {
                        node_type: TopologyNodeType::Intersection,
                        position: Vector2D::from_xy(x as f64, y as f64),
                    });
                    self.bfs_queue.push_back(BfsData {
                        pos: (x, y),
                        prev_pos: (x, y),
                        root_node: node_id,
                    })
                }
            }
        }
    }

    fn find_edges(&mut self, topology_map: &mut TopologyMap) {
        let (map_height, map_width) = self.occupancy_map.dim();
        let mut exploration_map: Array2<ExplorationData> =
            Array2::from_shape_fn((map_height, map_width), |(y, x)| ExplorationData {
                cell_state: CellState::Unvisited,
                root_node: None,
                pos: (x, y),
                prev_pos: None,
            });
        let mut unvisited_cells: HashSet<(usize, usize)> = HashSet::new();

        // Insert all occupied cells into unvisited cells list.
        for x in 0..self.occupancy_map.dim().1 {
            for y in 0..self.occupancy_map.dim().0 {
                if *self.occupancy_map.get((y, x)).unwrap() {
                    unvisited_cells.insert((x, y));
                }
            }
        }

        self.bfs_worker(&mut exploration_map, &mut unvisited_cells, topology_map);

        while !unvisited_cells.is_empty() {
            let root_pos = unvisited_cells.iter().next().unwrap();
            let node_id = topology_map.add_node(TopologyNode {
                node_type: TopologyNodeType::Waypoint,
                position: Vector2D::from_xy(root_pos.0 as f64, root_pos.1 as f64),
            });
            self.bfs_queue.push_back(BfsData {
                root_node: node_id,
                pos: *root_pos,
                prev_pos: *root_pos,
            });
            self.bfs_worker(&mut exploration_map, &mut unvisited_cells, topology_map);
        }
    }

    fn bfs_worker(
        &mut self,
        exploration_map: &mut Array2<ExplorationData>,
        unvisited_cells: &mut HashSet<(usize, usize)>,
        topology_map: &mut TopologyMap,
    ) {
        while !self.bfs_queue.is_empty() {
            let bfs_data = self.bfs_queue.pop_front().unwrap();
            let (x, y) = bfs_data.pos;
            let (x_prev, y_prev) = bfs_data.prev_pos;

            match exploration_map.get((y, x)).unwrap().cell_state {
                CellState::Visited => {
                    let other_side_root = exploration_map.get((y, x)).unwrap().root_node.unwrap();
                    let this_side_root = bfs_data.root_node;

                    TopologyExtractor::merge_and_add_edge(
                        topology_map,
                        exploration_map,
                        bfs_data.prev_pos,
                        bfs_data.pos,
                    );
                    continue;
                }
                CellState::Unvisited => {
                    let cell_data = exploration_map.get_mut((y, x)).unwrap();
                    cell_data.cell_state = CellState::Visited;
                    cell_data.root_node = Some(bfs_data.root_node);
                    cell_data.prev_pos = Some((x_prev, y_prev));
                    unvisited_cells.remove(&(x, y));
                }
                CellState::Merged => continue,
            };

            // Scan for straight-adjacent cells.
            for i in 0..GRID_OFFSETS_RIM.len() {
                let i_prev = (i + GRID_OFFSETS_RIM.len() - 1) % GRID_OFFSETS_RIM.len();
                let i_next = (i + 1) % GRID_OFFSETS_RIM.len();
                let [dx, dy] = GRID_OFFSETS_RIM.get(i).unwrap();
                let [dx_prev, dy_prev] = GRID_OFFSETS_RIM.get(i_prev).unwrap();
                let [dx_next, dy_next] = GRID_OFFSETS_RIM.get(i_next).unwrap();
                let (x_adj, y_adj) = ((x as isize + dx) as usize, (y as isize + dy) as usize);
                let (x_adj_prev, y_adj_prev) = (
                    (x as isize + dx_prev) as usize,
                    (y as isize + dy_prev) as usize,
                );
                let (x_adj_next, y_adj_next) = (
                    (x as isize + dx_next) as usize,
                    (y as isize + dy_next) as usize,
                );

                // If adjacent cell is not an occupied one, skip the loop.
                if !*self.occupancy_map.get((y_adj, x_adj)).unwrap() {
                    continue;
                }

                // If this cell is in a diagonal position,
                // and any of previous or next cells is occupied, skip the loop.
                if *dx != 0
                    && *dy != 0
                    && (*self.occupancy_map.get((y_adj_prev, x_adj_prev)).unwrap()
                        || *self.occupancy_map.get((y_adj_next, x_adj_next)).unwrap())
                {
                    continue;
                }

                // If adjacent cell is already visited, skip the loop.
                if exploration_map.get((y_adj, x_adj)).unwrap().cell_state != CellState::Unvisited {
                    continue;
                }

                self.bfs_queue.push_back(BfsData {
                    root_node: bfs_data.root_node,
                    pos: (x_adj, y_adj),
                    prev_pos: (x, y),
                });
            }
        }
    }

    fn merge_and_add_edge(
        topology_map: &mut TopologyMap,
        exploration_map: &mut Array2<ExplorationData>,
        this_side_prev_pos: (usize, usize),
        other_side_pos: (usize, usize),
    ) {
        let mut this_side_pos = this_side_prev_pos;
        let mut this_side_waypoints: VecDeque<(usize, usize)> = VecDeque::new();
        let this_side_root = exploration_map
            .get((this_side_pos.1, this_side_pos.0))
            .unwrap()
            .root_node
            .unwrap();

        loop {
            this_side_waypoints.push_front(this_side_pos);
            exploration_map
                .get_mut((this_side_pos.1, this_side_pos.0))
                .unwrap()
                .cell_state = CellState::Merged;

            let prev_pos = exploration_map
                .get((this_side_pos.1, this_side_pos.0))
                .unwrap()
                .prev_pos
                .unwrap();

            if prev_pos == this_side_pos {
                break;
            }

            this_side_pos = prev_pos;
        }

        let mut _other_side_pos = other_side_pos;
        let mut other_side_waypoints: VecDeque<(usize, usize)> = VecDeque::new();
        let other_side_root = exploration_map
            .get((_other_side_pos.1, _other_side_pos.0))
            .unwrap()
            .root_node
            .unwrap();

        loop {
            other_side_waypoints.push_front(_other_side_pos);
            exploration_map
                .get_mut((other_side_pos.1, other_side_pos.0))
                .unwrap()
                .cell_state = CellState::Merged;

            let prev_pos = exploration_map
                .get((_other_side_pos.1, _other_side_pos.0))
                .unwrap()
                .prev_pos
                .unwrap();

            if prev_pos == _other_side_pos {
                break;
            }

            _other_side_pos = prev_pos;
        }

        let mut waypoints: Vec<(Vector2D)> =
            Vec::with_capacity(this_side_waypoints.len() + other_side_waypoints.len());
        let lower_group: u32;
        let upper_group: u32;

        if this_side_root < other_side_root {
            lower_group = this_side_root;
            upper_group = other_side_root;

            for (x, y) in this_side_waypoints.iter() {
                waypoints.push(Vector2D::from_xy(*x as f64, *y as f64));
            }

            for (x, y) in other_side_waypoints.iter().rev() {
                waypoints.push(Vector2D::from_xy(*x as f64, *y as f64));
            }
        } else {
            lower_group = other_side_root;
            upper_group = this_side_root;

            for (x, y) in other_side_waypoints.iter() {
                waypoints.push(Vector2D::from_xy(*x as f64, *y as f64));
            }

            for (x, y) in this_side_waypoints.iter().rev() {
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
}

#[derive(Clone)]
struct BfsData {
    pub root_node: u32,
    pub pos: (usize, usize),
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
    pub prev_pos: Option<(usize, usize)>,
}
