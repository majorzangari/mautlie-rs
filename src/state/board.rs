use strum::IntoEnumIterator;

use super::board_move_gen::generate_moves;
use super::{Color, ColoredPiece, GenericPiece, board_move_gen::Move, fen, hash::calculate_hash};
use super::{game_constants, piece_move_gen};
use crate::util::bithelpers::BitFunctions;

#[derive(Debug, Clone)]
pub struct Board {
    pub pieces: [u64; 12],
    pub occupied: [u64; 2],
    pub piece_table: [Option<GenericPiece>; 64],
    pub side_to_move: Color,
    pub fullmove_clock: u16,
    pub state: BoardState,
    pub past_states: Vec<BoardState>,
}

#[derive(Debug, Clone)]
pub struct BoardState {
    pub halfmove_clock: u16,
    pub en_passant: Option<u8>,
    pub castling_rights: u8,
    pub captured_piece: Option<GenericPiece>,
    pub hash: u64,
}

impl Board {
    /// returns an empty board
    pub fn empty() -> Self {
        Self {
            pieces: [0; 12],
            occupied: [0; 2],
            piece_table: [None; 64],
            side_to_move: Color::White,
            fullmove_clock: 1,
            state: BoardState {
                halfmove_clock: 0,
                en_passant: None,
                castling_rights: 0,
                captured_piece: None,
                hash: 0,
            },
            past_states: Vec::new(),
        }
    }

    pub fn default_setup() -> Self {
        fen::fen_to_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }

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
        assert_eq!(
            self.occupied[Color::White as usize],
            calculated_occupied[Color::White as usize],
            "White occupied bitboard does not match pieces"
        );
        assert_eq!(
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
            assert_eq!(
                state_square, calc_square,
                "Piece table does not match pieces at index {}. state = {:?}, calculated = {:?}",
                i, state_square, calc_square
            );
        }

        assert!(
            match self.state.en_passant {
                Some(sq) => sq < 64,
                None => true,
            },
            "En passant square out of range: {:?}",
            self.state.en_passant
        );

        assert!(
            !self.state.castling_rights.contains(0xF0),
            "Invalid castling rights: {:b}",
            self.state.castling_rights
        );

        assert_eq!(
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

    pub fn generate_moves(&self) -> Vec<Move> {
        generate_moves(self)
    }

    pub fn make_move(&mut self, m: Move) -> Result<(), String> {
        todo!()
    }

    /// returns true if the given index is under attack by any piece of the given color
    /// does not consider en passant
    pub fn index_in_check(&self, index: u32, by_color: Color) -> bool {
        let friendly = self.occupied[Color::opposite(by_color) as usize];
        let enemy = self.occupied[by_color as usize];

        let potential_rook_attacks = piece_move_gen::get_rook_moves_bb(index, friendly, enemy);
        let rooks_and_queens = self.pieces
            [ColoredPiece::from_parts(by_color, GenericPiece::Rook) as usize]
            | self.pieces[ColoredPiece::from_parts(by_color, GenericPiece::Queen) as usize];
        if rooks_and_queens.contains(potential_rook_attacks) {
            return true;
        }

        let potential_bishop_attacks = piece_move_gen::get_bishop_moves_bb(index, friendly, enemy);
        let bishops_and_queens = self.pieces
            [ColoredPiece::from_parts(by_color, GenericPiece::Bishop) as usize]
            | self.pieces[ColoredPiece::from_parts(by_color, GenericPiece::Queen) as usize];
        if bishops_and_queens.contains(potential_bishop_attacks) {
            return true;
        }

        let potential_knight_attacks = piece_move_gen::get_knight_moves_bb(index, friendly);
        let knights =
            self.pieces[ColoredPiece::from_parts(by_color, GenericPiece::Knight) as usize];
        if knights.contains(potential_knight_attacks) {
            return true;
        }

        let potential_king_attacks = piece_move_gen::get_king_moves_bb(index, friendly);
        let king = self.pieces[ColoredPiece::from_parts(by_color, GenericPiece::King) as usize];
        if king.contains(potential_king_attacks) {
            return true;
        }

        let index_bb = 1u64 << index;
        let pawns = self.pieces[ColoredPiece::from_parts(by_color, GenericPiece::Pawn) as usize];
        let potential_pawn_attacks = match by_color {
            Color::White => {
                ((index_bb >> 9) & !game_constants::FILE_H)
                    | ((index_bb >> 7) & !game_constants::FILE_A)
            }
            Color::Black => {
                ((index_bb << 7) & !game_constants::FILE_H)
                    | ((index_bb << 9) & !game_constants::FILE_A)
            }
        };

        if pawns.contains(potential_pawn_attacks) {
            return true;
        }

        false
    }
}
