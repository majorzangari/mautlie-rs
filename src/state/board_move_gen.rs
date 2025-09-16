/// 6 bits from, 6 bits to, 4 bits flags
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
            data: ((from as u16) << 12) | ((to as u16) << 6) | (flags as u16),
        }
    }

    pub fn from_data(data: u16) -> Move {
        Move { data }
    }

    pub fn extract_from(&self) -> u16 {
        (self.data & FROM_MASK) >> 12
    }

    pub fn extract_to(&self) -> u16 {
        (self.data & TO_MASK) >> 6
    }

    pub fn extract_flags(&self) -> u16 {
        self.data & FLAGS_MASK
    }
}
