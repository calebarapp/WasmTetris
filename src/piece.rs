use macroquad::prelude::*;
use crate::constants::*;
use crate::board::Board;
use crate::piece_kind::*;
use macroquad::rand::gen_range;
const KICKS_OFFSETS: [(i32, i32); 5] = [
    (0,0),
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
];

struct Pose {
    row: i32,
    col:i32,
    orientation: u8
}
pub enum RotDir { CW,CCW }

#[derive(Copy, Clone, Debug)] 
pub struct Piece {
    pub kind: PieceKind,
    pub row:i32,
    pub col:i32,
    pub orientation: u8 //0,1,2,3
}
impl Piece {
    pub fn default() -> Self {
        Self {
            kind: PieceKind::None,
            row: 0,
            col: 0,
            orientation: 0
        }
    }
    pub fn new(kind: PieceKind, row:i32, col:i32) -> Self {
        Piece { kind, col, row, orientation:0 }
    }
    pub fn random_piece() -> Piece {
        let n = gen_range(0, 7);
        let mut kind = PieceKind::None;
        match n {
            1 => kind = PieceKind::T,
            2 => kind = PieceKind::O,
            3 => kind = PieceKind::I,
            4 => kind = PieceKind::Z,
            5 => kind = PieceKind::S,
            6 => kind = PieceKind::J,
            7 => kind = PieceKind::L,
            _ => {}
        }
        Piece::new( kind, 0, DEFAULT_SPAWN_COL )
    }

    pub fn cells(&self) -> Vec<(i32, i32)> {
        self.kind.rotations()[self.orientation as usize].iter()
            .map(move | (dc,dr) | (self.col + dc, self.row + dr)).collect()
        
    }
    pub fn can_move(&self, dcol: i32, drow: i32, board: &Board) -> bool {
        for (col, row) in self.cells() {
            let new_col = col + dcol;
            let new_row = row + drow;

            if !board.in_bounds(new_col, new_row) {
                return false;
            }
            // Check board occupancy
            if board.cell_filled(new_col, new_row) {
                return false
            }   
        }
        true
    }
    pub fn try_move_piece(&mut self, col:i32, row: i32, board: &Board) -> bool {
        if self.can_move(col, row, board) {
            self.col += col;
            self.row += row;
            return true;
        }
        return false;
    }
    pub fn try_rotate( &mut self, dir: RotDir, board: &Board) -> bool {
        let new_orient = match dir {
            RotDir::CW => (self.orientation + 1) & 3,
            RotDir::CCW => (self.orientation + 3) & 3
        };

        let base = Pose { col: self.col, row: self.row, orientation: new_orient };
        info!( "rotate: [{}]", new_orient );
        self.try_offsets( board, base )
    }

    fn fits_at(&self, board: &Board, pose: &Pose) -> bool {
        for &(dc, dr) in self.kind.rotations()[pose.orientation as usize].iter() {
            let c = pose.col + dc;
            let r = pose.row + dr;
            if !board.in_bounds(c, r) { return false; }
            if board.cell_filled(c, r) { return false; }
        }
        true
    }

    fn try_offsets(
        &mut self,
        board: &Board,
        base: Pose,
    ) -> bool {
        for &(kcol, krow) in &KICKS_OFFSETS {
            let candidate = Pose {
                col: base.col + kcol,
                row: base.row + krow,
                orientation: base.orientation,
            };
            if self.fits_at(board, &candidate) {
                self.col = candidate.col;
                self.row = candidate.row;
                self.orientation = candidate.orientation;
                return true;
            }
        }
        false
    }

    pub fn try_kick(
        &mut self,
        board: &Board,
    ) -> bool {
        let base = Pose { col: self.col, row: self.row, orientation: self.orientation };
        self.try_offsets( board, base )
    } 
}
