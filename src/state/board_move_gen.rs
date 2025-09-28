use super::{ColoredPiece, GenericPiece, board::Board, castling_rights, game_constants};
use crate::{
    state::{Color, move_gen_constants, piece_move_gen},
    util::bithelpers::BitFunctions,
};

/// 6 bits from, 6 bits to, 4 bits flags
#[derive(Debug)]
pub struct Move {
    data: u16,
}

pub mod move_flags {
    pub const QUIET: u8 = 0;
    pub const DOUBLE_PAWN_PUSH: u8 = 1;
    pub const SHORT_CASTLE: u8 = 2;
    pub const LONG_CASTLE: u8 = 3;
    pub const CAPTURE: u8 = 4;
    pub const EN_PASSANT: u8 = 5;
    pub const KNIGHT_PROMO: u8 = 8;
    pub const BISHOP_PROMO: u8 = 9;
    pub const ROOK_PROMO: u8 = 10;
    pub const QUEEN_PROMO: u8 = 11;
    pub const KNIGHT_PROMO_CAPTURE: u8 = 12;
    pub const BISHOP_PROMO_CAPTURE: u8 = 13;
    pub const ROOK_PROMO_CAPTURE: u8 = 14;
    pub const QUEEN_PROMO_CAPTURE: u8 = 15;

    pub const PROMO_FLAGS: u8 = 8;
    pub const PROMO_CAPTURE_FLAGS: u8 = 12;
}

const FROM_MASK: u16 = 0b1111110000000000;
const TO_MASK: u16 = 0b0000001111110000;
const FLAGS_MASK: u16 = 0b0000000000001111;

impl Move {
    pub fn from_parts(from: u8, to: u8, flags: u8) -> Move {
        Move {
            data: ((from as u16) << 10) | ((to as u16) << 4) | (flags as u16),
        }
    }

    pub fn from_data(data: u16) -> Move {
        Move { data }
    }

    pub fn extract_from(&self) -> u16 {
        (self.data & FROM_MASK) >> 10
    }

    pub fn extract_to(&self) -> u16 {
        (self.data & TO_MASK) >> 4
    }

    pub fn extract_flags(&self) -> u16 {
        self.data & FLAGS_MASK
    }

    pub fn to_uci(&self) -> String {
        let from = square_to_algebraic(self.extract_from() as u8);
        let to = square_to_algebraic(self.extract_to() as u8);
        let mut out = format!("{}{}", from, to);

        match self.extract_flags() as u8 {
            move_flags::KNIGHT_PROMO | move_flags::KNIGHT_PROMO_CAPTURE => out.push('n'),
            move_flags::BISHOP_PROMO | move_flags::BISHOP_PROMO_CAPTURE => out.push('b'),
            move_flags::ROOK_PROMO | move_flags::ROOK_PROMO_CAPTURE => out.push('r'),
            move_flags::QUEEN_PROMO | move_flags::QUEEN_PROMO_CAPTURE => out.push('q'),
            _ => {}
        }

        out
    }
}

/// returns the algebraic notation (e.g. "e4") for a given square index (0-63)
fn square_to_algebraic(square: u8) -> String {
    debug_assert!(square < 64);
    let file = square % 8;
    let rank = square / 8;
    let file_char = (b'a' + file) as char;
    let rank_char = (b'1' + rank) as char;
    format!("{}{}", file_char, rank_char)
}

/// for each set bit in pushes, appends a corresponding pawn push from the square shift indices
/// behind to the square of the set bit
fn generate_pawn_pushes(side: Color, pushes: u64, shift: u32, out: &mut Vec<Move>) {
    pushes.bit_for_each(|to_index| {
        let from_index = match side {
            Color::White => to_index - shift,
            Color::Black => to_index + shift,
        };

        let is_promo = (game_constants::RANK_8 & (1u64 << to_index) != 0)
            || (game_constants::RANK_1 & (1u64 << to_index) != 0);

        if is_promo {
            for flag in [
                move_flags::KNIGHT_PROMO,
                move_flags::BISHOP_PROMO,
                move_flags::ROOK_PROMO,
                move_flags::QUEEN_PROMO,
            ] {
                out.push(Move::from_parts(from_index as u8, to_index as u8, flag));
            }
        } else {
            out.push(Move::from_parts(
                from_index as u8,
                to_index as u8,
                move_flags::QUIET,
            ));
        }
    });
}

