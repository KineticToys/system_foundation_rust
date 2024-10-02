use std::io;

use image::{imageops, ColorType, ImageReader};
use ndarray::Array2;

pub struct GridMap {
    cells: Array2<GridMapCell>,
    cell_size: f64,
}

impl GridMap {
    pub fn with_cell_state(
        width: usize,
        height: usize,
        cell_size: f64,
        state: GridMapCellState,
    ) -> Self {
        return Self {
            cells: Array2::from_shape_fn((height, width), |(_, _)| GridMapCell::new(state)),
            cell_size: cell_size,
        };
    }

    pub fn from_image(
        image_path: &str,
        occupied_region_color: OccupiedRegionColor,
        threshold: u8,
        cell_size: f64,
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
        let (image_width, image_height) = grayscale_image.dimensions();

        let cells: Array2<GridMapCell> =
            Array2::from_shape_fn((image_height as usize, image_width as usize), |(r, c)| {
                let pixel_value = grayscale_image.get_pixel(c as u32, r as u32).0[0];
                let cell_state: GridMapCellState;

                if r == 0 || c == 0 || (r == image_height as usize - 1) || (c == image_width as usize - 1) {
                    cell_state = GridMapCellState::Occupied;
                } else if occupied_region_color == OccupiedRegionColor::Black
                    && pixel_value < threshold
                    || occupied_region_color == OccupiedRegionColor::White
                        && pixel_value > threshold
                {
                    cell_state = GridMapCellState::Occupied;
                } else {
                    cell_state = GridMapCellState::Vacant;
                }

                return GridMapCell { state: cell_state };
            });

        return Ok(Self {
            cells: cells,
            cell_size: cell_size,
        });
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

    /// Map the cells in grid map using user-defined mapping function.
    pub fn map<F, T>(&self, map_fn: F) -> Array2<T>
    where
        F: Fn(&GridMapCell) -> T,
    {
        let ret: Array2<T> = Array2::from_shape_fn(self.cells.dim(), |(r, c)| {
            (map_fn)(self.cells.get((r, c)).unwrap())
        });
        return ret;
    }

    /// Number of cells in horizontal direction.
    pub fn horizontal_cells(&self) -> usize {
        return self.cells.dim().1;
    }

    /// Number of cells in vertical direction.
    pub fn vertical_cells(&self) -> usize {
        return self.cells.dim().0;
    }
}

#[derive(Clone, Debug)]
pub struct GridMapCell {
    state: GridMapCellState,
}

impl GridMapCell {
    pub fn new(state: GridMapCellState) -> Self {
        return Self { state: state };
    }

    pub fn state(&self) -> &GridMapCellState {
        return &self.state;
    }

    pub fn state_mut(&mut self) -> &mut GridMapCellState {
        return &mut self.state;
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GridMapCellState {
    Vacant,
    Occupied,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OccupiedRegionColor {
    White,
    Black,
}

#[derive(Clone, Copy, Debug)]
pub enum GridMapError {
    ImageNotFound,
    ImageDecodeFailed,
}
