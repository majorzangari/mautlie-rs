use strum::IntoEnumIterator;

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
        for piece in ColoredPiece::iter() {
            let mut bb = self.pieces[piece as usize];
            while bb != 0 {
                let lsb_index = bb.pop_lsb();
                let generic_piece = piece.generic();
                self.piece_table[lsb_index as usize] = Some(generic_piece);
            }
        }
    }

    /// recalculates and sets the board hash from scratch
    pub fn recalculate_hash(&mut self) {
        self.state.hash = calculate_hash(self);
    }

    /// panics if the board is in an invalid state
    /// does nothing in release builds
    #[cfg(debug_assertions)]
    pub fn check_representation(&self) {
        let mut calculated_occupied = [0; 2];
        for piece in ColoredPiece::iter() {
            calculated_occupied[piece.color() as usize] |= self.pieces[piece as usize];
        }
        debug_assert_eq!(
            self.occupied[Color::White as usize],
            calculated_occupied[Color::White as usize],
            "White occupied bitboard does not match pieces"
        );
        debug_assert_eq!(
            self.occupied[Color::Black as usize],
            calculated_occupied[Color::Black as usize],
            "Black occupied bitboard does not match pieces"
        );

        let mut calculated_piece_table = [None; 64];
        for piece in ColoredPiece::iter() {
            let mut bb = self.pieces[piece as usize];
            while bb != 0 {
                let lsb_index = bb.pop_lsb();
                let generic_piece = piece.generic();
                if let Some(existing) = calculated_piece_table[lsb_index as usize] {
                    panic!(
                        "Multiple pieces on square {}: {:?} and {:?}",
                        lsb_index, existing, generic_piece
                    );
                }
                calculated_piece_table[lsb_index as usize] = Some(generic_piece);
            }
        }

        for (i, (state_square, calc_square)) in self
            .piece_table
            .iter()
            .zip(calculated_piece_table.iter())
            .enumerate()
        {
            debug_assert_eq!(
                state_square, calc_square,
                "Piece table does not match pieces at index {}. state = {:?}, calculated = {:?}",
                i, state_square, calc_square
            );
        }

        debug_assert!(
            self.state.en_passant.count_set_bits() <= 1,
            "Multiple en passant square set: {:b}",
            self.state.en_passant
        );

        debug_assert!(
            !self.state.castling_rights.contains(0xF0),
            "Invalid castling rights: {:b}",
            self.state.castling_rights
        );

        debug_assert_eq!(
            self.state.hash,
            calculate_hash(self),
            "Incorrect hash. state = {}, calculated = {}",
            self.state.hash,
            calculate_hash(self)
        )
    }

    #[cfg(not(debug_assertions))]
    pub fn check_representation(&self) {
        // no-op in release builds
    }
}
