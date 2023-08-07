use bevy::prelude::*;

use crate::constants::BLOCK_SCALE;

pub fn grid_to_matrix(coords: Vec2) -> (usize, usize) {
    let matrix_col = (((coords.x).abs() + 6.) / BLOCK_SCALE).floor() as usize;
    let matrix_row = ((coords.y + 6.) / BLOCK_SCALE).floor().abs() as usize;
    (matrix_row, matrix_col)
}
