use crate::evaluate::evaluate;
use chess::{Board, BoardStatus, ChessMove, MoveGen};
use std::{
    time::{Duration, Instant},
    vec,
};
const SEARCH_EXIT_KEY: i32 = std::i32::MIN;
const ALPHA: i32 = -100000;
const BETA: i32 = 100000;

static mut CURRENT_SEARCH_DEPTH: u8 = 0;
pub struct SearchResult {
    pub eval: i32,
    pub best_move: ChessMove,
    pub depth: u8,
    pub duration: Duration,
}
fn quiesce(board: &Board, alpha: i32, beta: i32) -> i32 {
    if board.status() == BoardStatus::Stalemate {
        return 0;
    }
    let mut alpha = alpha;
    let stand_pat = evaluate(board);
    if stand_pat >= beta {
        return beta;
    }
    if alpha < stand_pat {
        alpha = stand_pat;
    }
    let mut iterable = MoveGen::new_legal(&board);
    let targets = board.color_combined(!board.side_to_move());
    iterable.set_iterator_mask(*targets);
    for mv in &mut iterable {
        let score = -quiesce(&board.make_move_new(mv), -beta, -alpha);
        if score >= beta {
            return beta;
        }
        if score > alpha {
            alpha = score;
        }
    }
    return alpha;
}
fn sort_moves(mut iterable: MoveGen) -> Vec<ChessMove> {
    let mut vector = Vec::<ChessMove>::with_capacity(iterable.len());
    for mv in &mut iterable {
        vector.push(mv);
    }
    return vector;
}
pub fn alpha_beta(board: &Board, depth: u8, alpha: i32, beta: i32) -> i32 {
    if depth == 0 {
        return quiesce(board, alpha, beta);
    }
    if board.status() != BoardStatus::Ongoing {
        if board.status() == BoardStatus::Checkmate {
            unsafe {
                return -100000 + ((CURRENT_SEARCH_DEPTH - depth) as i32);
            }
        }
    }
    let mut iterable = MoveGen::new_legal(&board);
    let mut alpha = alpha;
    let targets = board.color_combined(!board.side_to_move());
    iterable.set_iterator_mask(*targets);
    // let mut best_move = ChessMove::default();
    for mv in &mut iterable {
        let new_board = board.make_move_new(mv);
        let score = -alpha_beta(&new_board, depth - 1, -beta, -alpha);
        if score >= beta {
            //tt set
            return beta;
        }
        if score > alpha {
            alpha = score;
            // best_move = mv;
        }
    }
    iterable.set_iterator_mask(!board.combined());
    for mv in &mut iterable {
        let new_board = board.make_move_new(mv);
        let score = -alpha_beta(&new_board, depth - 1, -beta, -alpha);
        if score >= beta {
            //tt set
            return beta;
        }
        if score > alpha {
            alpha = score;
            // best_move = mv;
        }
    }
    return alpha;
}
pub fn search(board: &Board, depth: u8) -> SearchResult {
    unsafe {
        CURRENT_SEARCH_DEPTH = depth;
    }
    let start = Instant::now();
    let mut best_move = ChessMove::default();
    let mut iterable = MoveGen::new_legal(&board);
    let mut alpha = ALPHA;
    let targets = board.color_combined(!board.side_to_move());
    iterable.set_iterator_mask(*targets);
    // let mut best_move = ChessMove::default();
    for mv in &mut iterable {
        let new_board = board.make_move_new(mv);
        let score = -alpha_beta(&new_board, depth - 1, -BETA, -alpha);
        if score > alpha {
            alpha = score;
            best_move = mv;
        }
    }
    iterable.set_iterator_mask(!board.combined());
    for mv in &mut iterable {
        let new_board = board.make_move_new(mv);
        let score = -alpha_beta(&new_board, depth - 1, -BETA, -alpha);
        if score > alpha {
            alpha = score;
            best_move = mv;
        }
    }
    return SearchResult {
        eval: 0,
        best_move,
        depth,
        duration: start.elapsed(),
    };
}
pub fn start_search(board: &Board, max_depth: u8) -> SearchResult {
    let mut result = search(board, 1);
    for i in 2..max_depth {
        let res = search(board, i);
        if res.eval != SEARCH_EXIT_KEY && res.eval != -SEARCH_EXIT_KEY {
            result = res;
        };
    }
    return result;
}
