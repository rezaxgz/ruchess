const NOT_FILE_A_BB: u64 = !72340172838076673;
const NOT_FILE_H_BB: u64 = !(72340172838076673 << 7);
const MVV_LVA: [[i8; 6]; 5] = [
    [15, 14, 13, 12, 11, 10], // victim P, attacker K, Q, R, B, N, P, None
    [25, 24, 23, 22, 21, 20], // victim N, attacker K, Q, R, B, N, P, None
    [35, 34, 33, 32, 31, 30], // victim B, attacker K, Q, R, B, N, P, None
    [45, 44, 43, 42, 41, 40], // victim R, attacker K, Q, R, B, N, P, None
    [55, 54, 53, 52, 51, 50], // victim Q, attacker K, Q, R, B, N, P, None
];
use chess::{Board, ChessMove, Color, MoveGen, Piece};

use crate::data::get_spst_value;
fn piece_value(piece: Piece) -> i8 {
    match piece {
        Piece::Pawn => 1,
        Piece::Bishop => 3,
        Piece::Knight => 3,
        Piece::Rook => 5,
        Piece::Queen => 9,
        Piece::King => 0,
    }
}
fn promotion_value(piece: Piece) -> i8 {
    match piece {
        Piece::Pawn => 0,
        Piece::King => 0,
        Piece::Bishop => 8,
        Piece::Knight => 8,
        Piece::Rook => 16,
        Piece::Queen => 32,
    }
}
fn move_value(
    m: &ChessMove,
    piece_at_start: Piece,
    piece_at_end: Option<Piece>,
    is_controled: bool,
    color: Color,
) -> i8 {
    let mut value: i8 = get_spst_value(color, piece_at_start, m.get_dest())
        - get_spst_value(color, piece_at_start, m.get_source());
    if piece_at_end.is_some() {
        let capture_value = MVV_LVA[piece_at_end.unwrap().to_index()][piece_at_start.to_index()];
        value += capture_value;
    }
    if is_controled && piece_at_start != Piece::Pawn {
        value -= 35;
    } else if m.get_promotion().is_some() {
        value += promotion_value(m.get_promotion().unwrap());
    }
    return value;
}
pub fn sort_moves(iterable: &mut MoveGen, board: &Board) -> Vec<ChessMove> {
    let pawns = board.pieces(Piece::Pawn);
    let controled = if board.side_to_move() == Color::White {
        ((pawns & board.color_combined(Color::Black)).0 >> 9 & NOT_FILE_H_BB)
            | ((pawns & board.color_combined(Color::Black)).0 >> 7 & NOT_FILE_A_BB)
    } else {
        ((pawns & board.color_combined(Color::White)).0 << 7 & NOT_FILE_H_BB)
            | ((pawns & board.color_combined(Color::White)).0 << 9 & NOT_FILE_A_BB)
    };
    let mut vector = Vec::<(ChessMove, i8)>::with_capacity(iterable.len());
    for mv in iterable {
        vector.push((
            mv,
            move_value(
                &mv,
                board.piece_on(mv.get_source()).unwrap(),
                board.piece_on(mv.get_dest()),
                (controled & (1 << mv.get_dest().to_index())) != 0,
                board.side_to_move(),
            ),
        ));
    }
    vector.sort_by(|b, a| a.1.cmp(&b.1));
    return vector.iter().map(|t| t.0).collect();
}
fn capture_value(piece: Piece, captured: Piece, promo: Option<Piece>, is_controled: bool) -> i8 {
    let mut value = (piece_value(captured) - piece_value(piece)) << 3;
    if is_controled && piece != Piece::Pawn {
        value -= 64;
    } else if promo.is_some() {
        value += promotion_value(promo.unwrap());
    }
    return value;
}
pub fn sort_captures(iterable: &mut MoveGen, board: &Board) -> Vec<ChessMove> {
    let mut vector = Vec::<ChessMove>::with_capacity(iterable.len());
    for mv in iterable {
        vector.push(mv);
    }
    let pawns = board.pieces(Piece::Pawn);
    let controled = if board.side_to_move() == Color::White {
        ((pawns & board.color_combined(Color::Black)).0 >> 9 & NOT_FILE_H_BB)
            | ((pawns & board.color_combined(Color::Black)).0 >> 7 & NOT_FILE_A_BB)
    } else {
        ((pawns & board.color_combined(Color::White)).0 << 7 & NOT_FILE_H_BB)
            | ((pawns & board.color_combined(Color::White)).0 << 9 & NOT_FILE_A_BB)
    };
    vector.sort_by(|b, a| {
        capture_value(
            board.piece_on(a.get_source()).unwrap(),
            board.piece_on(a.get_dest()).unwrap(),
            a.get_promotion(),
            (controled & (1 << a.get_dest().to_index())) != 0,
        )
        .cmp(&capture_value(
            board.piece_on(b.get_source()).unwrap(),
            board.piece_on(b.get_dest()).unwrap(),
            b.get_promotion(),
            (controled & (1 << b.get_dest().to_index())) != 0,
        ))
    });
    return vector;
}
