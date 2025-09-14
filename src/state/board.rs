use super::{Color, ColoredPiece, GenericPiece, hash::calculate_hash};
use crate::util::bithelpers::BitFunctions;

#[derive(Debug, Clone)]
pub struct Board {
    pub pieces: [u64; 12],
    pub occupied: [u64; 2],
    pub piece_table: [Option<GenericPiece>; 64],
    pub side_to_move: Color,
    pub state: BoardState,
    pub past_states: Vec<BoardState>,
}

#[derive(Debug, Clone)]
pub struct BoardState {
    pub halfmove_clock: u16,
    pub en_passant: u64,
    pub castling_rights: u8,
    pub captured_piece: Option<GenericPiece>,
    pub hash: u64,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            pieces: [0; 12],
            occupied: [0; 2],
            piece_table: [None; 64],
            side_to_move: Color::White,
            state: BoardState {
                halfmove_clock: 0,
                en_passant: 0,
                castling_rights: 0,
                captured_piece: None,
                hash: 0,
            },
            past_states: Vec::new(),
        }
    }
}

impl Board {
    /// update the occupied tables for both colors based on the bitboards
    pub fn update_occupied(&mut self) {
        self.occupied[Color::White as usize] = 0;
        self.occupied[Color::Black as usize] = 0;

        for (i, &bitboard) in self.pieces.iter().enumerate() {
            let color = if i % 2 == 0 {
                Color::White
            } else {
                Color::Black
            };
            self.occupied[color as usize] |= bitboard;
        }
    }

    /// update the piece table based on the bitboards
    pub fn update_piece_table(&mut self) {
        self.piece_table = [None; 64];
        for (i, mut bb) in self.pieces.iter().copied().enumerate() {
            while bb != 0 {
                let lsb_index = bb.pop_lsb();
                let piece = ColoredPiece::from_index(i).map(|p| p.generic());
                self.piece_table[lsb_index as usize] = piece;
            }
        }
    }

    /// recalculates and sets the board hash from scratch
    pub fn recalculate_hash(&mut self) {
        self.state.hash = calculate_hash(self);
    }
}
