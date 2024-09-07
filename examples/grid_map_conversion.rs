use image::{imageops, GenericImageView, ImageReader};
use ndarray::Array2;
use system_foundation_rust::map::{grid::grid_map::{GridMap, OccupiedRegionColor}, io::grid_map_exporter::GridMapExporter};

fn main() {
    let image_path = "example_data/turtle.jpg";
    let grid_map = GridMap::from_image(image_path, OccupiedRegionColor::White, 200, 0.001)
        .expect("Failed to convert image into grid map.");
    println!("Converted image into grid map.");
    GridMapExporter::export(&grid_map);
    println!("Exported grid map to png file.");
}
