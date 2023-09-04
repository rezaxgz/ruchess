use crate::board::Position;
use crate::data::{
    calc_king_pst, get_adjacent_files, get_distance_from_center, get_fileset_bb, get_front_spans,
    get_orthogonal_distance, ADJACENT_FILESETS, DARK_SQUARES, FILES, LIGHT_SQUARES,
    PAWN_SQUARE_TABLES, PIECE_SQUARE_TABLES, SECOND_RANK, SEVENTH_RANK,
};
use crate::transposition_table::TranspositionTable;
use chess::{
    CastleRights, Color::Black, Color::White, Piece::Bishop, Piece::Knight, Piece::Pawn,
    Piece::Queen, Piece::Rook,
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
const UNHEALTHY_PAWN_PENALTY: i16 = 10;
const OPEN_UNHEALTHY_PAWN_PENALTY: i16 = 10;
const KING_SIDE_CASTLE_FILESET: u8 = ADJACENT_FILESETS[6];
const QUEEN_SIDE_CASTLE_FILESET: u8 = ADJACENT_FILESETS[2];
const PAWN_STORM_PENALTY: [i16; 8] = [0, 0, -60, -30, -10, 0, 0, 0];
fn get_value<T>(m: T, e: T, endgame: f32) -> T {
    if endgame == 0.0 {
        return m;
    }
    return e;
}
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
fn get_material(board: &Position, color: u64) -> (f32, i16) {
    let material = (board.pieces(Knight) & color).count_ones() * KNIGHT_VALUE
        + (board.pieces(Bishop) & color).count_ones() * BISHOP_VALUE
        + (board.pieces(Rook) & color).count_ones() * ROOK_VALUE
        + (board.pieces(Queen) & color).count_ones() * QUEEN_VALUE;
    return (
        material as f32,
        (material + ((board.pieces(Pawn) & color).count_ones() * PAWN_VALUE)) as i16,
    );
}
pub fn evaluate(board: &Position, tt: &mut TranspositionTable) -> i16 {
    let white_combined = board.color_combined(White).0;
    let black_combined = board.color_combined(Black).0;

    let wp = board.pieces(Pawn) & white_combined;
    let bp = board.pieces(Pawn) & black_combined;
    let wr = board.pieces(Rook) & white_combined;
    let br = board.pieces(Rook) & black_combined;
    let wk = board.king_square(White).to_index();
    let bk = board.king_square(Black).to_index();

    let (white_material_without_pawns, white_material) = get_material(board, white_combined);
    let (black_material_without_pawns, black_material) = get_material(board, black_combined);

    let white_endgame = get_endgame_weight(white_material_without_pawns);
    let black_endgame = get_endgame_weight(black_material_without_pawns);

    let white_middlegame = 1.0 - white_endgame;
    let black_middlegame = 1.0 - black_endgame;

    let piece_scores = board.get_pst_values()
        + calc_king_pst(0, wk, black_endgame, black_middlegame)
        + calc_king_pst(1, bk, white_endgame, white_middlegame);

    let mop_eval: i16 = mop_up_eval(
        wk,
        bk,
        white_material_without_pawns,
        black_material_without_pawns,
        black_endgame,
    ) - mop_up_eval(
        bk,
        wk,
        black_material_without_pawns,
        white_material_without_pawns,
        white_endgame,
    );

    let (pawn_eval, wp_fileset, bp_fileset) = evaluate_pawns(
        tt,
        board.get_pawn_hash(),
        (white_endgame, black_endgame),
        (white_middlegame, black_middlegame),
        wp,
        bp,
    );
    let closed = wp_fileset & bp_fileset;
    let open = (!wp_fileset) & (!bp_fileset);
    let semi_open_white = bp_fileset & (!wp_fileset);
    let semi_open_black = wp_fileset & (bp_fileset);

    let rooks_eval = evaluate_rooks(
        wr,
        br,
        get_fileset_bb(open),
        get_fileset_bb(semi_open_white),
        get_fileset_bb(semi_open_black),
        get_fileset_bb(closed),
        wk,
        bk,
        (white_endgame, black_endgame),
    );

    let bishop_eval = evaluate_bishop_pair(board.pieces(Bishop) & white_combined)
        - evaluate_bishop_pair(board.pieces(Bishop) & black_combined);

    let queens_eval = evaluate_queens(board.pieces(Queen) & white_combined, bk)
        - evaluate_queens(board.pieces(Queen) & black_combined, wk);

    let seventh_rank_value = if (bp & SEVENTH_RANK) != 0 || bk > 55 {
        seventh_rank_bounus(
            board.pieces(Queen) & white_combined & SEVENTH_RANK,
            board.pieces(Rook) & white_combined & SEVENTH_RANK,
            black_endgame,
        )
    } else {
        0
    } - if (wp & SECOND_RANK) != 0 || wk < 8 {
        seventh_rank_bounus(
            board.pieces(Queen) & black_combined & SECOND_RANK,
            board.pieces(Rook) & black_combined & SECOND_RANK,
            white_endgame,
        )
    } else {
        0
    };
    let king_eval = if black_endgame != 0.0 && (board.pieces(Queen) & black_combined) != 0 {
        evaluate_king_safety(wp, bp, wk, 0, board.castle_rights(White))
    } else {
        0
    } - if white_endgame != 0.0 && (board.pieces(Queen) & white_combined) != 0 {
        evaluate_king_safety(bp, wp, bk, 1, board.castle_rights(Black))
    } else {
        0
    };
    let tempo_bounus = if board.side_to_move() == White {
        get_value(20, 10, black_endgame)
    } else {
        get_value(-20, -10, white_endgame)
    };

    let eval = white_material - black_material
        + mop_eval
        + piece_scores
        + pawn_eval
        + bishop_eval
        + rooks_eval
        + queens_eval
        + seventh_rank_value
        + tempo_bounus
        + king_eval;
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
            score -= if is_open {
                OPEN_UNHEALTHY_PAWN_PENALTY
            } else {
                UNHEALTHY_PAWN_PENALTY
            };
        } else {
            fileset |= 1 << file;
        }
        if front_span == 0 {
            //passer
            let rank = (i >> 3) as usize;
            score += PASSED_PAWN_VALUES[if color == 0 { 7 - rank } else { rank }]
        }
        if (get_adjacent_files(file) & pawns) == 0 {
            //isolated pawn
            score -= if is_open {
                OPEN_UNHEALTHY_PAWN_PENALTY
            } else {
                UNHEALTHY_PAWN_PENALTY
            };
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
        return (score, pawn_data.w_filesets, pawn_data.b_filesets);
    } else {
        let w_data = get_pawn_data(wp, bp, 0);
        let b_data = get_pawn_data(bp, wp, 1);
        let score = w_data.0 - b_data.0
            + (w_data.1 as f32 * middle_game.1
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
    open: u64,
    semi_open_white: u64,
    semi_open_black: u64,
    closed: u64,
    wk: usize,
    bk: usize,
    endgame: (f32, f32),
) -> i16 {
    let mut score = 0;
    let w_adjacent = get_adjacent_files(wk & 7) & br;
    let b_adjacent = get_adjacent_files(bk & 7) & wr;
    let w_file = FILES[wk & 7] & br;
    let b_file = FILES[bk & 7] & wr;
    //closed files: -10
    score -= (closed & wr).count_ones() as i16 * 10 - (closed & br).count_ones() as i16 * 10;

    //open file
    score += (open & wr).count_ones() as i16 * 10
        + (open & b_adjacent).count_ones() as i16 * get_value(20, 10, endgame.1)
        + (open & b_file).count_ones() as i16 * get_value(30, 10, endgame.1);

    score -= (open & br).count_ones() as i16 * 10
        + (open & w_adjacent).count_ones() as i16 * get_value(20, 10, endgame.0)
        + (open & w_file).count_ones() as i16 * get_value(20, 10, endgame.0);

    if endgame.1 == 0.0 {
        score += (semi_open_white & b_adjacent).count_ones() as i16 * 10
            + (semi_open_white & b_file).count_ones() as i16 * 20;
    }
    if endgame.0 == 0.0 {
        score -= (semi_open_black & w_adjacent).count_ones() as i16 * 10
            + (semi_open_black & w_file).count_ones() as i16 * 20;
    }
    return score;
}
fn evaluate_queens(mut queens: u64, their_king: usize) -> i16 {
    let mut score = 0;
    while queens != 0 {
        score += 10 - get_orthogonal_distance(queens.trailing_zeros() as usize, their_king) as i16;
        queens &= queens - 1;
    }
    return score;
}
fn seventh_rank_bounus(queens: u64, rooks: u64, endgame: f32) -> i16 {
    return (rooks.count_ones() * get_value(10, 30, endgame)
        + queens.count_ones() * get_value(10, 20, endgame)) as i16;
}
fn evaluate_pawn_shield(pawns: u64, king: usize, color: usize) -> i16 {
    let mut score = 0;
    let mut fileset = ADJACENT_FILESETS[king & 7];
    let king_file = king >> 3;
    while fileset != 0 {
        let file = fileset.trailing_zeros() as usize;
        let file_bb = FILES[file] & pawns;
        fileset &= fileset - 1;
        let penalty = if file_bb == 0 {
            36
        } else {
            let pawn = if color == 0 {
                file_bb.trailing_zeros()
            } else {
                63 - file_bb.leading_zeros()
            };
            let distance_to_8 = if color == 0 {
                7 - (pawn >> 3)
            } else {
                pawn >> 3
            } as i16;
            36 - (distance_to_8 * distance_to_8)
        };
        if file == king_file {
            score -= penalty << 1;
        } else {
            score -= penalty;
        }
    }
    return score;
}
fn evaluate_pawn_storm(their_pawns: u64, mut fileset: u8, color: usize) -> i16 {
    let mut score = 0;
    while fileset != 0 {
        let file = fileset.trailing_zeros() as usize;
        let bb = FILES[file] & their_pawns;
        fileset &= fileset - 1;
        if bb != 0 {
            let pawn = if color == 0 {
                bb.trailing_zeros()
            } else {
                63 - bb.leading_zeros()
            };
            score += PAWN_STORM_PENALTY[if color == 0 {
                (pawn >> 3) as usize
            } else {
                (7 - (pawn >> 3)) as usize
            }];
        }
    }
    return score;
}
fn evaluate_king_safety(
    my_pawns: u64,
    their_pawns: u64,
    king: usize,
    color: usize,
    castling_rights: CastleRights,
) -> i16 {
    let mut storm_value = evaluate_pawn_storm(their_pawns, ADJACENT_FILESETS[king & 7], color);
    if castling_rights != CastleRights::NoRights {
        let value = if castling_rights == CastleRights::KingSide {
            evaluate_pawn_storm(their_pawns, KING_SIDE_CASTLE_FILESET, color)
        } else if castling_rights == CastleRights::QueenSide {
            evaluate_pawn_storm(their_pawns, QUEEN_SIDE_CASTLE_FILESET, color)
        } else {
            std::cmp::max(
                evaluate_pawn_storm(their_pawns, KING_SIDE_CASTLE_FILESET, color),
                evaluate_pawn_storm(their_pawns, QUEEN_SIDE_CASTLE_FILESET, color),
            )
        };
        storm_value = (storm_value + value) / 2;
    }
    return evaluate_pawn_shield(my_pawns, king, color) + storm_value;
}
