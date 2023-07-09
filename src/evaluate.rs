use crate::data::{
    get_distance_from_center, get_orthogonal_distance, get_pst_value, ADJACENT_FILES, FRONT_SPANS,
};
use chess::{Board, Color, Piece, Square};

const ENDGAME_MATERIAL_START: f32 = (500 * 2 + 320 + 300) as f32;
const MULTIPLIER: f32 = 1.0 / ENDGAME_MATERIAL_START as f32;
const PASSED_PAWN_VALUES: [i16; 7] = [0, 90, 60, 40, 25, 15, 15];
const ISOLATED_PAWN_PENALTY: [i16; 9] = [0, -10, -25, -50, -80, -85, -90, -95, -100];
// const FLANKS: [u64; 3] = [
//     FILES[0] | FILES[1] | FILES[2],
//     FILES[3] | FILES[4],
//     FILES[5] | FILES[6] | FILES[7],
// ];
// const NOT_RANK_2: u64 = !0xFF00;
// const NOT_RANK_7: u64 = !0xFF000000000000;
// const RANK_2: u64 = 0xFF00;
// const RANK_7: u64 = 0xFF000000000000;
fn get_endgame_weight(material: f32) -> f32 {
    if material < ENDGAME_MATERIAL_START {
        return 1.0 - (material * MULTIPLIER);
    } else {
        return 0.0;
    };
}
pub fn evaluate(board: &Board) -> i16 {
    let mut white_material: u32 = 0;
    let mut black_material: u32 = 0;

    let wp = board.pieces(Piece::Pawn) & board.color_combined(Color::White);
    let bp = board.pieces(Piece::Pawn) & board.color_combined(Color::Black);
    white_material +=
        (board.pieces(Piece::Knight) & board.color_combined(Color::White)).popcnt() * 300;
    white_material +=
        (board.pieces(Piece::Bishop) & board.color_combined(Color::White)).popcnt() * 320;
    white_material +=
        (board.pieces(Piece::Rook) & board.color_combined(Color::White)).popcnt() * 500;
    white_material +=
        (board.pieces(Piece::Queen) & board.color_combined(Color::White)).popcnt() * 900;
    let white_material_without_pawns = white_material as f32;
    white_material += wp.popcnt() * 100;
    black_material +=
        (board.pieces(Piece::Knight) & board.color_combined(Color::Black)).popcnt() * 300;
    black_material +=
        (board.pieces(Piece::Bishop) & board.color_combined(Color::Black)).popcnt() * 320;
    black_material +=
        (board.pieces(Piece::Rook) & board.color_combined(Color::Black)).popcnt() * 500;
    black_material +=
        (board.pieces(Piece::Queen) & board.color_combined(Color::Black)).popcnt() * 900;
    let black_material_without_pawns = black_material as f32;
    black_material += bp.popcnt() * 100;

    let white_endgame = get_endgame_weight(white_material_without_pawns);
    let black_endgame = get_endgame_weight(black_material_without_pawns);

    let white_middlegame = 1.0 - white_endgame;
    let black_middlegame = 1.0 - black_endgame;
    let mut piece_scores = 0;
    for i in 0..64 {
        unsafe {
            let sq = Square::new(i);
            let p = board.piece_on(sq);
            if p.is_some() {
                let piece = p.unwrap();
                let color = board.color_on(sq).unwrap().to_index();
                if color == 0 {
                    piece_scores +=
                        get_pst_value(color, piece, i as usize, black_endgame, black_middlegame);
                } else {
                    piece_scores +=
                        get_pst_value(color, piece, i as usize, white_endgame, white_middlegame);
                }
            }
        }
    }
    //TODO:
    //king safety
    let mop_eval: i16 = mop_up_eval(
        board.king_square(Color::White).to_index(),
        board.king_square(Color::Black).to_index(),
        white_material_without_pawns,
        black_material_without_pawns,
        black_endgame,
    ) - mop_up_eval(
        board.king_square(Color::Black).to_index(),
        board.king_square(Color::White).to_index(),
        black_material_without_pawns,
        white_material_without_pawns,
        white_endgame,
    );
    let pawn_eval = evaluate_pawns(wp.0, bp.0, 0) - evaluate_pawns(bp.0, wp.0, 1);
    let eval = white_material as i16 - black_material as i16 + mop_eval + piece_scores + pawn_eval;
    if board.side_to_move() == Color::White {
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
fn evaluate_pawns(pawns: u64, enemy_pawns: u64, color: usize) -> i16 {
    let mut score = 0;
    let mut p = pawns;
    let mut isolated_pawn_count = 0;
    while p != 0 {
        let i = p.trailing_zeros();
        let file = i & 7;
        unsafe {
            if FRONT_SPANS[color][i as usize] & enemy_pawns == 0 {
                let rank = (i >> 3) as usize;
                score += PASSED_PAWN_VALUES[if color == 0 { 7 - rank } else { rank }];
            }
            if (ADJACENT_FILES[file as usize] & pawns) == 0 {
                //isolated pawn
                isolated_pawn_count += 1;
            }
        }
        p &= p - 1;
    }
    score += ISOLATED_PAWN_PENALTY[isolated_pawn_count];
    return score;
}
