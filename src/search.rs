use crate::{
    evaluate::evaluate,
    moves::{sort_captures, sort_moves},
    transposition_table::{EntryType, TranspositionTable},
};
use chess::{Board, BoardStatus, ChessMove, MoveGen, Piece, Rank};
use std::time::{Duration, Instant};
const SEARCH_EXIT_KEY: i16 = std::i16::MAX;
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
) -> i16 {
    let key = board.get_hash();
    let tt_value = tt.look_up_pos(key);
    if tt_value.is_some() && ply_from_root < 3 && (depth <= tt_value.unwrap().depth) {
        return tt_value.unwrap().eval;
    }
    if depth == 0 {
        return quiesce(board, alpha, beta, ply_from_root + 1);
    }
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
    let mut iterable = MoveGen::new_legal(&board);
    let moves = sort_moves(&mut iterable, board);
    let mut best_move = ChessMove::default();
    let mut alpha = alpha;
    let mut tt_type = EntryType::UpperBound;
    // let mut best_move = ChessMove::default();
    for i in 0..moves.len() {
        let mv = moves.get(i).unwrap();
        let piece = board.piece_on(mv.get_source()).unwrap();
        let new_board = board.make_move_new(*mv);
        let is_check = board.checkers().0 != 0;
        let mut extention = if is_check && extended < 6 { 1 } else { 0 };
        let rank = mv.get_dest().get_rank();
        if piece == Piece::Pawn && (rank == Rank::Second || rank == Rank::Seventh) {
            extention += 1;
        }
        let score = -alpha_beta(
            &new_board,
            ply_from_root + 1,
            depth + extention - 1,
            extended + extention,
            -beta,
            -alpha,
            init,
            tt,
        );
        if score >= beta {
            tt.set_pos(key, beta, EntryType::UpperBound, depth, *mv);
            return beta;
        }
        if score > alpha {
            alpha = score;
            tt_type = EntryType::Exact;
            best_move = *mv;
        }
    }
    tt.set_pos(key, alpha, tt_type, depth, best_move);
    return alpha;
}
fn search(
    board: &Board,
    moves: &mut Vec<ChessMove>,
    max_depth: u8,
    init: &Instant,
    tt: &mut TranspositionTable,
) -> SearchResult {
    let start = Instant::now();
    let mut best_move = *moves.get(0).unwrap();
    let mut alpha = ALPHA;
    let mut ext_moves = Vec::<(ChessMove, i16)>::with_capacity(moves.len());
    for i in 0..moves.len() {
        let mv = *moves.get(i).unwrap();
        let new_board = board.make_move_new(mv);
        let score = -alpha_beta(&new_board, 1, max_depth - 1, 0, -BETA, -alpha, init, tt);
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
) -> SearchResult {
    unsafe {
        TIME_LIMIT = max_duration;
    }
    let start = Instant::now();
    let mut iterable = MoveGen::new_legal(&board);
    let mut moves = sort_moves(&mut iterable, board);
    let mut result = search(board, &mut moves, 1, &start, tt);
    for i in 2..=max_depth {
        let res = search(board, &mut moves, i, &start, tt);
        if res.eval != SEARCH_EXIT_KEY && res.eval != -SEARCH_EXIT_KEY {
            result = res;
        } else {
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
    let mut moves = sort_moves(&mut iterable, board);
    return search(
        board,
        &mut moves,
        depth,
        &Instant::now(),
        &mut TranspositionTable::init(),
    );
}
