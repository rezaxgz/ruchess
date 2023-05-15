use chess::{Board, ChessMove};
use std::str::FromStr;

use crate::board_util::{init_board, print_board};
const AUTHOR: &str = "Reza";
const ENGINENAME: &str = "ruchess";
pub fn uci() {
    let scanner = std::io::stdin();
    let mut line = String::new();
    let mut board = Board::default();
    let default_baord = Board::default();
    loop {
        line.clear();
        scanner.read_line(&mut line).unwrap();
        let string = line.trim();
        match string {
            "uci" => {
                println!("id name {}", ENGINENAME);
                println!("id author {}", AUTHOR);
            }
            "isready" => println!("readyok"),
            "ucinewgame" => board = Board::default(),
            "quit" => std::process::exit(0),
            "print" => print_board(init_board(board.to_string())),
            a if a.starts_with("position") => {
                if string.starts_with("position fen") {
                    let fen = string[13..string.len()].to_owned();
                    println!("{}", fen);
                    board = Board::from_str(&fen).expect("Valid FEN");
                }
                if string.contains("moves") {
                    let i: usize = string.find("moves").unwrap();
                    let move_list = &string[(i + 5)..string.len()].trim();
                    for m in move_list.split(" ") {
                        println!("{}", m);
                        board = board.make_move_new(ChessMove::from_str(m).unwrap());
                    }
                }
            }
            _ => {}
        }
    }
}
