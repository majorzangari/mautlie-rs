#![allow(dead_code)]
#![allow(unused)]

use state::{board::Board, fen};

mod state;
mod util;

fn main() {
    state::init();

    let board = Board::default_setup();
    print!("{:?}", board);
}
