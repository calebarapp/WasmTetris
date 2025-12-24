use crate::board::*;
use crate::piece::*;
use crate::constants::*;
use macroquad::prelude::*;
use crate::piece_kind::*;
use crate::button::*;

pub enum PlayState {
    Start,
    Playing,
    ClearBlocks,
    Paused,
    GameOver
}
pub struct GameState {
    pub play_state: PlayState,
    pub board: Board,
    pub current: Piece,
    pub next: Piece,
    pub full_rows: Vec<i32>,
    pub flash_anim_color: Color,
    pub score: i32,
    fall_timer: f32,
    input_timer:f32,
    lock_delta: f32,
    player_interacting: bool,
    clear_row_timer: f32,
    clear_row_flash_timer: f32,
    pre_lock_moves:u8,
}
impl Default for GameState {
    fn default() -> Self{
        Self {
            board: Board::new(),
            current: Piece::default(),
            next: Piece::default(),
            play_state: PlayState::Start,            
            player_interacting: false,
            full_rows: Vec::new(),
            fall_timer: 0.0,
            input_timer: 0.0,
            lock_delta: 0.0,
            clear_row_timer: 0.0,
            clear_row_flash_timer: 0.0,
            flash_anim_color: WHITE,
            pre_lock_moves:0,
            score:0,
        }
    }

}
impl GameState {
    pub fn new() -> Self {
        Self::default()
    }

// ======================================
// Main Update Loop
// ======================================
    // Update should only update game state, not render/draw
    pub fn update(&mut self) {
        let dt = get_frame_time() * SECOND; // seconds since last frame
        // PlayState: Playing
        // gravity / lock
        match self.play_state {
            PlayState::Start => {
                self.exec_start_frame();
            },
            PlayState::Playing => {
                self.exec_playing_frame(dt);
            },
            PlayState::ClearBlocks => {
                self.exec_clearblock_frame(dt);
            },
            PlayState::Paused => {
                
            },
            PlayState::GameOver => {
                self.exec_gameover_frame();
            }
        }
    }
    // ===================================================
    //Game Over
    // ===================================================
    fn exec_gameover_frame(&mut self) {
        if is_key_pressed(KeyCode::Enter) {
            *self = Self::default();
            self.play_state = PlayState::Start;
        }
    }

    // ===================================================
    //Start Menu
    // ===================================================
    fn exec_start_frame(&mut self) {
        if is_key_pressed(KeyCode::Enter) {
            // init game
            self.next = self.get_next_piece();
            self.play_state = PlayState::Playing;
        }
    }
    
    // ===================================================
    //Playing
    // ===================================================
    fn exec_playing_frame(&mut self, dt: f32) {
        if self.current.kind == PieceKind::None {
            self.spawn_next_piece();
        }
        self.handle_input_playing(dt);
        self.try_drop_current_piece(dt);
        self.try_piece_lock(dt);
        self.try_clear_lines();
    }
    // handle user input
    fn handle_input_playing(&mut self, delta: f32) {
        self.player_interacting = false;
        self.process_key_press();
        self.process_key_hold(delta);
    }

    fn process_key_hold(&mut self, delta: f32) {
        self.input_timer += delta;
        if self.input_timer < INPUT_INTERVAL_MS {
            return;
        }
        self.input_timer -= INPUT_INTERVAL_MS;
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            self.move_left();
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            self.move_right();
        }
        // Hold keys
        
        // soft Drop
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            self.soft_drop();
        }
    }
    
    fn process_key_press(&mut self) {
        self.player_interacting = false;
        // Rotate CW
        if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::X) {
            self.player_interacting = true;
            self.current.try_rotate( RotDir::CW, &self.board );
        }
        // Rotate CCW
        if is_key_pressed(KeyCode::Z) {
            self.player_interacting = true;
            self.current.try_rotate(RotDir::CCW, &self.board );
        }
        // HardDrop
        if is_key_pressed(KeyCode::Space) {
            self.player_interacting = true;
            // Todo: Hard drop
        }
    }

    fn soft_drop(&mut self) {
        if self.current.try_move_piece( 0, 1, &self.board ) {
            // SCORE: +2 Points for soft drop
            self.score += 2;
        }
    }

