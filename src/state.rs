pub mod board;
pub mod fen;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum GenericPiece {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

#[repr(usize)]
#[derive(Debug, Clone, Copy)]
pub enum Color {
    White = 0,
    Black = 1,
}

#[repr(usize)]
#[derive(Debug, Clone, Copy)]
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

impl ColoredPiece {
    pub fn color(self) -> Color {
        if self as u8 % 2 == 0 {
            Color::White
        } else {
            Color::Black
        }
    }

    pub fn generic(self) -> GenericPiece {
        match self {
            ColoredPiece::WhitePawn | ColoredPiece::BlackPawn => GenericPiece::Pawn,
            ColoredPiece::WhiteKnight | ColoredPiece::BlackKnight => GenericPiece::Knight,
            ColoredPiece::WhiteBishop | ColoredPiece::BlackBishop => GenericPiece::Bishop,
            ColoredPiece::WhiteRook | ColoredPiece::BlackRook => GenericPiece::Rook,
            ColoredPiece::WhiteQueen | ColoredPiece::BlackQueen => GenericPiece::Queen,
            ColoredPiece::WhiteKing | ColoredPiece::BlackKing => GenericPiece::King,
        }
    }

    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(ColoredPiece::WhitePawn),
            1 => Some(ColoredPiece::BlackPawn),
            2 => Some(ColoredPiece::WhiteKnight),
            3 => Some(ColoredPiece::BlackKnight),
            4 => Some(ColoredPiece::WhiteBishop),
            5 => Some(ColoredPiece::BlackBishop),
            6 => Some(ColoredPiece::WhiteRook),
            7 => Some(ColoredPiece::BlackRook),
            8 => Some(ColoredPiece::WhiteQueen),
            9 => Some(ColoredPiece::BlackQueen),
            10 => Some(ColoredPiece::WhiteKing),
            11 => Some(ColoredPiece::BlackKing),
            _ => None,
        }
    }
}

pub mod castling_rights {
    pub const WHITE_SHORT: u8 = 0b0001;
    pub const WHITE_LONG: u8 = 0b0010;
    pub const BLACK_SHORT: u8 = 0b0100;
    pub const BLACK_LONG: u8 = 0b1000;
    pub const WHITE_ALL: u8 = WHITE_SHORT | WHITE_LONG;
    pub const BLACK_ALL: u8 = BLACK_SHORT | BLACK_LONG;
    pub const ALL: u8 = WHITE_ALL | BLACK_ALL;
}
