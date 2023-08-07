use bevy::prelude::*;

use crate::constants::BLOCK_SCALE;

pub fn grid_to_matrix(coords: Vec2) -> (i32, i32) {
    let matrix_col = (((coords.x).abs() + 6.) / BLOCK_SCALE).floor() as i32;
    let matrix_row = ((coords.y + 6.) / BLOCK_SCALE).floor().abs() as i32;
    (matrix_row, matrix_col)
}
