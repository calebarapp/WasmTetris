use crate::board::*;
use crate::piece::*;
use crate::constants::*;
use macroquad::prelude::*;
use crate::piece_kind::*;
use crate::button::*;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Actions {
    None,
    Moved,
    Rotate,
    SoftDrop,
    HardDrop,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum ClearResult {
    None,
    Single,
    Double,
    Triple,
    Tetris,
    TSpinMini,
    TSpinSingle,
    TSpinDouble,
    TSpinTriple,
}
pub enum PlayState {
    Start,
    Playing,
    ClearBlocks,
    Paused,
    GameOver
}
pub struct GameState {
    // public
    pub play_state: PlayState,
    pub board: Board,
    pub current_piece: Piece,
    pub next_piece: Piece,
    pub flash_anim_color: Color,
    pub score: i32,

    // gamestate
    player_interacting: bool,
    pre_lock_moves:u8,
    last_action: Actions,
    level: i32,
    last_clear_result: ClearResult,
    back_to_back: bool,
    lines_cleared:i32,

    // timing
    fall_timer: f32,
    input_timer:f32,
    lock_delta: f32,
    clear_row_timer: f32,
    clear_row_flash_timer: f32,
    
    
}
impl Default for GameState {
    fn default() -> Self{
        Self {
            play_state: PlayState::Start,            
            board: Board::new(),
            current_piece: Piece::default(),
            next_piece: Piece::default(),
            
            player_interacting: false,
            pre_lock_moves:0,
            score:0,
            back_to_back: false,
            last_clear_result: ClearResult::None,
            last_action: Actions::None,
            level:1,
            lines_cleared: 0,

            fall_timer: 0.0,
            input_timer: 0.0,
            lock_delta: 0.0,
            clear_row_timer: 0.0,
            clear_row_flash_timer: 0.0,
            flash_anim_color: WHITE,


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
            self.next_piece = self.get_next_piece();
            self.play_state = PlayState::Playing;
        }
    }
    
    // ===================================================
    //Playing
    // ===================================================
    fn exec_playing_frame(&mut self, dt: f32) {
        if self.current_piece.kind == PieceKind::None {
            self.spawn_next_piece_piece();
        }
        self.handle_input_playing(dt);
        self.try_drop_current_piece(dt);
        self.try_piece_lock(dt);
        self.try_clear_lines(); 
    }
    // handle user input
    fn handle_input_playing(&mut self, delta: f32) {
        self.player_interacting = false;
        let press_action = self.process_key_press();
        let hold_actions= self.process_key_hold(delta);
        // HardDrop > SoftDrop > Move > rotate
        let mut new_state = Actions::None;
        if press_action == Actions::HardDrop {
            new_state = press_action;
        } else if hold_actions == Actions::SoftDrop ||  hold_actions == Actions::Moved {
            new_state = hold_actions;
        } else {
            new_state = press_action;
        }
        // idle
        if new_state != Actions::None {
            self.last_action = new_state;
        }
    }

    fn process_key_hold(&mut self, delta: f32) -> Actions {
        self.input_timer += delta;
        if self.input_timer < INPUT_INTERVAL_MS {
            return Actions::None;
        }
        self.input_timer -= INPUT_INTERVAL_MS;
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            self.move_left();
            return Actions::Moved;
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            self.move_right();
            return Actions::Moved;
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            self.soft_drop();
            return Actions::SoftDrop;
        }
        return Actions::None;
    }

    fn process_key_press(&mut self) -> Actions {
        self.player_interacting = false;
        // Rotate CW
        if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::X) {
            self.player_interacting = true;
            if self.current_piece.try_rotate( RotDir::CW, &self.board ) {
                return Actions::Rotate;
            }
        }
        // Rotate CCW
        if is_key_pressed(KeyCode::Z) {
            self.player_interacting = true;
            if self.current_piece.try_rotate(RotDir::CCW, &self.board ) {
                return Actions::Rotate;
            }
        }
        // HardDrop
        if is_key_pressed(KeyCode::Space) {
            self.player_interacting = true;
            // Todo: Hard drop
        }
        return Actions::None;
    }

    fn soft_drop(&mut self) {
        if self.current_piece.try_move_piece( 0, 1, &self.board ) {
            // SCORE: +1 Points for soft drop
            self.update_score(1);
        }
    }
    fn move_right(&mut self) {
        //move right
        self.current_piece.try_move_piece( 1, 0, &self.board );
        self.player_interacting = true;
    }
    fn move_left(&mut self) {
        //move left
        self.current_piece.try_move_piece( -1, 0, &self.board );
        self.player_interacting = true;
    }
    fn try_drop_current_piece(&mut self, delta: f32) {
        // Move block
        self.fall_timer += delta;
        if self.fall_timer < FALL_INTERVAL_MS {
            return
        }
        self.fall_timer -= FALL_INTERVAL_MS;    
        self.clear_lock_timer();
        self.current_piece.try_move_piece( 0, 1, &self.board );
    }

    fn update_score(&mut self, points: i32) {
        self.score += points;
    }
    fn clear_lock_timer(&mut self) {
        self.lock_delta = 0.0;
    }
    fn try_piece_lock(&mut self, delta: f32) {
        // is grounded?
        if self.current_piece.can_move(0, 1, &self.board) {
            return;
        }
        //Invariant: Piece has Lock pending
        // current_piecely interacting? Allow 15 movements before locking
        if self.player_interacting && self.pre_lock_moves < PRE_LOCK_MOVES_ALLOWED {
            self.pre_lock_moves += 1;
            self.clear_lock_timer();
            return;
        }
        // Lock piece
        self.lock_delta += delta;
        if self.lock_delta > LOCK_DELTA_THRESHOLD {
            // lock! Save to board and clear current_piece piece
            self.pre_lock_moves = 0;
            self.board.lock_piece( &self.current_piece );
            self.current_piece = Piece::default();
        }
    }
    fn get_next_piece(&mut self) -> Piece {
        let mut piece = Piece::random_piece();
        piece.try_kick( &self.board );
        piece
    }
    fn spawn_next_piece_piece(&mut self) {
        self.current_piece = self.next_piece;
        self.next_piece = self.get_next_piece();
        if !self.next_piece.can_move( 0,0, &self.board) {
            self.play_state = PlayState::GameOver
        }
    }
    fn try_clear_lines(&mut self) {
        let full_rows = self.board.full_rows();
        let line_cnt = full_rows.len() as i32;
        self.lines_cleared += line_cnt;
        // calculate score
        let t_spin = self.last_action == Actions::Rotate && self.board.piece_surrounded(&self.current_piece);
        let clear_result = if t_spin {
            match line_cnt {
                0 => ClearResult::TSpinMini,
                1 => ClearResult::TSpinSingle,
                2 => ClearResult::TSpinDouble,
                3 => ClearResult::TSpinTriple,
                _ => ClearResult::None, // This should never happen unless t_spin calc is broken.
            }
        } else {
            match line_cnt {
                1 => ClearResult::Single,
                2 => ClearResult::Double,
                3 => ClearResult::Triple,
                4 => ClearResult::Tetris,
                _ => ClearResult::None,
            }
        };
        let base_points = match clear_result {
            ClearResult::Single        => 100,
            ClearResult::Double        => 300,
            ClearResult::Triple        => 500,
            ClearResult::Tetris        => 800,
            ClearResult::TSpinMini     => 100,   // or 0, depending on your rules
            ClearResult::TSpinSingle   => 800,
            ClearResult::TSpinDouble   => 1200,
            ClearResult::TSpinTriple   => 1600,
            ClearResult::None          => 0,
        };
        let mut score = base_points * self.level;
        // No lines cleared
        // Back-to-back mode when:
        // * Tetris, or T-Spin happens
        // * Future Tetris or T-Spin will score +50%
        let is_b2b = matches!(
            clear_result 
            , ClearResult::Tetris
                | ClearResult::TSpinSingle
                | ClearResult::TSpinDouble
                | ClearResult::TSpinTriple ); 
        if is_b2b {
            if self.back_to_back {
                score += score / 2;

            } else {
                self.back_to_back = true;
            }
        } 
        else 
        {
            self.back_to_back = false;
        }
        // update score
        self.update_score( score );
        // set previous
        self.last_clear_result = clear_result;
        // Update level
        self.level = self.lines_cleared / 10 + 1;
        if line_cnt > 0 {
            info!( "level:[{}], lines_cleared:[{}], score:[{}]", self.level, self.lines_cleared, score );
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
            for &i in &self.board.full_rows() {
                self.board.set_row_color( self.flash_anim_color ,i );
            }
        }
    }
    fn try_clear_full_rows(&mut self, delta:f32) {
        self.clear_row_timer += delta;
        if self.clear_row_timer > CLEAR_ROW_INTERVAL_MS {
            self.clear_row_timer = 0.0;
            self.board.clear_and_collapse();
            // Can we 
            // clear animation flags
            self.clear_row_timer = 0.0;
            self.flash_anim_color = WHITE;
            self.play_state = PlayState::Playing;
        }
    }
}