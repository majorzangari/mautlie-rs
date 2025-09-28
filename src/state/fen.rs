use crate::state::ColoredPiece;

use super::board::Board;
use super::board_move_gen;
use super::{Color, castling_rights};

/// parse a FEN string and return a Board representation
pub fn fen_to_board(fen: &str) -> Result<Board, String> {
    let parts: Vec<&str> = fen.split(' ').collect();
    if parts.len() < 4 {
        return Err("FEN string must have at least 4 parts".to_string());
    }

    let mut board = Board::empty();

    parse_fen_pieces(&mut board, parts[0])?;
    parse_fen_side_to_move(&mut board, parts[1])?;
    parse_fen_castling_rights(&mut board, parts[2])?;
    parse_fen_en_passant(&mut board, parts[3])?;

    board.state.halfmove_clock = if let Some(s) = parts.get(4) {
        match s.parse() {
            Ok(n) => n,
            Err(_) => return Err("Invalid halfmove number in FEN string".to_string()),
        }
    } else {
        0
    };

    board.fullmove_clock = if let Some(s) = parts.get(5) {
        match s.parse() {
            Ok(n) => n,
            Err(_) => return Err("Invalid fullmove number in FEN string".to_string()),
        }
    } else {
        1
    };

    board.recalculate_hash();

    board.check_representation();
    Ok(board)
}

/// parse the piece placement part of a FEN string and update the board accordingly
#[rustfmt::skip]
fn parse_fen_pieces(board: &mut Board, pieces: &str) -> Result<(), String> {
    let mut rank = 7;
    let mut file = 0;
    for c in pieces.chars() {
        let square_index = rank * 8 + file;
        match c {
            'P' => board.pieces[ColoredPiece::WhitePawn   as usize] |= 1 << square_index,
            'N' => board.pieces[ColoredPiece::WhiteKnight as usize] |= 1 << square_index,
            'B' => board.pieces[ColoredPiece::WhiteBishop as usize] |= 1 << square_index,
            'R' => board.pieces[ColoredPiece::WhiteRook   as usize] |= 1 << square_index,
            'Q' => board.pieces[ColoredPiece::WhiteQueen  as usize] |= 1 << square_index,
            'K' => board.pieces[ColoredPiece::WhiteKing   as usize] |= 1 << square_index,
            'p' => board.pieces[ColoredPiece::BlackPawn   as usize] |= 1 << square_index,
            'n' => board.pieces[ColoredPiece::BlackKnight as usize] |= 1 << square_index,
            'b' => board.pieces[ColoredPiece::BlackBishop as usize] |= 1 << square_index,
            'r' => board.pieces[ColoredPiece::BlackRook   as usize] |= 1 << square_index,
            'q' => board.pieces[ColoredPiece::BlackQueen  as usize] |= 1 << square_index,
            'k' => board.pieces[ColoredPiece::BlackKing   as usize] |= 1 << square_index,
            '0'..='8' => {
                let empty_squares = c.to_digit(10).ok_or("Invalid digit in FEN string")?;
                file += empty_squares;
                continue;
            }
            '/' => {
                rank -= 1;
                file = 0;
                continue;
            }
            _ => return Err("Invalid character in FEN string".to_string()),
        }
        file += 1;
    }
    board.update_occupied();
    board.update_piece_table();
    Ok(())
}

/// parse the side to move part of a FEN string and update the board accordingly
fn parse_fen_side_to_move(board: &mut Board, side: &str) -> Result<(), String> {
    match side {
        "w" => {
            board.side_to_move = Color::White;
            Ok(())
        }
        "b" => {
            board.side_to_move = Color::Black;
            Ok(())
        }
        _ => Err("Invalid side to move in FEN string".to_string()),
    }
}

/// parse the castling rights part of a FEN string and update the board accordingly
fn parse_fen_castling_rights(board: &mut Board, rights: &str) -> Result<(), String> {
    for c in rights.chars() {
        match c {
            'K' => board.state.castling_rights |= castling_rights::WHITE_SHORT,
            'Q' => board.state.castling_rights |= castling_rights::WHITE_LONG,
            'k' => board.state.castling_rights |= castling_rights::BLACK_SHORT,
            'q' => board.state.castling_rights |= castling_rights::BLACK_LONG,
            '-' => {}
            _ => return Err("Invalid castling rights in FEN string".to_string()),
        }
    }
    Ok(())
}

/// parse the en passant target square part of a FEN string and update the board accordingly
fn parse_fen_en_passant(board: &mut Board, en_passant: &str) -> Result<(), String> {
    if en_passant == "-" {
        board.state.en_passant = None;
        return Ok(());
    }
    let file = en_passant
        .chars()
        .next()
        .ok_or("Invalid en passant in FEN string")?;
    let rank = en_passant
        .chars()
        .nth(1)
        .ok_or("Invalid en passant in FEN string")?;

    let file_chars = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    let file_index = file_chars
        .iter()
        .position(|&c| c == file)
        .ok_or("Invalid file in en passant")?;
    let rank_index = rank.to_digit(10).ok_or("Invalid rank in en passant")? as usize;
    let square_index = rank_index * 8 + file_index;
    if square_index > 63 {
        return Err("En passant square out of bounds".to_string());
    }

    board.state.en_passant = Some(square_index as u8);
    Ok(())
}

pub fn board_to_fen(board: &Board) -> String {
    todo!();
}
