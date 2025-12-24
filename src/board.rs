use crate::constants::*;
use macroquad::prelude::*;
use crate::piece::Piece;


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FilledStruct {
    pub col: i32,
    pub row: i32,
    pub color: Color
}
impl FilledStruct {
    pub fn new(col: i32, row: i32, color: Color) -> Self {
        Self { col, row, color }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Cell {
    Empty,
    Filled(Color) 
}

pub struct Board {
    height: usize,
    width: usize,
    cells: Vec<Cell>
}

impl Board {
    pub fn new() -> Self {
        //cells
        Self {
            height: GRID_H,
            width: GRID_W,
            cells: vec![Cell::Empty; GRID_H as usize * GRID_W as usize],
        }
    }
    pub fn in_bounds(&self, col: i32, row: i32) -> bool {
        col >= 0 
        && row >= 0 
        && col < GRID_W as i32 
        && row < GRID_H as i32 
    }
    pub fn lock_piece( &mut self, piece: &Piece) {
        for (col, row) in piece.cells() {
            // we use an vector as a flattened grid - we need to calculate position
            let idx = (row as usize * GRID_W ) + col as usize;
            self.cells[idx] = Cell::Filled(piece.kind.color());
        }
    }
    pub fn filled_cells(&self) -> Vec<FilledStruct>{
        let mut out = Vec::new(); 
        for (i, cell) in self.cells.iter().enumerate() {
            if let Cell::Filled(color) = cell {
                // this cell is filled - calculate grid location
                let row = (i / self.width ) as i32;
                let col = (i % self.width) as i32;
                out.push( FilledStruct::new( col, row, *color ) );
            }
        }
        out
    }
    fn cell_idx(&self, col:i32,row: i32) -> usize {
        (row as usize * GRID_W ) + col as usize
    }
    pub fn cell_filled(&self, col:i32, row:i32) -> bool {
        self.cells[self.cell_idx(col, row)] != Cell::Empty
    }
    pub fn full_rows(&self) -> Vec<i32> {
        // search from bottom of board to top checking for complete lines
        // abort search through coloumns when empty column is found
        let mut out: Vec<i32> = Vec::new();
        // error in here...
        let row_cnt = GRID_H as i32;
        let col_cnt = GRID_W as i32;
        for row in (0..row_cnt).rev() {
            let mut row_filled: bool = true;
            for col in (0..col_cnt as i32) {
                if !self.cell_filled(col, row) {
                    row_filled = false;
                    break;
                }
            }
            if row_filled {
                info!( "Full row: {}", row );
                out.push(row);
            }
        }
        out
    }
    
    pub fn clear_and_collapse(&mut self) {
        let rows = self.full_rows();
        if rows.is_empty() { return; }

        let mut cleared = vec![false; GRID_H as usize];
        for row in rows {
            if (0..GRID_H as i32).contains(&row) {
                cleared[row as usize] = true;
            }
        }

        let mut write = GRID_H - 1;
        for read in (0..GRID_H).rev() {
            if cleared[read] {
                continue; // disapear
            }
            if read != write {
                for col in 0..GRID_W as i32 {
                    let src = self.cell_idx(col, read as i32);
                    let dst = self.cell_idx(col, write as i32);
                    self.cells[dst] = self.cells[src];
                }
            }
            write -= 1;
        } 

    }
    pub fn set_row_color(&mut self, color: Color, row: i32) {
        for col in (0..GRID_W as i32) {
            let idx = self.cell_idx(col, row);
            self.cells[idx] = Cell::Filled(color);
        }
    }
    
}