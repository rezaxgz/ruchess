use crate::board::Position;
use crate::data::{
    calc_king_pst, get_adjacent_files, get_distance_from_center, get_fileset_bb, get_front_spans,
    get_orthogonal_distance, DARK_SQUARES, FILES, LIGHT_SQUARES, PAWN_SQUARE_TABLES,
    PIECE_SQUARE_TABLES,
};
use crate::transposition_table::TranspositionTable;
use chess::BitBoard;
use chess::{
    Color::Black, Color::White, Piece::Bishop, Piece::Knight, Piece::Pawn, Piece::Queen,
    Piece::Rook,
};
pub static mut PAWN_TT_HITS: u32 = 0;
const PAWN_VALUE: u32 = 100;
const KNIGHT_VALUE: u32 = 310;
const BISHOP_VALUE: u32 = 320;
const ROOK_VALUE: u32 = 500;
const QUEEN_VALUE: u32 = 975;
const ENDGAME_MATERIAL_START: f32 = (ROOK_VALUE * 2 + BISHOP_VALUE + KNIGHT_VALUE) as f32;
const MULTIPLIER: f32 = 1.0 / ENDGAME_MATERIAL_START as f32;
const PASSED_PAWN_VALUES: [i16; 7] = [0, 90, 60, 40, 25, 15, 15];
const BISHOP_PAIR_VALUE: i16 = 50;

