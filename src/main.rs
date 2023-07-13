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
use data::init;
use uci::uci;
//file at  target\<debug|release>\app.exe,
fn main() {
    init();
    uci();
}
