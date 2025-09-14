pub mod const_rand;

pub fn clear_lsb(bitboard: &mut u64) {
    *bitboard &= *bitboard - 1;
}
