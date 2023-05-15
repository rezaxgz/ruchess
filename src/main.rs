#![warn(dead_code)]
#![warn(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
mod benchmark;
mod board_util;
mod evaluate;
mod perft;
mod search;
mod uci;
use benchmark::benchmark;
use chess::{Board, BoardStatus, ChessMove, MoveGen, Piece, Square};
use perft::{default_perft, go_perft};
use search::search;
use std::str::FromStr;
fn main() {
    let res = search(
        &Board::from_str("1nn1kb1r/pppppppp/2q5/4r3/4P1b1/5N2/PPPP1PPP/RNBQKB1R w KQk - 0 1")
            .unwrap(),
        6,
    );
    println!("{}", res.best_move.to_string());
    println!("{:?}", res.duration);
}
