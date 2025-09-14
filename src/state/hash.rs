use crate::util::const_rand;

use super::{Color, ColoredPiece, GenericPiece};

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
    0xbb09af174b919702 // arbitrary
}

const fn generate_castle_hash() -> [u64; 4] {
    let mut table = [0; 4];
    let mut seed = 0x3f30da02e189b20d;

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
    let mut seed = 0x65e4cba05505ca9c;

    let mut rng = const_rand::XorShift64::new(seed);

    let mut i = 0;
    while i < 8 {
        table[i] = rng.next();
        i += 1;
    }
    table
}
