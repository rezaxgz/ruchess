use crate::board::Position;
use crate::evaluate::PAWN_TT_HITS;
use crate::{
    evaluate::evaluate,
    moves::{sort_captures, sort_moves},
    transposition_table::{EntryType, TranspositionTable},
};
use chess::{BoardStatus, ChessMove, MoveGen, Piece, Rank};
use std::time::{Duration, Instant};
const SEARCH_EXIT_KEY: i16 = std::i16::MAX;
const NEG_SEARCH_EXIT_KEY: i16 = -SEARCH_EXIT_KEY;
const ALPHA: i16 = -i16::MAX;
const BETA: i16 = i16::MAX;
static mut TIME_LIMIT: Duration = Duration::new(0, 0);
static mut NODES: u32 = 0;
static mut TT_HITS: u32 = 0;
static mut BETA_CUTOFFS: u32 = 0;
pub struct SearchResult {
    pub eval: i16,
    pub best_move: ChessMove,
    pub depth: u8,
    pub duration: Duration,
}

fn quiesce(board: &Position, alpha: i16, beta: i16, tt: &mut TranspositionTable) -> i16 {
    let mut alpha = alpha;
    unsafe {
        NODES += 1;
    }
    let stand_pat = evaluate(board, tt);
    if stand_pat >= beta {
        unsafe {
            BETA_CUTOFFS += 1;
        }
        return beta;
    }
    if alpha < stand_pat {
        alpha = stand_pat;
    }
    let mut iterable = MoveGen::new_legal(&board.board);
    let targets = board.color_combined(!board.side_to_move());
    iterable.set_iterator_mask(*targets);
    let moves = sort_captures(&mut iterable, &board.board);
    for mv in moves {
        let score = -quiesce(&board.make_move_new(mv), -beta, -alpha, tt);
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
    board: &Position,
    ply_from_root: u8,
    depth: u8,
    extended: u8,
    alpha: i16,
    beta: i16,
    init: &Instant,
    tt: &mut TranspositionTable,
) -> i16 {
    unsafe {
        if init.elapsed() >= TIME_LIMIT {
            return SEARCH_EXIT_KEY;
        }
        NODES += 1;
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
        unsafe {
            TT_HITS += 1;
        }
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
            EntryType::None => {}
        }
    }
    if depth == 0 {
        return quiesce(board, alpha, beta, tt);
    }
    let mut iterable = MoveGen::new_legal(&board.board);
    let moves = sort_moves(
        &mut iterable,
        &board.board,
        tt_move,
        tt.get_killers(ply_from_root as usize),
    );
    let mut best_move = moves[0].0;
    let mut alpha = alpha;
    let mut tt_type = EntryType::UpperBound;
    // let mut best_move = ChessMove::default();
    for i in 0..moves.len() {
        let mv = moves[i].0;
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
            );
        }
        if score == NEG_SEARCH_EXIT_KEY {
            return SEARCH_EXIT_KEY;
        }
        if score >= beta {
            unsafe {
                BETA_CUTOFFS += 1;
            }
            tt.store_killer(ply_from_root as usize, mv);
            tt.set_pos(key, score, EntryType::LowerBound, depth, mv);
            return beta;
        }
        if score > alpha {
            alpha = score;
            tt_type = EntryType::Exact;
            best_move = mv;
        }
    }
    tt.set_pos(key, alpha, tt_type, depth, best_move);
    return alpha;
}
fn search(
    board: &Position,
    moves: &mut Vec<(ChessMove, i16)>,
    mut alpha: i16,
    _beta: i16,
    max_depth: u8,
    init: &Instant,
    tt: &mut TranspositionTable,
    draws: &Vec<u64>,
) -> SearchResult {
    let start = Instant::now();
    let mut best_move = moves[0].0;
    for i in 0..moves.len() {
        let (mv, _prev) = moves[i];
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
        moves[i] = (mv, score);
        if score > alpha {
            alpha = score;
            best_move = mv;
        }
    }
    moves.sort_by(|b, a| a.1.cmp(&b.1));
    tt.set_pos(
        board.get_hash(),
        alpha,
        EntryType::Exact,
        max_depth,
        best_move,
    );
    return SearchResult {
        eval: alpha,
        best_move,
        depth: max_depth,
        duration: start.elapsed(),
    };
}
pub fn start_search(
    board: &Position,
    max_depth: u8,
    max_duration: Duration,
    tt: &mut TranspositionTable,
    draws: &Vec<u64>,
    log: bool,
) -> SearchResult {
    unsafe {
        TIME_LIMIT = max_duration;
    }
    let start = Instant::now();
    let mut iterable = MoveGen::new_legal(&board.board);
    let mut moves = sort_moves(
        &mut iterable,
        &board.board,
        ChessMove::default(),
        &tt.default_killers,
    );
    let alpha = ALPHA;
    let beta = BETA;
    let mut result = search(board, &mut moves, alpha, beta, 1, &start, tt, draws);
    if moves.len() == 1 {
        return result;
    }
    for i in 2..=max_depth {
        unsafe {
            TT_HITS = 0;
            NODES = 0;
            PAWN_TT_HITS = 0;
            BETA_CUTOFFS = 0;
        }
        let res = search(board, &mut moves, alpha, beta, i, &start, tt, draws);
        let old_alpha = result.eval;
        result = res;
        if start.elapsed() >= max_duration {
            if result.eval == ALPHA {
                result.eval = old_alpha;
            }
            if log {
                unsafe {
                    println!(
                        "info depth {} bestmove {} ({}) tt_hits: {:?} pawn_tt_hits: {:?} cut_offs: {} nodes {:?}, {:?}",
                        i,
                        result.best_move.to_string(),
                        result.eval,
                        TT_HITS,
                        PAWN_TT_HITS,
                        BETA_CUTOFFS,
                        NODES,
                        start.elapsed()
                    );
                }
            }
            break;
        }
        if log {
            unsafe {
                println!(
                    "info depth {} bestmove {} ({}) tt_hits: {} pawn_tt_hits: {} cut_offs: {} nodes {}, {:?}",
                    i,
                    result.best_move.to_string(),
                    result.eval,
                    TT_HITS,
                    PAWN_TT_HITS,
                    BETA_CUTOFFS,
                    NODES,
                    start.elapsed()
                );
            }
        }
    }
    result.duration = start.elapsed();
    return result;
}
