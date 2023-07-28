use crate::{
    evaluate::evaluate,
    moves::{sort_captures, sort_moves},
    transposition_table::{EntryType, TranspositionTable},
};
use chess::{Board, BoardStatus, ChessMove, MoveGen, Piece, Rank};
use std::time::{Duration, Instant};
const SEARCH_EXIT_KEY: i16 = std::i16::MAX;
const NEG_SEARCH_EXIT_KEY: i16 = -SEARCH_EXIT_KEY;
const ALPHA: i16 = -i16::MAX;
const BETA: i16 = i16::MAX;

static mut TIME_LIMIT: Duration = Duration::new(0, 0);
pub struct SearchResult {
    pub eval: i16,
    pub best_move: ChessMove,
    pub depth: u8,
    pub duration: Duration,
}

fn quiesce(board: &Board, alpha: i16, beta: i16, ply_from_root: u8) -> i16 {
    let status = board.status();
    if status == BoardStatus::Checkmate {
        return -10000 + (ply_from_root as i16);
    } else if status == BoardStatus::Stalemate {
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
    let moves = sort_captures(&mut iterable, board);
    for mv in moves {
        let score = -quiesce(&board.make_move_new(mv), -beta, -alpha, ply_from_root + 1);
        if score >= beta {
            return beta;
        }
        if score > alpha {
            alpha = score;
        }
    }
    return alpha;
}
fn alpha_beta(
    board: &Board,
    ply_from_root: u8,
    depth: u8,
    extended: u8,
    alpha: i16,
    beta: i16,
    init: &Instant,
    tt: &mut TranspositionTable,
    age: u16,
) -> i16 {
    unsafe {
        if init.elapsed() >= TIME_LIMIT {
            return SEARCH_EXIT_KEY;
        }
    }
    let status = board.status();
    if status == BoardStatus::Checkmate {
        return -10000 + (ply_from_root as i16);
    } else if status == BoardStatus::Stalemate {
        return 0;
    }
    let key = board.get_hash();
    let tt_value = tt.look_up_pos(key);
    let tt_move = if tt_value.is_some() {
        tt_value.unwrap().best_move
    } else {
        ChessMove::default()
    };
    if tt_value.is_some() && (depth <= tt_value.unwrap().depth) {
        match tt_value.unwrap().entry_type {
            EntryType::Exact => return tt_value.unwrap().eval,
            EntryType::UpperBound => {
                if tt_value.unwrap().eval <= alpha {
                    return alpha;
                }
            }
            EntryType::LowerBound => {
                if tt_value.unwrap().eval >= beta {
                    return beta;
                }
            }
        }
    }
    if depth == 0 {
        return quiesce(board, alpha, beta, ply_from_root + 1);
    }
    let mut iterable = MoveGen::new_legal(&board);
    let moves = sort_moves(
        &mut iterable,
        board,
        tt_move,
        tt.get_killers(ply_from_root as usize),
    );
    let mut best_move = ChessMove::default();
    let mut alpha = alpha;
    let mut tt_type = EntryType::UpperBound;
    // let mut best_move = ChessMove::default();
    for i in 0..moves.len() {
        let mv = moves[i];
        let piece = board.piece_on(mv.get_source()).unwrap();
        let is_capture = board.piece_on(mv.get_dest()).is_some();
        let new_board = board.make_move_new(mv);
        let is_check = board.checkers().0 != 0;
        let mut extention = if is_check && extended < 6 { 1 } else { 0 };
        let rank = mv.get_dest().get_rank();
        if piece == Piece::Pawn && (rank == Rank::Second || rank == Rank::Seventh) {
            extention += 1;
        }
        let reduction: u8 = if extention == 0
            && ply_from_root > 2
            && !is_capture
            && i > 4
            && mv.get_promotion().is_none()
            && depth > 3
        {
            1
        } else {
            0
        };
        let mut score = 0;
        let mut needs_full_search = true;
        if reduction != 0 {
            score = -alpha_beta(
                &new_board,
                ply_from_root + 1,
                depth - 1 - reduction,
                extended,
                -beta,
                -alpha,
                init,
                tt,
                age,
            );
            needs_full_search = score > alpha;
        }
        if needs_full_search {
            score = -alpha_beta(
                &new_board,
                ply_from_root + 1,
                depth + extention - 1,
                extended + extention,
                -beta,
                -alpha,
                init,
                tt,
                age,
            );
        }
        if score == NEG_SEARCH_EXIT_KEY {
            return SEARCH_EXIT_KEY;
        }
        if score >= beta {
            tt.store_killer(ply_from_root as usize, mv);
            tt.set_pos(
                key,
                beta,
                EntryType::LowerBound,
                depth,
                ply_from_root,
                mv,
                age,
            );
            return beta;
        }
        if score > alpha {
            alpha = score;
            tt_type = EntryType::Exact;
            best_move = mv;
        }
    }
    tt.set_pos(key, alpha, tt_type, depth, ply_from_root, best_move, age);
    return alpha;
}
fn search(
    board: &Board,
    moves: &mut Vec<ChessMove>,
    max_depth: u8,
    init: &Instant,
    tt: &mut TranspositionTable,
    draws: &Vec<u64>,
    age: u16,
) -> SearchResult {
    let start = Instant::now();
    let mut best_move = moves[0];
    let mut alpha = ALPHA;
    let mut ext_moves = Vec::<(ChessMove, i16)>::with_capacity(moves.len());
    for i in 0..moves.len() {
        let mv = moves[i];
        let piece = board.piece_on(mv.get_source()).unwrap();
        let new_board = board.make_move_new(mv);
        let mut score = 0;
        if !draws.contains(&new_board.get_hash()) {
            let mut extention = if board.checkers().0 != 0 { 1 } else { 0 };
            let rank = mv.get_dest().get_rank();
            if piece == Piece::Pawn && (rank == Rank::Second || rank == Rank::Seventh) {
                extention += 1;
            }
            let reduction = if i > 4 && extention == 0 && max_depth > 2 {
                1
            } else {
                0
            };
            let mut needs_full_search = true;
            if reduction == 1 {
                score = -alpha_beta(
                    &new_board,
                    1,
                    max_depth - 1 - reduction,
                    extention,
                    -BETA,
                    -alpha,
                    init,
                    tt,
                    age,
                );
                needs_full_search = score > alpha;
            }
            if needs_full_search {
                score = -alpha_beta(
                    &new_board,
                    1,
                    max_depth - 1 + extention,
                    extention,
                    -BETA,
                    -alpha,
                    init,
                    tt,
                    age,
                );
            }
        }

        if score == (NEG_SEARCH_EXIT_KEY) {
            return SearchResult {
                eval: alpha,
                best_move,
                depth: max_depth,
                duration: start.elapsed(),
            };
        }
        ext_moves.push((mv, score));
        if score > alpha {
            alpha = score;
            best_move = mv;
        }
    }
    ext_moves.sort_by(|b, a| a.1.cmp(&b.1));
    moves.clear();
    for i in ext_moves {
        moves.push(i.0);
    }
    tt.set_pos(
        board.get_hash(),
        alpha,
        EntryType::Exact,
        max_depth,
        0,
        best_move,
        age,
    );
    return SearchResult {
        eval: alpha,
        best_move,
        depth: max_depth,
        duration: start.elapsed(),
    };
}
pub fn start_search(
    board: &Board,
    max_depth: u8,
    max_duration: Duration,
    tt: &mut TranspositionTable,
    draws: &Vec<u64>,
    age: u16,
) -> SearchResult {
    unsafe {
        TIME_LIMIT = max_duration;
    }
    let start = Instant::now();
    let mut iterable = MoveGen::new_legal(&board);
    let mut moves = sort_moves(
        &mut iterable,
        board,
        ChessMove::default(),
        &tt.default_killers,
    );
    let mut result = search(board, &mut moves, 1, &start, tt, draws, age);
    if moves.len() == 1 {
        return result;
    }
    for i in 2..=max_depth {
        let res = search(board, &mut moves, i, &start, tt, draws, age);
        result = res;
        // println!(
        //     "info depth {} bestmove {} ({})",
        //     i,
        //     result.best_move.to_string(),
        //     result.eval
        // );
        if start.elapsed() >= max_duration {
            break;
        }
    }
    result.duration = start.elapsed();
    return result;
}
pub fn search_at_fixed_depth(board: &Board, depth: u8) -> SearchResult {
    unsafe {
        TIME_LIMIT = Duration::MAX;
    }
    let mut iterable = MoveGen::new_legal(&board);
    let mut tt = TranspositionTable::init();
    let mut moves = sort_moves(
        &mut iterable,
        board,
        ChessMove::default(),
        &tt.default_killers,
    );
    return search(
        board,
        &mut moves,
        depth,
        &Instant::now(),
        &mut tt,
        &vec![],
        0,
    );
}