fn move_right(&mut self) {
        //move right
        self.current.try_move_piece( 1, 0, &self.board );
        self.player_interacting = true;
    }

fn move_left(&mut self) {
        //move left
        self.current.try_move_piece( -1, 0, &self.board );
        self.player_interacting = true;
    }
    fn try_drop_current_piece(&mut self, delta: f32) {
        // Move block
        self.fall_timer += delta;
        if self.fall_timer >= FALL_INTERVAL_MS {
            self.fall_timer -= FALL_INTERVAL_MS;    
            self.clear_lock_timer();
            if self.current.try_move_piece( 0, 1, &self.board ) {
                // SCORE: +1 for gravity drop
                self.update_score(1);
            }
        }
    }

fn update_score(&mut self, points: i32) {
        self.score += points;
    }
    fn clear_lock_timer(&mut self) {
        self.lock_delta = 0.0;
    }
    fn try_piece_lock(&mut self, delta: f32) {
        // is grounded?
        if self.current.can_move(0, 1, &self.board) {
            return;
        }
        //Invariant: Piece has Lock pending
        // currently interacting? Allow 15 movements before locking
        if self.player_interacting && self.pre_lock_moves < PRE_LOCK_MOVES_ALLOWED {
            self.pre_lock_moves += 1;
            self.clear_lock_timer();
            return;
        }
        // Lock piece
        self.lock_delta += delta;
        if self.lock_delta > LOCK_DELTA_THRESHOLD {
            // lock! Save to board and clear current piece
            self.pre_lock_moves = 0;
            self.board.lock_piece( &self.current );
            self.current = Piece::default();
        }
    }
    fn get_next_piece(&mut self) -> Piece {
        let mut piece = Piece::random_piece();
        piece.try_kick( &self.board );
        piece
    }
    fn spawn_next_piece(&mut self) {
        self.current = self.next;
        self.next = self.get_next_piece();
        
        if !self.next.can_move( 0,0, &self.board) {
            self.play_state = PlayState::GameOver
        }
    }
    fn try_clear_lines(&mut self) {
        self.full_rows = self.board.full_rows();
        if self.full_rows.len() > 0 {
            self.play_state = PlayState::ClearBlocks;
        }
    }

    // ===================================================
    // Clear Blocks
    // ===================================================
    fn exec_clearblock_frame(&mut self, dt: f32) {
        self.handle_flash_animation(dt);
        self.try_clear_full_rows(dt);
    }
    fn handle_flash_animation(&mut self, delta: f32) {
        self.clear_row_flash_timer += delta;
        if self.clear_row_flash_timer > CLEAR_ROW_FLASH_INTERVAL_MS {
            self.clear_row_flash_timer = 0.0;
            // toggle between LIGHTGREY and WHITE
            if self.flash_anim_color == WHITE {
                self.flash_anim_color = LIGHTGRAY;
            } else {
                self.flash_anim_color = WHITE;
            }
            for &i in &self.full_rows {
                self.board.set_row_color( self.flash_anim_color ,i );
            }
        }
    }
    fn try_clear_full_rows(&mut self, delta:f32) {
        self.clear_row_timer += delta;
        if self.clear_row_timer > CLEAR_ROW_INTERVAL_MS {
            self.clear_row_timer = 0.0;
            self.board.clear_and_collapse( &self.full_rows );
            // Can we 
            // clear animation flags
            self.full_rows = Vec::new(); 
            self.clear_row_timer = 0.0;
            self.flash_anim_color = WHITE;
            self.play_state = PlayState::Playing;
        }
    }
}