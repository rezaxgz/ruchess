use chess::{Board, ChessMove, MoveGen, Piece};
use std::{
    str::FromStr,
    time::{Duration, Instant},
};
fn value(piece: Piece) -> u8 {
    match piece {
        Piece::Pawn => 1,
        Piece::Bishop => 3,
        Piece::Knight => 3,
        Piece::Rook => 5,
        Piece::Queen => 9,
        Piece::King => 0,
    }
}
fn move_value(m: &ChessMove, piece_at_start: Option<Piece>, piece_at_end: Option<Piece>) -> usize {
    return 0;
}
fn sort_moves(mut iterable: MoveGen, board: &Board) -> Vec<ChessMove> {
    let mut vector = Vec::<ChessMove>::with_capacity(iterable.len());
    for mv in &mut iterable {
        vector.push(mv);
    }
    vector.sort_by(|b, a| {
        move_value(
            a,
            board.piece_on(a.get_source()),
            board.piece_on(a.get_dest()),
        )
        .cmp(&move_value(
            b,
            board.piece_on(b.get_source()),
            board.piece_on(b.get_dest()),
        ))
    });
    return vector;
}
pub fn benchmark(n: usize) -> Duration {
    let start = Instant::now();
    let board = Board::default();
    for _ in 0..n {
        let iterable = MoveGen::new_legal(&board);
        let moves = sort_moves(iterable, &board);
    }
    let duration = start.elapsed();
    println!("{:?}", duration);
    return duration;
}
