#![allow(dead_code)]
#![allow(unused)]

use state::{board::Board, fen};

mod state;
mod util;

fn main() {
    state::init();

    let board = Board::default_setup();

    let moves = board.generate_moves();
    for (i, cmove) in moves.iter().enumerate() {
        println!("Move {}: {}", i + 1, cmove.to_uci());
    }
}
