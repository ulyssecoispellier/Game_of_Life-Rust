use macroquad::prelude::*;
use crate::coord_cells::CellCoord;

pub fn renderer(coord_cells: &CellCoord) {
    clear_background(BLACK);
    set_fullscreen(true);

    for (x, column) in coord_cells.iter().enumerate() {
        for (y, s) in column.iter().enumerate() {
            if 1 == coord_cells[x][y] {
                draw_rectangle(x as f32 * 10.0, y as f32 * 10.0, 10.0, 10.0, WHITE);
            } 
        }
    }
}