fn get_endgame_weight(material: f32) -> f32 {
    if material < ENDGAME_MATERIAL_START {
        return 1.0 - (material * MULTIPLIER);
    } else {
        return 0.0;
    };
}
fn evaluate_bishop_pair(bishops: u64) -> i16 {
    if ((bishops & LIGHT_SQUARES) != 0) && ((bishops & DARK_SQUARES) != 0) {
        return BISHOP_PAIR_VALUE;
    }
    return 0;
}
fn get_material(board: &Position, color: &BitBoard) -> (f32, i16) {
    let material = (board.pieces(Knight) & color).popcnt() * KNIGHT_VALUE
        + (board.pieces(Bishop) & color).popcnt() * BISHOP_VALUE
        + (board.pieces(Rook) & color).popcnt() * ROOK_VALUE
        + (board.pieces(Queen) & color).popcnt() * QUEEN_VALUE;
    return (
        material as f32,
        (material + ((board.pieces(Pawn) & color).popcnt() * PAWN_VALUE)) as i16,
    );
}
pub fn evaluate(board: &Position, tt: &mut TranspositionTable) -> i16 {
    let white_combined = board.color_combined(White);
    let black_combined = board.color_combined(Black);

    let wp = board.pieces(Pawn) & white_combined;
    let bp = board.pieces(Pawn) & black_combined;
    let wr = board.pieces(Rook) & white_combined;
    let br = board.pieces(Rook) & black_combined;

    let (white_material_without_pawns, white_material) = get_material(board, white_combined);
    let (black_material_without_pawns, black_material) = get_material(board, black_combined);

    let white_endgame = get_endgame_weight(white_material_without_pawns);
    let black_endgame = get_endgame_weight(black_material_without_pawns);

    let white_middlegame = 1.0 - white_endgame;
    let black_middlegame = 1.0 - black_endgame;

    let piece_scores = board.get_pst_values()
        + calc_king_pst(
            0,
            board.king_square(White).to_index(),
            black_endgame,
            black_middlegame,
        )
        + calc_king_pst(
            1,
            board.king_square(Black).to_index(),
            white_endgame,
            white_middlegame,
        );

    let mop_eval: i16 = mop_up_eval(
        board.king_square(White).to_index(),
        board.king_square(Black).to_index(),
        white_material_without_pawns,
        black_material_without_pawns,
        black_endgame,
    ) - mop_up_eval(
        board.king_square(Black).to_index(),
        board.king_square(White).to_index(),
        black_material_without_pawns,
        white_material_without_pawns,
        white_endgame,
    );

    let (pawn_eval, wp_fileset, bp_fileset) = evaluate_pawns(
        tt,
        board.get_pawn_hash(),
        (white_endgame, black_endgame),
        (white_middlegame, black_middlegame),
        wp.0,
        bp.0,
    );
    let closed = wp_fileset & bp_fileset;
    let open = (!wp_fileset) & (!bp_fileset);
    let semi_open_white = bp_fileset & (!wp_fileset);
    let semi_open_black = wp_fileset & (bp_fileset);

    let rooks_eval = evaluate_rooks(wr.0, br.0, open, semi_open_white, semi_open_black, closed);

    let bishop_eval = evaluate_bishop_pair((board.pieces(Bishop) & white_combined).0)
        - evaluate_bishop_pair((board.pieces(Bishop) & black_combined).0);

    let eval = white_material - black_material
        + mop_eval
        + piece_scores
        + pawn_eval
        + bishop_eval
        + rooks_eval;
    // + king_eval;
    if board.side_to_move() == White {
        return eval;
    }
    return -eval;
}
fn mop_up_eval(
    my_king: usize,
    their_king: usize,
    my_material: f32,
    their_material: f32,
    endgame: f32,
) -> i16 {
    let mut score: i16 = 0;
    if my_material > (their_material + 200.0) && endgame > 0.0 {
        score += get_distance_from_center(their_king) as i16 * 10;
        score += (14 - get_orthogonal_distance(my_king, their_king) as i16) * 4
    }
    return score;
}
fn get_pawn_data(pawns: u64, enemy_pawns: u64, color: usize) -> (i16, u8, i16, i16) {
    let mut score = 0;
    let mut p = pawns;
    let mut fileset: u8 = 0;
    let mut middle_game = 0;
    let mut endgame = 0;
    while p != 0 {
        let i = p.trailing_zeros() as usize;
        p &= p - 1;
        let file = i & 7;
        endgame += PAWN_SQUARE_TABLES[color][i];
        middle_game += PIECE_SQUARE_TABLES[color][0][i];
        let front_span = get_front_spans(color, i) & enemy_pawns;
        let is_open = front_span & FILES[file] == 0;
        if ((fileset >> file) & 1) == 1 {
            //doubled pawn
            score -= if is_open { 20 } else { 10 };
        } else {
            fileset |= 1 << file;
        }
        if front_span == 0 {
            //passer
            let rank = (i >> 3) as usize;
            score += PASSED_PAWN_VALUES[if color == 0 { 7 - rank } else { rank }];
        }
        if (get_adjacent_files(file) & pawns) == 0 {
            //isolated pawn
            score -= if is_open { 20 } else { 10 };
        }
    }
    return (score, fileset, middle_game, endgame);
}
fn evaluate_pawns(
    tt: &mut TranspositionTable,
    hash: u64,
    endgame: (f32, f32),
    middle_game: (f32, f32),
    wp: u64,
    bp: u64,
) -> (i16, u8, u8) {
    let entry = tt.look_up_pawn_structure(hash);
    if entry.is_some() {
        unsafe { PAWN_TT_HITS += 1 }
        let pawn_data = entry.unwrap();
        let score = pawn_data.eval
            + (pawn_data.w_pst.0 as f32 * middle_game.1
                + pawn_data.w_pst.1 as f32 * endgame.1
                + pawn_data.b_pst.0 as f32 * middle_game.0
                + pawn_data.b_pst.1 as f32 * endgame.0) as i16;
        // + get_value(10, 20, endgame.0) * pawn_data.unhealthy_pawns_count.1 as i16
        // - get_value(10, 20, endgame.1) * pawn_data.unhealthy_pawns_count.0 as i16;
        return (score, pawn_data.w_filesets, pawn_data.b_filesets);
    } else {
        let w_data = get_pawn_data(wp, bp, 0);
        let b_data = get_pawn_data(bp, wp, 1);
        let mut score = w_data.0 - b_data.0;
        score += (w_data.1 as f32 * middle_game.1
            + w_data.2 as f32 * endgame.1
            + b_data.1 as f32 * middle_game.0
            + b_data.2 as f32 * endgame.0) as i16;
        tt.set_pawn_struct(
            hash,
            w_data.1,
            b_data.1,
            (b_data.2, b_data.3),
            (w_data.2, w_data.3),
            w_data.0 - b_data.0,
        );
        return (score, w_data.1, b_data.1);
    };
}
fn evaluate_rooks(
    wr: u64,
    br: u64,
    open: u8,
    semi_open_white: u8,
    semi_open_black: u8,
    closed: u8,
) -> i16 {
    let mut score = 0;
    //closed files: -10
    score -= (get_fileset_bb(closed) & wr).count_ones() as i16 * 10
        - (get_fileset_bb(closed) & br).count_ones() as i16 * 10;

    score += (get_fileset_bb(open) & wr).count_ones() as i16 * 30;

    score += (get_fileset_bb(semi_open_white) & wr).count_ones() as i16 * 20;

    score -= (get_fileset_bb(open) & br).count_ones() as i16 * 30;
    score -= (get_fileset_bb(semi_open_black) & br).count_ones() as i16 * 20;
    return score;
}
