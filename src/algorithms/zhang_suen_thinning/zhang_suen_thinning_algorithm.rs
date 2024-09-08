/*
 * This code is written in reference from:
 * https://rosettacode.org/wiki/Zhang-Suen_thinning_algorithm
 */

use image::{Rgb, RgbImage};
use ndarray::Array2;

static GRID_OFFSETS: [(isize, isize); 8] = [
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

#[derive(Clone, Copy, PartialEq)]
enum PixelState {
    Vacant,
    Occupied,
    MarkedForPrune,
}

pub struct ZhangSuenThinningAlgorithm {
    image: Array2<PixelState>,
}

impl ZhangSuenThinningAlgorithm {
    pub fn new() -> Self {
        return Self {
            image: ndarray::array![[]],
        };
    }

    /// Run Zhang-Suen thinning algorithm.
    pub fn run(&mut self, occupancy_map: &Array2<bool>) -> Array2<bool> {
        let (map_height, map_width) = occupancy_map.dim();
        self.image = Array2::from_shape_fn((map_height, map_width), |(y, x)| {
            return match occupancy_map.get((y, x)).unwrap().clone() {
                true => PixelState::Occupied,
                false => PixelState::Vacant,
            };
        });

        loop {
            let pruned_pixels =
                self.single_pass([[2, 4, 6], [4, 6, 8]]) + self.single_pass([[2, 4, 8], [2, 6, 8]]);
            if pruned_pixels == 0 {
                break;
            }
        }

        let ret: Array2<bool> = Array2::from_shape_fn((map_height, map_width), |(y, x)| {
            self.image.get((y, x)).unwrap().clone() != PixelState::Vacant
        });

        self.image = ndarray::array![[]];
        return ret;
    }

    fn single_pass(&mut self, triplet_conditions: [[usize; 3]; 2]) -> usize {
        let image_height = self.image.dim().0;
        let image_width = self.image.dim().1;
        let mut pruned_pixels: usize = 0;

        for x in 0..image_width {
            for y in 0..image_height {
                if x == 0 || x == (image_width - 1) || y == 0 || y == (image_height - 1) {
                    continue;
                }

                let pixel_val = self.image.get((y, x)).unwrap().clone();

                if pixel_val == PixelState::Vacant {
                    continue;
                }

                let cond_b = self.compute_condition_b(x, y);
                if cond_b < 2 || cond_b > 6 {
                    continue;
                }

                let cond_a = self.compute_condition_a(x, y);
                if cond_a != 1 {
                    continue;
                }

                if !self.assert_triplet_condition(x, y, triplet_conditions[0]) {
                    continue;
                }

                if !self.assert_triplet_condition(x, y, triplet_conditions[1]) {
                    continue;
                }

                *self.image.get_mut((y, x)).unwrap() = PixelState::MarkedForPrune;
            }
        }

        for x in 0..image_width {
            for y in 0..image_height {
                let mut_ref = self.image.get_mut((y, x)).unwrap();
                if *mut_ref == PixelState::MarkedForPrune {
                    *mut_ref = PixelState::Vacant;
                    pruned_pixels += 1;
                }
            }
        }

        return pruned_pixels;
    }

    fn compute_condition_a(&self, x: usize, y: usize) -> usize {
        let mut transitions = 0_usize;

        for i in 0..GRID_OFFSETS.len() {
            let (dx1, dy1) = GRID_OFFSETS.get(i % GRID_OFFSETS.len()).unwrap();
            let (dx2, dy2) = GRID_OFFSETS.get((i + 1) % GRID_OFFSETS.len()).unwrap();
            let val1 = self
                .image
                .get(((y as isize + dy1) as usize, (x as isize + dx1) as usize))
                .unwrap()
                .clone();
            let val2 = self
                .image
                .get(((y as isize + dy2) as usize, (x as isize + dx2) as usize))
                .unwrap()
                .clone();

            if val1 == PixelState::Vacant && val2 != PixelState::Vacant {
                transitions += 1;
            }
        }

        return transitions;
    }

    fn compute_condition_b(&self, x: usize, y: usize) -> usize {
        let mut non_vacant_pixels = 0_usize;
        for (dx, dy) in GRID_OFFSETS.iter() {
            if self
                .image
                .get(((y as isize + dy) as usize, (x as isize + dx) as usize))
                .unwrap()
                .clone()
                != PixelState::Vacant
            {
                non_vacant_pixels += 1;
            }
        }

        return non_vacant_pixels;
    }

    fn assert_triplet_condition(&self, x: usize, y: usize, c: [usize; 3]) -> bool {
        for cond in c {
            let (dx, dy) = GRID_OFFSETS.get(cond - 2).unwrap();
            let val = self
                .image
                .get(((y as isize + dy) as usize, (x as isize + dx) as usize))
                .unwrap()
                .clone();

            if val == PixelState::Vacant {
                return true;
            }
        }

        return false;
    }
}
