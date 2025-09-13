use super::{Color, GenericPiece};

#[derive(Debug, Clone)]
pub struct Board {
    pub pieces: [u64; 12],
    pub occupied: [u64; 2],
    pub piece_table: [GenericPiece; 64],
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
