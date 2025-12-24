use crate::constants::*;
use crate::game::*;
use crate::piece::Piece;
use macroquad::prelude::*;
use crate::board::Board;

pub fn draw_frame(game: &GameState) {
    clear_background(BLACK);
    render_background();
    draw_buttons(&game);
    match game.play_state {
            PlayState::Start => {
                draw_text_centered(
                    "Press ENTER to begin",
                    300.0,
                    50.0,
                    WHITE,
                );
            },
            PlayState::Playing => {
                draw_board(&game.board);
                draw_piece(&game.current);
                draw_score(game.score);
            },
            PlayState::ClearBlocks => {
                draw_board(&game.board);
                draw_score(game.score);
            },
            PlayState::Paused => {

            },
            PlayState::GameOver => {
                draw_text_centered(
                    "Game Over!",
                    300.0,
                    50.0,
                    WHITE,
                );
            }
        }
}
fn draw_piece(piece:&Piece) {
    for (dcol, drow) in piece.cells() {
        let col:i32 = dcol;
        let row:i32 = drow;
        draw_block(col, row,piece.kind.color(), DARKGRAY);
    }
}
fn draw_board(board:&Board) {
    for cell in board.filled_cells() {
        draw_block(cell.col, cell.row, cell.color, DARKGRAY);
    }
}
fn render_background() {
    // draw boundaries 
    let offset_x = (screen_width() - BOARD_W) / 2.0;
    // left border
    draw_line(
        offset_x,
        0.0,
        offset_x,
        BOARD_H as f32,
        2.0,
        GRAY,
    );
    // right border
    draw_line(
        offset_x + BOARD_W as f32,
        0.0,
        offset_x + BOARD_W as f32,
        BOARD_H as f32,
        2.0,
        GRAY,
    );
    // bottom border
    draw_line(
        offset_x,
        BOARD_H as f32,
        offset_x + BOARD_W as f32,
        BOARD_H as f32,
        2.0,
        GRAY,
    );
}
fn draw_score(score: i32) {
    let text = score.to_string();
    let dims = measure_text(&text, None, 24 as u16, 1.0);
    let x = screen_width() - dims.width - 15.0;
    draw_text(&text, x, 50.0, 24.0, WHITE );
}
pub fn coords_to_pixels(col: i32, row: i32) -> (f32, f32) {
    (col as f32 * SQUARE_SIZE, row as f32 * SQUARE_SIZE)
}
pub fn draw_block(col:i32, row:i32, fill: Color, border:Color) {
    let (px, py) = coords_to_pixels(col, row);
    let offset_x = px + (screen_width() - BOARD_W) / 2.0;
    //let offset_y = py + (screen_height() - BOARD_H) / 2.0;
    let offset_y: f32 = py;
    draw_rectangle
        ( offset_x
        , offset_y
        , SQUARE_SIZE
        , SQUARE_SIZE
        , fill);
    draw_rectangle_lines
        ( offset_x
        , offset_y
        , SQUARE_SIZE
        , SQUARE_SIZE
        , LINE_THICKNESS 
        , border);
}
pub fn draw_text_centered(text: &str, y: f32, size: f32, color: Color) {
    let dims = measure_text(text, None, size as u16, 1.0);
    let x = (screen_width() - dims.width) * 0.5;
    draw_text(text, x, y, size, color);
}
fn draw_buttons(game:&GameState ) {

}