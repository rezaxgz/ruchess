use crate::data::{get_distance_from_center, get_orthogonal_distance, get_pst_value};
use chess::{Board, Color, Piece, Square};

const ENDGAME_MATERIAL_START: u32 = 500 * 2 + 320 + 300;
const MULTIPLIER: f32 = 1.0 / ENDGAME_MATERIAL_START as f32;
// const FLANKS: [u64; 3] = [
//     FILES[0] | FILES[1] | FILES[2],
//     FILES[3] | FILES[4],
//     FILES[5] | FILES[6] | FILES[7],
// ];
// const NOT_RANK_2: u64 = !0xFF00;
// const NOT_RANK_7: u64 = !0xFF000000000000;
// const RANK_2: u64 = 0xFF00;
// const RANK_7: u64 = 0xFF000000000000;
fn get_endgame_weight(material: u32) -> f32 {
    if material < ENDGAME_MATERIAL_START {
        return 1.0 - (material as f32 * MULTIPLIER);
    } else {
        return 0.0;
    };
}
pub fn evaluate(board: &Board) -> i16 {
    let mut white_eval: u32 = 0;
    let mut black_eval: u32 = 0;

    white_eval += (board.pieces(Piece::Knight) & board.color_combined(Color::White)).popcnt() * 300;
    white_eval += (board.pieces(Piece::Bishop) & board.color_combined(Color::White)).popcnt() * 320;
    white_eval += (board.pieces(Piece::Rook) & board.color_combined(Color::White)).popcnt() * 500;
    white_eval += (board.pieces(Piece::Queen) & board.color_combined(Color::White)).popcnt() * 900;
    let white_material_without_pawns = white_eval;
    white_eval += (board.pieces(Piece::Pawn) & board.color_combined(Color::White)).popcnt() * 100;
    black_eval += (board.pieces(Piece::Knight) & board.color_combined(Color::Black)).popcnt() * 300;
    black_eval += (board.pieces(Piece::Bishop) & board.color_combined(Color::Black)).popcnt() * 320;
    black_eval += (board.pieces(Piece::Rook) & board.color_combined(Color::Black)).popcnt() * 500;
    black_eval += (board.pieces(Piece::Queen) & board.color_combined(Color::Black)).popcnt() * 900;
    let black_material_without_pawns = black_eval;
    black_eval += (board.pieces(Piece::Pawn) & board.color_combined(Color::Black)).popcnt() * 100;

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
    let eval = white_eval as i16 - black_eval as i16 + mop_eval + piece_scores;
    if board.side_to_move() == Color::White {
        return eval;
    }
    return -eval;
}
fn mop_up_eval(
    my_king: usize,
    their_king: usize,
    my_material: u32,
    their_material: u32,
    endgame: f32,
) -> i16 {
    let mut score: i16 = 0;
    if my_material > (their_material + 200) && endgame > 0.0 {
        score += get_distance_from_center(their_king) as i16 * 10;
        score += (14 - get_orthogonal_distance(my_king, their_king) as i16) * 4
    }
    return score;
}