fn generate_en_passant(board: &Board, out: &mut Vec<Move>) {
    let side = board.side_to_move;
    let pawns = board.pieces[ColoredPiece::from_parts(side, GenericPiece::Pawn) as usize];

    let ep_square = match board.state.en_passant {
        Some(sq) => sq,
        None => return,
    };

    let ep_bit = 1u64 << ep_square;

    let attackers = match side {
        Color::White => {
            ((ep_bit >> 9) & !game_constants::FILE_H) | ((ep_bit >> 7) & !game_constants::FILE_A)
        }
        Color::Black => {
            ((ep_bit << 7) & !game_constants::FILE_H) | ((ep_bit << 9) & !game_constants::FILE_A)
        }
    } & pawns;

    attackers.bit_for_each(|from_index| {
        out.push(Move::from_parts(
            from_index as u8,
            ep_square,
            move_flags::EN_PASSANT,
        ));
    });
}

/// generates all pseudo-legal pawn moves at given board state and appends them to provided vec
fn generate_pawn_moves(board: &Board, out: &mut Vec<Move>) {
    let side = board.side_to_move;
    let friendly = board.occupied[side as usize];
    let enemy = board.occupied[Color::opposite(side) as usize];
    let empty = !(friendly | enemy);

    let pawns = board.pieces[ColoredPiece::from_parts(side, GenericPiece::Pawn) as usize];

    let single_dest = match side {
        Color::White => (pawns << 8) & empty,
        Color::Black => (pawns >> 8) & empty,
    };
    generate_pawn_pushes(side, single_dest, 8, out);

    let double_rank = match side {
        Color::White => game_constants::RANK_4,
        Color::Black => game_constants::RANK_5,
    };

    let double_dest = match side {
        Color::White => (single_dest << 8) & empty & double_rank,
        Color::Black => (single_dest >> 8) & empty & double_rank,
    };
    generate_pawn_pushes(side, double_dest, 16, out);

    generate_en_passant(board, out);
}

fn generate_castling(board: &Board, out: &mut Vec<Move>) {
    let side = board.side_to_move;
    let friendly = board.occupied[side as usize];
    let enemy = board.occupied[Color::opposite(side) as usize];
    let occupied = friendly | enemy;

    let rights = board.state.castling_rights;

    let short_available = match side {
        Color::White => rights.contains(castling_rights::WHITE_SHORT),
        Color::Black => rights.contains(castling_rights::BLACK_SHORT),
    };

    if short_available {
        let clearance_mask = match side {
            Color::White => castling_rights::WHITE_SHORT_CLEARANCE_MASK,
            Color::Black => castling_rights::BLACK_SHORT_CLEARANCE_MASK,
        };

        if !occupied.contains(clearance_mask) {
            let check_mask = match side {
                Color::White => castling_rights::WHITE_SHORT_CHECK_MASK,
                Color::Black => castling_rights::BLACK_SHORT_CHECK_MASK,
            };
            if check_mask.bit_for_all(|sq| !board.index_in_check(sq, Color::opposite(side))) {
                let (king_from, king_to) = match side {
                    Color::White => (4u8, 6u8),
                    Color::Black => (60u8, 62u8),
                };
                out.push(Move::from_parts(
                    king_from,
                    king_to,
                    move_flags::SHORT_CASTLE,
                ));
            }
        }
    }

    let long_available = match side {
        Color::White => rights.contains(castling_rights::WHITE_LONG),
        Color::Black => rights.contains(castling_rights::BLACK_LONG),
    };

    if long_available {
        let clearance_mask = match side {
            Color::White => castling_rights::WHITE_LONG_CLEARANCE_MASK,
            Color::Black => castling_rights::BLACK_LONG_CLEARANCE_MASK,
        };

        if !occupied.contains(clearance_mask) {
            let check_mask = match side {
                Color::White => castling_rights::WHITE_LONG_CHECK_MASK,
                Color::Black => castling_rights::BLACK_LONG_CHECK_MASK,
            };
            if check_mask.bit_for_all(|sq| !board.index_in_check(sq, Color::opposite(side))) {
                let (king_from, king_to) = match side {
                    Color::White => (4u8, 2u8),
                    Color::Black => (60u8, 58u8),
                };
                out.push(Move::from_parts(
                    king_from,
                    king_to,
                    move_flags::LONG_CASTLE,
                ));
            }
        }
    }
}

