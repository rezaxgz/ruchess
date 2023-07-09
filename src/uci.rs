use crate::{
    board_util::{init_board, print_board},
    perft::go_perft,
    search::{search_at_fixed_depth, start_search},
    transposition_table::TranspositionTable,
};
use chess::{Board, ChessMove};
use std::time::Instant;
use std::{str::FromStr, time::Duration};
const AUTHOR: &str = "Reza";
const ENGINENAME: &str = "ruchess";
// fn count_moves(game: &String) -> u8 {
//     let mut count: u8 = 0;
//     for i in game.as_bytes() {
//         if *i == (' ' as u8) {
//             count += 1;
//         }
//     }
//     return count;
// }
pub fn uci() {
    let mut prev_cmd = String::new();
    let scanner = std::io::stdin();
    let mut line = String::new();
    let mut board = Board::default();
    let mut tt = TranspositionTable::init();
    loop {
        line.clear();
        scanner.read_line(&mut line).unwrap();
        let string = line.trim();
        match string {
            "uci" => {
                println!("id name {}", ENGINENAME);
                println!("id author {}", AUTHOR);
                println!("uciok");
            }
            "isready" => println!("readyok"),
            "ucinewgame" => board = Board::default(),
            "quit" => std::process::exit(0),
            "print" => print_board(init_board(board.to_string())),
            a if a.starts_with("position") => {
                if prev_cmd.contains("moves")
                    && prev_cmd.len() > 0
                    && string.trim().starts_with(&prev_cmd.trim())
                {
                    let move_list = string[(prev_cmd.trim().len())..string.len()].trim();
                    for m in move_list.split(" ") {
                        board = board.make_move_new(ChessMove::from_str(m).unwrap());
                    }
                } else {
                    if string.starts_with("position fen") {
                        let fen = string[13..string.len()].to_owned();
                        board = Board::from_str(&fen).expect("Valid FEN");
                    }
                    if string.contains("startpos") {
                        board = Board::default();
                    }
                    if string.contains("moves") {
                        let i: usize = string.find("moves").unwrap();
                        let move_list = string[(i + 5)..string.len()].trim();
                        for m in move_list.split(" ") {
                            board = board.make_move_new(ChessMove::from_str(m).unwrap());
                        }
                    }
                }

                prev_cmd = line.clone();
            }
            a if a.starts_with("go") => {
                // if !moves_played.starts_with(tt.get_moves().as_str()) {
                //     tt = TranspositionTable::init();
                // }
                if a.contains("perft") {
                    let i: usize = string.find("perft").unwrap();
                    let depth = string[(i + 5)..string.len()].trim().parse::<usize>();
                    if depth.is_ok() {
                        let start = Instant::now();
                        let res = go_perft(&board, depth.unwrap());
                        let duration = start.elapsed();
                        println!("{} in {:?}", res, duration,);
                    } else {
                        println!("invalid depth")
                    }
                } else {
                    if a.contains("movetime") {
                        let i: usize = string.find("movetime ").unwrap();
                        let time = string[(i + 9)..string.len()].trim().parse::<u32>().unwrap();
                        let res = start_search(
                            &board,
                            7,
                            Duration::new((time / 1000) as u64, (time % 1000) * 1000000),
                            &mut tt,
                        );
                        println!("bestmove {}", res.best_move.to_string());
                    } else {
                        let res = start_search(&board, 7, Duration::new(5, 0), &mut tt);
                        println!("bestmove {}", res.best_move.to_string());
                    }
                    // if use_book
                    //     && !moves_played
                    //         .clone()
                    //         .starts_with(book.get_opening().as_str())
                    //     || count_moves(&moves_played) > 19
                    // {
                    //     use_book = false;
                    //     let res = start_search(&board, 7, Duration::new(7, 0), &mut tt);
                    //     println!("bestmove {}", res.best_move.to_string());
                    // } else if use_book {
                    //     let book_move = book.get_next_move(&moves_played);
                    //     if book_move != "" {
                    //         println!("bestmove {}", book.get_next_move(&moves_played));
                    //     } else {
                    //         let res = start_search(&board, 7, Duration::new(7, 0), &mut tt);
                    //         println!("bestmove {}", res.best_move.to_string());
                    //     }
                    // } else {
                    // }
                }
            }
            a if a.starts_with("search") => {
                let res = search_at_fixed_depth(&board, 4);
                println!(
                    "bestmove {}: time {:?}",
                    res.best_move.to_string(),
                    res.duration
                );
            }
            _ => {}
        }
    }
}
