use system_foundation_rust::{
    map::{
        grid::grid_map::{GridMap, OccupiedRegionColor},
        io::{grid_map_exporter::GridMapExporter, topology_map_exporter::TopologyMapExporter},
        topology::topology_generation::{
            topology_coordinate_converter::TopologyCoordinateConverter,
            topology_extractor::TopologyExtractor, topology_vectorizer::TopologyVectorizer,
        },
    },
    math::numerics::vector2i::Vector2I,
};

fn main() {
    let image_path = "example_data/kawaii-cupcake.gif";
    let grid_map = GridMap::from_image(image_path, OccupiedRegionColor::White, 100, 1_f64)
        .expect("Failed to convert image into grid map.");
    println!("Grid map generated.");
    let topology_map = TopologyExtractor::extract(&grid_map);
    let (vectorized_topology_map, node_groups) = TopologyVectorizer::vectorizer(&topology_map);
    println!(
        "Topology map generated: {} nodes, {} edges, {} groups.",
        vectorized_topology_map.get_node_count(),
        vectorized_topology_map.get_edge_count(),
        node_groups.len()
    );

    let converter = TopologyCoordinateConverter::new(
        0.004,
        (grid_map.horizontal_cells(), grid_map.vertical_cells()),
    );

    GridMapExporter::export(&grid_map);
    TopologyMapExporter::export_topology_as_image(
        "topology_map.png",
        &vectorized_topology_map,
        1_f64,
        20,
        false,
    );
}