pub fn generate_moves(board: &Board) -> Vec<Move> {
    let mut out = Vec::with_capacity(move_gen_constants::MAX_PSEUDO_LEGAL_MOVES);
    let side = board.side_to_move;
    let friendly = board.occupied[side as usize];
    let enemy = board.occupied[Color::opposite(side) as usize];

    generate_pawn_moves(board, &mut out);

    board.pieces[ColoredPiece::from_parts(side, GenericPiece::Knight) as usize].bit_for_each(
        |knight_index| {
            let knight_move_bb = piece_move_gen::get_knight_moves_bb(knight_index, friendly);

            knight_move_bb.bit_for_each(|to_index| {
                let is_capture = (board.piece_table[to_index as usize].is_some());
                let flags = if is_capture {
                    move_flags::CAPTURE
                } else {
                    move_flags::QUIET
                };
                out.push(Move::from_parts(knight_index as u8, to_index as u8, flags));
            });
        },
    );

    let effective_bishops = board.pieces
        [ColoredPiece::from_parts(side, GenericPiece::Bishop) as usize]
        | board.pieces[ColoredPiece::from_parts(side, GenericPiece::Queen) as usize];
    effective_bishops.bit_for_each(|bishop_index| {
        let bishop_move_bb = piece_move_gen::get_bishop_moves_bb(bishop_index, friendly, enemy);

        bishop_move_bb.bit_for_each(|to_index| {
            let is_capture = (board.piece_table[to_index as usize].is_some());
            let flags = if is_capture {
                move_flags::CAPTURE
            } else {
                move_flags::QUIET
            };
            out.push(Move::from_parts(bishop_index as u8, to_index as u8, flags));
        });
    });

    let effective_rooks = board.pieces[ColoredPiece::from_parts(side, GenericPiece::Rook) as usize]
        | board.pieces[ColoredPiece::from_parts(side, GenericPiece::Queen) as usize];
    effective_rooks.bit_for_each(|rook_index| {
        let rook_move_bb = piece_move_gen::get_rook_moves_bb(rook_index, friendly, enemy);

        rook_move_bb.bit_for_each(|to_index| {
            let is_capture = (board.piece_table[to_index as usize].is_some());
            let flags = if is_capture {
                move_flags::CAPTURE
            } else {
                move_flags::QUIET
            };
            out.push(Move::from_parts(rook_index as u8, to_index as u8, flags));
        });
    });

    board.pieces[ColoredPiece::from_parts(side, GenericPiece::King) as usize].bit_for_each(
        |king_index| {
            let king_move_bb = piece_move_gen::get_king_moves_bb(king_index, friendly);

            king_move_bb.bit_for_each(|to_index| {
                let is_capture = (board.piece_table[to_index as usize].is_some());
                let flags = if is_capture {
                    move_flags::CAPTURE
                } else {
                    move_flags::QUIET
                };
                out.push(Move::from_parts(king_index as u8, to_index as u8, flags));
            });
        },
    );

    generate_castling(board, &mut out);

    out
}
