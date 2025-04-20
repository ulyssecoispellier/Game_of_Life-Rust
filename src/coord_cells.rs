use macroquad::prelude::screen_width;
use macroquad::window::screen_height;

pub type CellCoord = Vec<Vec<u32>>;
pub fn new_coord_cell() -> CellCoord {
    let mut columns = Vec::with_capacity(screen_width() as usize / 10);

    for _ in 0..screen_width() as usize {
        let mut column = Vec::with_capacity(screen_height() as usize / 10);

        for _ in 0..screen_height() as usize {
            column.push(0)
        }
        columns.push(column);
    }
    columns
}