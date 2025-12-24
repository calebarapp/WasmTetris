use macroquad::prelude::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PieceKind {
    None
    , T
    , O
    , I
    , Z
    , S
    , J
    , L

}
impl PieceKind {
    pub const fn rotations(&self) -> &'static [[(i32,i32);4]; 4] {
        // TODO: Validate orientation order - especially for T and L
        match self {
            PieceKind::O => &[
                [(0,0),(0,1),(1,0),(1,1)],
                [(0,0),(0,1),(1,0),(1,1)],
                [(0,0),(0,1),(1,0),(1,1)],
                [(0,0),(0,1),(1,0),(1,1)],
            ],
            PieceKind::I => &[
                [(-1,0),(0,0),(1,0),(2,0)],
                [(0,-1),(0,0),(0,1),(0,2)],
                [(-1,0),(0,0),(1,0),(2,0)],
                [(0,-1),(0,0),(0,1),(0,2)],
            ],
            PieceKind::T => &[
                [(0,0),(1,0),(1,-1),(2,0)],
                [(0,0),(0,1),(0,-1),(1,0)],
                [(0,0),(1,0),(1,1),(2,0)],
                [(1,0),(1,1),(1,-1),(0,0)],
            ],
            PieceKind::L => &[
                [(0,0),(0,1),(0,2),(1,2)],
                [(0,1),(1,1),(2,1),(2,0)],
                [(0,0),(1,0),(2,0),(0,1)],
                [(0,0),(1,0),(1,1),(1,2)],
                ],
                PieceKind::J => &[
                [(1,0),(1,1),(1,2),(0,2)],
                [(0,0),(0,1),(1,1),(2,1)],
                [(0,0),(1,0),(0,1),(0,2)],
                [(0,0),(1,0),(2,0),(2,1)],
            ],
            PieceKind::Z => &[
                [(0,0),(1,0),(1,1),(2,1)],
                [(1,0),(1,1),(0,1),(0,2)],
                [(0,0),(1,0),(1,1),(2,1)],
                [(1,0),(1,1),(0,1),(0,2)],
            ],
            PieceKind::S =>&[
                [(0,1),(1,1),(1,0),(2,0)],
                [(0,0),(0,1),(1,1),(1,2)],
                [(0,1),(1,1),(1,0),(2,0)],
                [(0,0),(0,1),(1,1),(1,2)],
            ],
            PieceKind::None =>&[
                [(0,0);4],
                [(0,0);4],
                [(0,0);4],
                [(0,0);4],
            ],
        }
    }
    
    pub const fn color(&self) -> Color {
        // same as before
        match self {
            PieceKind::O    => YELLOW,
            PieceKind::I    => BLUE,
            PieceKind::T    => PURPLE,
            PieceKind::L    => ORANGE,
            PieceKind::J    => GREEN,
            PieceKind::Z    => RED,
            PieceKind::S    => SKYBLUE,
            PieceKind::None => BLACK,
        }
    }
}