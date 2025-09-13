pub mod board;

#[repr(u8)]
#[derive(Debug, Clone)]
pub enum GenericPiece {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

#[repr(usize)]
#[derive(Debug, Clone)]
pub enum Color {
    White = 0,
    Black = 1,
}

#[repr(usize)]
#[derive(Debug, Clone)]
pub enum ColoredPiece {
    WhitePawn = 0,
    BlackPawn = 1,
    WhiteKnight = 2,
    BlackKnight = 3,
    WhiteBishop = 4,
    BlackBishop = 5,
    WhiteRook = 6,
    BlackRook = 7,
    WhiteQueen = 8,
    BlackQueen = 9,
    WhiteKing = 10,
    BlackKing = 11,
}

pub mod castling_rights {
    pub const WHITE_KINGSIDE: u8 = 0b0001;
    pub const WHITE_QUEENSIDE: u8 = 0b0010;
    pub const BLACK_KINGSIDE: u8 = 0b0100;
    pub const BLACK_QUEENSIDE: u8 = 0b1000;
    pub const WHITE_ALL: u8 = WHITE_KINGSIDE | WHITE_QUEENSIDE;
    pub const BLACK_ALL: u8 = BLACK_KINGSIDE | BLACK_QUEENSIDE;
    pub const ALL: u8 = WHITE_ALL | BLACK_ALL;
}
