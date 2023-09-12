const NOT_FILE_A_BB: u64 = !72340172838076673;
const NOT_FILE_H_BB: u64 = !(72340172838076673 << 7);
const MVV_LVA: [[i32; 6]; 5] = [
    [15, 14, 13, 12, 11, 10], // victim P, attacker P, N, B, R, Q, K
    [25, 24, 23, 22, 21, 20], // victim N, attacker P, N, B, R, Q, K
    [35, 34, 33, 32, 31, 30], // victim B, attacker P, N, B, R, Q, K
    [45, 44, 43, 42, 41, 40], // victim R, attacker P, N, B, R, Q, K
    [55, 54, 53, 52, 51, 50], // victim Q, attacker P, N, B, R, Q, K
];
const ROOK: usize = 0;
const BISHOP: usize = 1;
use crate::data::KNIGHT_MOVES;
use crate::magics::{MAGIC_NUMBERS, MOVES, RAYS};
use crate::{data::get_spst_value, transposition_table::Killers};
use chess::{Board, ChessMove, Color, MoveGen, Piece};
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub enum MoveType {
    BadCapture, //captures with a piece of higher value that can be recaptured by a pawn
    QuietMove,
    CounterMove,
    KillerMove,
    GoodCapture,
    Promotion,
    HashMove,
}
fn piece_value(piece: Piece) -> i8 {
    match piece {
        Piece::Pawn => 1,
        Piece::King => 0,
        Piece::Bishop => 3,
        Piece::Knight => 3,
        Piece::Rook => 5,
        Piece::Queen => 9,
    }
}
fn promotion_value(piece: Piece) -> i32 {
    match piece {
        Piece::Pawn => 0,
        Piece::King => 0,
        Piece::Bishop => 8,
        Piece::Knight => 8,
        Piece::Rook => 16,
        Piece::Queen => 32,
    }
}
fn move_type(
    piece_at_start: Piece,
    piece_at_end: Option<Piece>,
    is_controled: bool,
    is_tt_move: bool,
    is_killer: bool,
    is_counter: bool,
    is_promo: bool,
) -> MoveType {
    if is_tt_move {
        return MoveType::HashMove;
    }
    if is_promo {
        return MoveType::Promotion;
    }
    if piece_at_end.is_some() {
        if is_controled && piece_value(piece_at_start) > piece_value(piece_at_end.unwrap()) {
            return MoveType::BadCapture;
        }
        return MoveType::GoodCapture;
    }
    if is_killer {
        return MoveType::KillerMove;
    }
    if is_counter {
        return MoveType::CounterMove;
    }
    return MoveType::QuietMove;
}
fn move_value(
    m: &ChessMove,
    piece_at_start: Piece,
    piece_at_end: Option<Piece>,
    is_controled: bool,
    color: Color,
    is_tt_move: bool,
    is_killer: bool,
    is_counter: bool,
    history_value: i32,
) -> (ChessMove, i32, MoveType) {
    let mt = move_type(
        piece_at_start,
        piece_at_end,
        is_controled,
        is_tt_move,
        is_killer,
        is_counter,
        m.get_promotion().is_some(),
    );
    let mut value = 0;
    if piece_at_end.is_some() {
        //captures sorted with MVV_LVA
        value += MVV_LVA[piece_at_end.unwrap().to_index()][piece_at_start.to_index()];
    } else {
        //quiets sorter with history heuristic
        if is_controled && piece_at_start != Piece::Pawn {
            value -= 1000000;
        }
        value += history_value;
    }

    if m.get_promotion().is_some() {
        value += promotion_value(m.get_promotion().unwrap());
    } else {
        value += (get_spst_value(color, piece_at_start, m.get_dest())
            - get_spst_value(color, piece_at_start, m.get_source())) as i32;
    }
    return (*m, value, mt);
}
pub fn sort_moves(
    iterable: &mut MoveGen,
    board: &Board,
    tt_move: ChessMove,
    killer_moves: &Killers,
    history: &[[i32; 64]; 6],
    counter_move: ChessMove,
) -> Vec<(ChessMove, i32, MoveType)> {
    let pawns = board.pieces(Piece::Pawn);
    let controled = if board.side_to_move() == Color::White {
        ((pawns & board.color_combined(Color::Black)).0 >> 9 & NOT_FILE_H_BB)
            | ((pawns & board.color_combined(Color::Black)).0 >> 7 & NOT_FILE_A_BB)
    } else {
        ((pawns & board.color_combined(Color::White)).0 << 7 & NOT_FILE_H_BB)
            | ((pawns & board.color_combined(Color::White)).0 << 9 & NOT_FILE_A_BB)
    };
    let mut vector = Vec::<(ChessMove, i32, MoveType)>::with_capacity(iterable.len());
    for mv in iterable {
        let p = board.piece_on(mv.get_source()).unwrap();
        vector.push(move_value(
            &mv,
            p,
            board.piece_on(mv.get_dest()),
            (controled & (1 << mv.get_dest().to_index())) != 0,
            board.side_to_move(),
            mv == tt_move,
            killer_moves.contains(&mv),
            mv == counter_move,
            history[p.to_index()][mv.get_dest().to_index()],
        ));
    }
    vector.sort_by(|b, a| {
        if a.2 == b.2 {
            a.1.cmp(&b.1)
        } else {
            a.2.cmp(&b.2)
        }
    });
    return vector;
}
fn capture_value(piece: Piece, captured: Piece, promo: Option<Piece>, is_controled: bool) -> i16 {
    let mut value = MVV_LVA[captured.to_index()][piece.to_index()] as i16;
    if is_controled && piece != Piece::Pawn && piece_value(piece) > piece_value(captured) {
        value -= 50;
    } else if promo.is_some() {
        value += promotion_value(promo.unwrap()) as i16;
    }
    return value;
}
pub fn sort_captures(iterable: &mut MoveGen, board: &Board) -> Vec<ChessMove> {
    let pawns = board.pieces(Piece::Pawn);
    let controled = if board.side_to_move() == Color::White {
        ((pawns & board.color_combined(Color::Black)).0 >> 9 & NOT_FILE_H_BB)
            | ((pawns & board.color_combined(Color::Black)).0 >> 7 & NOT_FILE_A_BB)
    } else {
        ((pawns & board.color_combined(Color::White)).0 << 7 & NOT_FILE_H_BB)
            | ((pawns & board.color_combined(Color::White)).0 << 9 & NOT_FILE_A_BB)
    };
    let mut vector = Vec::<(ChessMove, i16)>::with_capacity(iterable.len());
    for mv in iterable {
        vector.push((
            mv,
            capture_value(
                board.piece_on(mv.get_source()).unwrap(),
                board.piece_on(mv.get_dest()).unwrap(),
                mv.get_promotion(),
                (controled & (1 << mv.get_dest().to_index())) != 0,
            ),
        ));
    }
    vector.sort_by(|b, a| a.1.cmp(&b.1));
    return vector.iter().map(|t| t.0).collect();
}
pub fn get_rook_moves(sq: usize, blockers: u64) -> u64 {
    let magic = MAGIC_NUMBERS[ROOK][sq];
    return MOVES[(magic.offset as usize)
        + ((magic.magic_number * (blockers & magic.mask)) >> magic.rightshift) as usize]
        & RAYS[ROOK][sq];
}
pub fn get_bishop_moves(sq: usize, blockers: u64) -> u64 {
    let magic = MAGIC_NUMBERS[BISHOP][sq];
    return MOVES[(magic.offset as usize)
        + ((magic.magic_number * (blockers & magic.mask)) >> magic.rightshift) as usize]
        & RAYS[BISHOP][sq];
}
pub fn get_knight_moves(sq: usize) -> u64 {
    return KNIGHT_MOVES[sq];
}
