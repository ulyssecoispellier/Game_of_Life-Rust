use crate::coord_cells::{new_coord_cell, CellCoord};

pub fn life_or_death(cell_coord: &CellCoord) -> CellCoord {
    let mut life_present = 0;
    let mut new_cell_coord = new_coord_cell();

    for (x, column) in cell_coord.iter().enumerate() {
        for (y, s) in column.iter().enumerate() {
            if 1 == cell_coord[x][y] {
                for i in -1..=1 {
                    for j in -1..=1 {
                        if cell_coord[(x as isize + i) as usize][(y as isize + j) as usize] == 1 { life_present += 1 }
                    }
                }

                match life_present {
                    1 | 0 => new_cell_coord[x][y] = 0,
                    2 | 3 => new_cell_coord[x][y] = 1,
                    _ => new_cell_coord[x][y] = 0,
                }
            } else {
                for i in -1..=1 {
                    for j in -1..=1 {
                        if cell_coord[(x as isize + i) as usize][(y as isize + j) as usize] == 1 { life_present += 1 }
                    }
                }

                match life_present {
                    3 => new_cell_coord[x][y] = 1,
                    _ => new_cell_coord[x][y] = 0,
                }
            }
        }
    }

    new_cell_coord
}