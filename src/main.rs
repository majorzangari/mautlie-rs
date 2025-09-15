#![allow(dead_code)]
#![allow(unused)]

use state::fen;

mod state;
mod util;

fn main() {
    let board = fen::fen_to_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    print!("{:?}", board);
}
