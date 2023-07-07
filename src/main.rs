#![warn(dead_code)]
#![warn(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
mod benchmark;
mod board_util;
mod data;
mod evaluate;
mod moves;
mod perft;
mod search;
mod transposition_table;
mod uci;
use crate::{
    board_util::print_move_list,
    data::{get_pst_value, get_spst_value, PIECE_SQUARE_TABLES},
    evaluate::evaluate,
    search::{search_at_fixed_depth, start_search},
};
use benchmark::benchmark;
use board_util::print_bitboard;
use chess::{BitBoard, Board, BoardStatus, ChessMove, Color, MoveGen, Piece, Square};
use data::{init, FRONT_SPANS};
use moves::sort_moves;
use perft::{default_perft, go_perft};
use std::fs;
use std::{str::FromStr, time::Duration};
use transposition_table::TranspositionTable;
use uci::uci;
//file at  target\<debug|release>\app.exe,
fn main() {
    init();
    uci();
}
