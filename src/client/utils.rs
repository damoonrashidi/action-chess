use comfy::{vec2, Vec2};

use crate::state::coordinate::Coord;

const SQUARE_SIZE: f32 = 1.0;

#[allow(clippy::cast_lossless)]
#[must_use]
pub fn get_vec2_for_coord(coord: &Coord) -> comfy::Vec2 {
    let y = coord.1 as f32 * SQUARE_SIZE;
    let x = coord.0 as f32 * SQUARE_SIZE;
    vec2(x, y)
}

#[must_use]
pub fn get_coord_for_pos(_pos: Vec2) -> Coord {
    Coord(0, 0)
}
