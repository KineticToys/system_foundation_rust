use system_foundation_rust::{
    map::{
        grid::grid_map::{GridMap, OccupiedRegionColor},
        io::topology_map_exporter::TopologyMapExporter,
        topology::topology_generation::topology_extractor::TopologyExtractor,
    },
    math::numerics::vector2i::Vector2I,
};

fn main() {
    let image_path = "example_data/kawaii-cupcake.gif";
    let grid_map = GridMap::from_image(image_path, OccupiedRegionColor::White, 200, 1_f64)
        .expect("Failed to convert image into grid map.");
    println!("Grid map generated.");
    let topology_extractor = TopologyExtractor::new();
    let topology_map = topology_extractor.extract(&grid_map);
    println!("Topology map generated.");

    TopologyMapExporter::export_topology_as_image(
        "topology_map.png",
        &topology_map,
        1_f64,
        20,
        true,
    );
}
