use std::io;

use image::{imageops, ColorType, ImageReader};
use ndarray::Array2;

pub struct GridMap {
    cells: Array2<GridMapCell>,
    cell_size: f64,
}

impl GridMap {
    pub fn with_cell_state(width: usize, height: usize, cell_size: f64, state: CellState) -> Self {
        return Self {
            cells: Array2::from_shape_fn((height, width), |(_, _)| GridMapCell::new(state)),
            cell_size: cell_size,
        };
    }

    pub fn from_image(
        image_path: &str,
        occupied_region_color: OccupiedRegionColor,
        threshold: u8,
    ) -> Result<Self, GridMapError> {
        let image_reader = match ImageReader::open(image_path) {
            Ok(reader) => reader,
            Err(_) => return Err(GridMapError::ImageNotFound),
        };
        let raw_image = match image_reader.decode() {
            Ok(img) => img,
            Err(_) => return Err(GridMapError::ImageDecodeFailed),
        };

        let grayscale_image = imageops::colorops::grayscale(&raw_image);
        todo!();
    }

    pub fn get_by_cell(&self, row: usize, column: usize) -> Option<&GridMapCell> {
        return self.cells.get((row, column));
    }

    pub fn get_by_cell_mut(&mut self, row: usize, column: usize) -> Option<&mut GridMapCell> {
        return self.cells.get_mut((row, column));
    }

    pub fn get_by_coordinate(&self, x: f64, y: f64) -> Option<&GridMapCell> {
        return self.get_by_coordinate_mut(x, y);
    }

    pub fn get_by_coordinate_mut(&self, x: f64, y: f64) -> Option<&GridMapCell> {
        let (height, width) = self.cells.dim();
        let (roi_width, roi_height) = (
            width as f64 * self.cell_size,
            height as f64 * self.cell_size,
        );

        todo!();
    }
}

#[derive(Clone, Debug)]
pub struct GridMapCell {
    state: CellState,
}

impl GridMapCell {
    pub fn new(state: CellState) -> Self {
        return Self { state: state };
    }

    pub fn state(&self) -> &CellState {
        return &self.state;
    }

    pub fn state_mut(&mut self) -> &mut CellState {
        return &mut self.state;
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellState {
    Vacant,
    Occupied,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OccupiedRegionColor {
    White,
    Black,
}

pub enum GridMapError {
    ImageNotFound,
    ImageDecodeFailed,
}
