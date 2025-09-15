use strum::IntoEnumIterator;

use crate::util::{bithelpers::BitFunctions, const_rand};

use super::{Color, ColoredPiece, GenericPiece, board::Board, castling_rights};

pub const PIECE_HASH: [[u64; 12]; 64] = generate_piece_hash();
pub const SIDE_HASH: u64 = generate_side_hash();
pub const CASTLE_HASH: [u64; 4] = generate_castle_hash();
pub const EN_PASSANT_HASH: [u64; 8] = generate_en_passant_hash();

const fn generate_piece_hash() -> [[u64; 12]; 64] {
    let mut table = [[0; 12]; 64];
    let mut seed = 0x1b9013473f4957ca; // arbitrary

    let mut rng = const_rand::XorShift64::new(seed);

    let mut piece_index = 0;
    let mut square_index = 0;
    while piece_index < 12 {
        while square_index < 64 {
            table[square_index][piece_index] = rng.next();
            square_index += 1;
        }
        square_index = 0;
        piece_index += 1;
    }

    table
}

const fn generate_side_hash() -> u64 {
    let seed = 0xbb09af174b919702; // arbitrary
    let mut rng = const_rand::XorShift64::new(seed);
    rng.next()
}

const fn generate_castle_hash() -> [u64; 4] {
    let mut table = [0; 4];
    let mut seed = 0x3f30da02e189b20d; // arbitrary

    let mut rng = const_rand::XorShift64::new(seed);

    let mut i = 0;
    while i < 4 {
        table[i] = rng.next();
        i += 1;
    }
    table
}

const fn generate_en_passant_hash() -> [u64; 8] {
    let mut table = [0; 8];
    let mut seed = 0x65e4cba05505ca9c; // arbitrary

    let mut rng = const_rand::XorShift64::new(seed);

    let mut i = 0;
    while i < 8 {
        table[i] = rng.next();
        i += 1;
    }
    table
}

/// returns the hash of the given castling right
fn get_castling_hash(castling_rights: u8) -> u64 {
    let mut hash = 0;
    if castling_rights.contains(castling_rights::WHITE_SHORT) {
        hash ^= CASTLE_HASH[0];
    }
    if castling_rights.contains(castling_rights::WHITE_LONG) {
        hash ^= CASTLE_HASH[1];
    }
    if castling_rights.contains(castling_rights::BLACK_SHORT) {
        hash ^= CASTLE_HASH[2];
    }
    if castling_rights.contains(castling_rights::BLACK_LONG) {
        hash ^= CASTLE_HASH[3];
    }
    hash
}

/// returns the Zobrist hash of the given board position
pub fn calculate_hash(board: &Board) -> u64 {
    let mut hash = 0;

    for piece in ColoredPiece::iter() {
        let mut bb = board.pieces[piece as usize];
        while bb != 0 {
            let lsb_index = bb.pop_lsb() as usize;
            hash ^= PIECE_HASH[lsb_index][piece as usize];
        }
    }

    if matches!(board.side_to_move, Color::Black) {
        hash ^= SIDE_HASH;
    }

    hash ^= get_castling_hash(board.state.castling_rights);

    if board.state.en_passant != 0 {
        let file = board.state.en_passant.trailing_zeros() as usize % 8;
        hash ^= EN_PASSANT_HASH[file];
    }

    hash
}
