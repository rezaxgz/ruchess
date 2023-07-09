mod benchmark;
mod board_util;
// mod book;
mod data;
mod evaluate;
mod moves;
mod perft;
mod search;
mod transposition_table;
mod uci;
use chess::Board;
use data::init;
use evaluate::evaluate;
use std::str::FromStr;
use uci::uci;
//file at  target\<debug|release>\app.exe,
fn main() {
    init();
    let board = Board::from_str("r4rk1/1p2b1p1/p2p4/3PpP2/3p3P/1P1P3P/1P3PB1/R4RK1 b - - 0 1")
        .expect("Valid FEN");
    evaluate(&board);
    uci();
}
