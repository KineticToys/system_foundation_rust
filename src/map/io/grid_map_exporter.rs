use image::{Rgb, RgbImage};

use crate::map::grid::grid_map::{GridMap, GridMapCellState};

pub struct GridMapExporter;

impl GridMapExporter {
    pub fn export(grid_map: &GridMap) {
        let width = grid_map.horizontal_cells();
        let height = grid_map.vertical_cells();
        let mut img: RgbImage = RgbImage::new(width as u32, height as u32);
        
        for r in 0..height {
            for c in 0..width {
                *img.get_pixel_mut(c as u32, r as u32) = match grid_map.get_by_cell(r, c).unwrap().state() {
                    GridMapCellState::Occupied => Rgb([0, 0, 0]),
                    GridMapCellState::Vacant => Rgb([255, 255, 255]),
                };
            }
        }

        img.save("grid_map.png");
    }
}
