use crate::{
    board_util::print_board,
    book::init_book_full,
    perft::go_perft,
    search::{search_at_fixed_depth, start_search},
    transposition_table::TranspositionTable,
};
use chess::{Board, ChessMove, Color};
use std::time::Instant;
use std::{str::FromStr, time::Duration};
const AUTHOR: &str = "Reza";
const ENGINENAME: &str = "ruchess";
fn allocate_time(my_time: u32) -> Duration {
    let time = (my_time / 30).min(15000);
    let dur = Duration::new(time as u64 / 1000, (time % 1000) * 1000000);
    return dur;
}
fn add_repetition(table: &mut Vec<(u64, u8)>, hash: u64) {
    for i in 0..table.len() {
        if table[i].0 == hash {
            table[i] = (hash, table[i].1 + 1);
            return;
        }
    }
    table.push((hash, 1));
}
fn get_possible_drawns(table: &Vec<(u64, u8)>) -> Vec<u64> {
    return table
        .into_iter()
        .filter(|a| a.1 == 2)
        .map(|i| i.0)
        .collect();
}
pub fn uci() {
    let mut prev_cmd = String::new();
    let scanner = std::io::stdin();
    let mut line = String::new();
    let mut board = Board::default();
    let mut tt = TranspositionTable::init();
    let mut book = init_book_full();
    let mut repetition_table: Vec<(u64, u8)> = Vec::new();
    let mut use_book = true;
    let mut book_move = String::from("");
    loop {
        line.clear();
        scanner.read_line(&mut line).unwrap();
        let string = line.trim();
        let args: Vec<&str> = string.split(" ").collect();
        match string {
            "uci" => {
                println!("id name {}", ENGINENAME);
                println!("id author {}", AUTHOR);
                println!("uciok");
            }
            "isready" => println!("readyok"),
            "ucinewgame" => {
                board = Board::default();
                tt.clear();
                book.reset();
                repetition_table.clear();
            }
            "quit" => std::process::exit(0),
            "print" => print_board(&board),

            a if a.starts_with("position") => {
                if prev_cmd.contains("moves")
                    && prev_cmd.len() > 0
                    && string.starts_with(&prev_cmd.trim())
                {
                    let move_list = string[(prev_cmd.trim().len())..string.len()].trim();
                    let i: usize = string.find("moves").unwrap();
                    let all_move_list = string[(i + 5)..string.len()].trim();
                    let book_res = book.check(all_move_list);
                    match book_res {
                        Some(x) => {
                            book_move = x;
                        }
                        None => use_book = false,
                    }
                    for m in move_list.split(" ") {
                        board = board.make_move_new(ChessMove::from_str(m).unwrap());
                        add_repetition(&mut repetition_table, board.get_hash());
                    }
                } else {
                    if string.starts_with("position fen") {
                        use_book = false;
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
                            add_repetition(&mut repetition_table, board.get_hash());
                        }
                        let book_res = book.check(move_list);
                        match book_res {
                            Some(x) => {
                                book_move = x;
                            }
                            None => use_book = false,
                        }
                    }
                    if string == "position startpos" {
                        let book_res = book.check("");
                        match book_res {
                            Some(x) => {
                                book_move = x;
                            }
                            None => use_book = false,
                        }
                    }
                }

                prev_cmd = String::from(string);
            }
            a if a.starts_with("go") => {
                if a.contains("perft") {
                    let i = args.iter().position(|r| *r == "perft").unwrap() + 1;
                    let depth = args[i].parse::<usize>();
                    if depth.is_ok() {
                        let start = Instant::now();
                        let res = go_perft(&board, depth.unwrap());
                        let duration = start.elapsed();
                        println!("{} in {:?}", res, duration,);
                    } else {
                        println!("invalid depth")
                    }
                } else {
                    if use_book && book_move != "".to_string() {
                        println!("bestmove {}", book_move);
                        book_move = String::from("");
                        continue;
                    }
                    let allocated_time: Duration;
                    if a.contains("movetime") {
                        let i = args.iter().position(|r| *r == "movetime").unwrap() + 1;
                        let time = args[i].parse::<u32>().unwrap();
                        allocated_time = Duration::new(time as u64 / 1000, (time % 1000) * 1000000);
                    } else if a.contains("wtime") && a.contains("btime") {
                        let wtime = args[args.iter().position(|r| *r == "wtime").unwrap() + 1]
                            .parse::<u32>()
                            .unwrap();
                        let btime = args[args.iter().position(|r| *r == "btime").unwrap() + 1]
                            .parse::<u32>()
                            .unwrap();
                        if board.side_to_move() == Color::White {
                            allocated_time = allocate_time(wtime);
                        } else {
                            allocated_time = allocate_time(btime);
                        }
                    } else {
                        allocated_time = Duration::new(3, 0);
                    }
                    let res = start_search(
                        &board,
                        20,
                        allocated_time,
                        &mut tt,
                        &get_possible_drawns(&repetition_table),
                    );
                    println!("bestmove {}", res.best_move.to_string());
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
