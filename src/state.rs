use strum_macros::EnumIter;

pub mod board;
pub mod board_move_gen;
pub mod fen;
pub mod hash;

mod piece_move_gen;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
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

impl Color {
    pub fn opposite(self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[repr(usize)]
#[derive(Debug, Clone, Copy, EnumIter)]
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

    pub fn from_parts(color: Color, generic: GenericPiece) -> Self {
        let index = (generic as usize) * 2 + (color as usize);
        ColoredPiece::from_index(index).unwrap()
    }
}

pub mod game_constants {
    pub const RANK_1: u64 = 0x00000000000000FF;
    pub const RANK_2: u64 = 0x000000000000FF00;
    pub const RANK_3: u64 = 0x0000000000FF0000;
    pub const RANK_4: u64 = 0x00000000FF000000;
    pub const RANK_5: u64 = 0x000000FF00000000;
    pub const RANK_6: u64 = 0x0000FF0000000000;
    pub const RANK_7: u64 = 0x00FF000000000000;
    pub const RANK_8: u64 = 0xFF00000000000000;

    pub const FILE_A: u64 = 0x0101010101010101;
    pub const FILE_B: u64 = 0x0202020202020202;
    pub const FILE_C: u64 = 0x0404040404040404;
    pub const FILE_D: u64 = 0x0808080808080808;
    pub const FILE_E: u64 = 0x1010101010101010;
    pub const FILE_F: u64 = 0x2020202020202020;
    pub const FILE_G: u64 = 0x4040404040404040;
    pub const FILE_H: u64 = 0x8080808080808080;
}

pub mod move_gen_constants {
    pub const MAX_PSEUDO_LEGAL_MOVES: usize = 256;
}

pub mod castling_rights {
    pub const WHITE_SHORT: u8 = 0b0001;
    pub const WHITE_LONG: u8 = 0b0010;
    pub const BLACK_SHORT: u8 = 0b0100;
    pub const BLACK_LONG: u8 = 0b1000;
    pub const WHITE_ALL: u8 = WHITE_SHORT | WHITE_LONG;
    pub const BLACK_ALL: u8 = BLACK_SHORT | BLACK_LONG;
    pub const ALL: u8 = WHITE_ALL | BLACK_ALL;
    pub const NONE: u8 = 0b0000;

    pub const WHITE_SHORT_CLEARANCE_MASK: u64 = 0b1100000;
    pub const WHITE_LONG_CLEARANCE_MASK: u64 = 0b1110;
    pub const BLACK_SHORT_CLEARANCE_MASK: u64 = WHITE_SHORT_CLEARANCE_MASK << 56;
    pub const BLACK_LONG_CLEARANCE_MASK: u64 = WHITE_LONG_CLEARANCE_MASK << 56;

    pub const WHITE_SHORT_CHECK_MASK: u64 = 0b1110000;
    pub const WHITE_LONG_CHECK_MASK: u64 = 0b11110;
    pub const BLACK_SHORT_CHECK_MASK: u64 = WHITE_SHORT_CHECK_MASK << 56;
    pub const BLACK_LONG_CHECK_MASK: u64 = WHITE_LONG_CHECK_MASK << 56;
}

pub fn init() {
    piece_move_gen::init_magic_info();
}
