use image::{imageops, GenericImageView, ImageReader};
use ndarray::Array2;

fn main() {
    let image_path = "example_data/turtle.jpg";
}

fn image_to_occupancy_map(
    image_path: &str,
    threshold: u8,
    occupied_region_color: OccupiedRegionColor,
) -> Array2<bool> {
    let image_reader = ImageReader::open(image_path).expect("Failed to open image file.");
    let raw_image = image_reader.decode().expect("Failed to decode image.");
    let grayscale_image = imageops::colorops::grayscale(&raw_image);
    let mut occupancy_map: Array2<bool> = Array2::from_elem(
        (
            grayscale_image.height() as usize,
            grayscale_image.width() as usize,
        ),
        true,
    );

    for row in 0..(grayscale_image.height() as usize) {
        for col in 0..(grayscale_image.width() as usize) {
            let pixel = grayscale_image.get_pixel(col as u32, row as u32).0[0];
            if occupied_region_color == OccupiedRegionColor::Black && pixel > threshold
                || occupied_region_color == OccupiedRegionColor::White && pixel < threshold
            {
                *occupancy_map.get_mut((row, col)).unwrap() = false;
            }
        }
    }

    return occupancy_map;
}

#[derive(PartialEq)]
enum OccupiedRegionColor {
    White,
    Black,
}
