mod piece;
mod board;
mod constants;
mod game; 
mod render;
mod piece_kind;
mod button;
use macroquad::prelude::*;
use crate::game::*;
use macroquad::miniquad::date;
use macroquad::rand::srand;

#[macroquad::main("wasm_tetris")]
async fn main() {
    info!(">> Tetris starting");
    srand(date::now() as u64);
    let mut game: GameState = GameState::new();
    loop {
        game.update();
        render::draw_frame( &game );
        next_frame().await;
    }
}


