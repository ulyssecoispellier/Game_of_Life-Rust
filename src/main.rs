use macroquad::prelude::*;
use game_of_life::coord_cells::new_coord_cell;
use game_of_life::lif_or_death::life_or_death;
use game_of_life::renderer;

#[macroquad::main("Game_of_Life-Rust")]
async fn main() {
    let mut coord_cells = new_coord_cell();
    coord_cells[screen_width() as usize /10][screen_height() as usize /10] = 1;
    
    
    
    loop {
        let new_coord_cell = life_or_death(&coord_cells);
        
        renderer::renderer(&new_coord_cell);
        next_frame().await
    }
}