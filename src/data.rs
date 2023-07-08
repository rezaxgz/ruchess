use chess::{Color, Piece, Square};

static mut DISTANCE_FROM_CENTER: [u8; 64] = [0; 64];
static mut SQUARE_DISTANCE: [[u8; 64]; 64] = [[0; 64]; 64];
static mut ORTHOGONAL_DISTANCE: [[u8; 64]; 64] = [[0; 64]; 64];
pub const PIECE_SQUARE_TABLES: [[[i16; 64]; 6]; 2] = [
    [
        [
            0, 0, 0, 0, 0, 0, 0, 0, 5, 10, 10, -20, -20, 10, 10, 5, 5, -5, -10, 0, 0, -10, -5, 5,
            0, 0, 0, 20, 20, 0, 0, 0, 5, 5, 10, 25, 25, 10, 5, 5, 10, 10, 20, 30, 30, 20, 10, 10,
            50, 50, 50, 50, 50, 50, 50, 50, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        [
            -50, -40, -30, -30, -30, -30, -40, -50, -40, -20, 0, 5, 5, 0, -20, -40, -30, 5, 10, 15,
            15, 10, 5, -30, -30, 0, 15, 20, 20, 15, 0, -30, -30, 5, 15, 20, 20, 15, 5, -30, -30, 0,
            10, 15, 15, 10, 0, -30, -40, -20, 0, 0, 0, 0, -20, -40, -50, -40, -30, -30, -30, -30,
            -40, -50,
        ],
        [
            -20, -10, -10, -10, -10, -10, -10, -20, -10, 5, 0, 0, 0, 0, 5, -10, -10, 10, 10, 10,
            10, 10, 10, -10, -10, 0, 10, 10, 10, 10, 0, -10, -10, 5, 5, 10, 10, 5, 5, -10, -10, 0,
            5, 10, 10, 5, 0, -10, -10, 0, 0, 0, 0, 0, 0, -10, -20, -10, -10, -10, -10, -10, -10,
            -20,
        ],
        [
            0, 0, 0, 5, 5, 2, 0, 0, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0,
            0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, 5, 10, 10, 10, 10,
            10, 10, 5, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        [
            -20, -10, -10, -5, -5, -10, -10, -20, -10, 0, 0, 0, 0, 5, 0, -10, -10, 0, 5, 5, 5, 5,
            5, -10, -5, 0, 5, 5, 5, 5, 0, 0, -5, 0, 5, 5, 5, 5, 0, -5, -10, 0, 5, 5, 5, 5, 0, -10,
            -10, 0, 0, 0, 0, 0, 0, -10, -20, -10, -10, -5, -5, -10, -10, -20,
        ],
        [
            20, 30, 10, 0, 0, 10, 30, 20, 20, 20, -5, -5, -5, -5, 20, 20, -10, -20, -20, -20, -20,
            -20, -20, -10, -20, -30, -30, -40, -40, -30, -30, -20, -30, -40, -40, -50, -50, -40,
            -40, -30, -40, -50, -50, -60, -60, -50, -50, -40, -60, -60, -60, -60, -60, -60, -60,
            -60, -80, -70, -70, -70, -70, -70, -70, -80,
        ],
    ],
    [
        [
            -0, -0, -0, -0, -0, -0, -0, -0, -50, -50, -50, -50, -50, -50, -50, -50, -10, -10, -20,
            -30, -30, -20, -10, -10, -5, -5, -10, -25, -25, -10, -5, -5, -0, -0, -0, -20, -20, -0,
            -0, -0, -5, 5, 10, -0, -0, 10, 5, -5, -5, -10, -10, 20, 20, -10, -10, -5, -0, -0, -0,
            -0, -0, -0, -0, -0,
        ],
        [
            50, 40, 30, 30, 30, 30, 40, 50, 40, 20, -0, -0, -0, -0, 20, 40, 30, -0, -10, -15, -15,
            -10, -0, 30, 30, -5, -15, -20, -20, -15, -5, 30, 30, -0, -15, -20, -20, -15, -0, 30,
            30, -5, -10, -15, -15, -10, -5, 30, 40, 20, -0, -5, -5, -0, 20, 40, 50, 40, 30, 30, 30,
            30, 40, 50,
        ],
        [
            20, 10, 10, 10, 10, 10, 10, 20, 10, -0, -0, -0, -0, -0, -0, 10, 10, -0, -5, -10, -10,
            -5, -0, 10, 10, -5, -5, -10, -10, -5, -5, 10, 10, -0, -10, -10, -10, -10, -0, 10, 10,
            -10, -10, -10, -10, -10, -10, 10, 10, -5, -0, -0, -0, -0, -5, 10, 20, 10, 10, 10, 10,
            10, 10, 20,
        ],
        [
            -0, -0, -0, -0, -0, -0, -0, -0, -5, -10, -10, -10, -10, -10, -10, -5, 5, -0, -0, -0,
            -0, -0, -0, 5, 5, -0, -0, -0, -0, -0, -0, 5, 5, -0, -0, -0, -0, -0, -0, 5, 5, -0, -0,
            -0, -0, -0, -0, 5, 5, -0, -0, -0, -0, -0, -0, 5, -0, -0, -0, -5, -5, -2, -0, -0,
        ],
        [
            20, 10, 10, 5, 5, 10, 10, 20, 10, -0, -0, -0, -0, -0, -0, 10, 10, -0, -5, -5, -5, -5,
            -0, 10, 5, -0, -5, -5, -5, -5, -0, 5, -0, -0, -5, -5, -5, -5, -0, 5, 10, -5, -5, -5,
            -5, -5, -0, 10, 10, -0, -5, -0, -0, -0, -0, 10, 20, 10, 10, 5, 5, 10, 10, 20,
        ],
        [
            80, 70, 70, 70, 70, 70, 70, 80, 60, 60, 60, 60, 60, 60, 60, 60, 40, 50, 50, 60, 60, 50,
            50, 40, 30, 40, 40, 50, 50, 40, 40, 30, 20, 30, 30, 40, 40, 30, 30, 20, 10, 20, 20, 20,
            20, 20, 20, 10, -20, -20, 5, 5, 5, 5, -20, -20, -20, -30, -10, 0, 0, -10, -30, -20,
        ],
    ],
];
pub const KING_SQUARE_TABLES: [[i16; 64]; 2] = [
    [
        -50, -30, -30, -30, -30, -30, -30, -50, -30, -25, 0, 0, 0, 0, -25, -30, -25, -20, 20, 25,
        25, 20, -20, -25, -20, -15, 30, 40, 40, 30, -15, -20, -15, -10, 35, 45, 45, 35, -10, -15,
        -10, -5, 20, 30, 30, 20, -5, -10, -5, 0, 5, 5, 5, 5, 0, -5, -20, -10, -10, -10, -10, -10,
        -10, -20,
    ],
    [
        20, 10, 10, 10, 10, 10, 10, 20, 5, 0, -5, -5, -5, -5, 0, 5, 10, 5, -20, -30, -30, -20, 5,
        10, 15, 10, -35, -45, -45, -35, 10, 15, 20, 15, -30, -40, -40, -30, 15, 20, 25, 20, -20,
        -25, -25, -20, 20, 25, 30, 25, 0, 0, 0, 0, 25, 30, 50, 30, 30, 30, 30, 30, 30, 50,
    ],
];
pub const PAWN_SQUARE_TABLES: [[i16; 64]; 2] = [
    [
        0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 20,
        20, 20, 20, 20, 20, 20, 20, 30, 30, 30, 30, 30, 30, 30, 30, 50, 50, 50, 50, 50, 50, 50, 50,
        80, 80, 80, 80, 80, 80, 80, 80, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, -80, -80, -80, -80, -80, -80, -80, -80, -50, -50, -50, -50, -50,
        -50, -50, -50, -30, -30, -30, -30, -30, -30, -30, -30, -20, -20, -20, -20, -20, -20, -20,
        -20, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, 0, 0,
        0, 0, 0, 0, 0, 0,
    ],
];
pub static mut SORT_PIECE_SQ_TABLE: [[[i8; 64]; 6]; 2] = [[[0; 64]; 6]; 2];
pub static mut FRONT_SPANS: [[u64; 64]; 2] = [[0; 64]; 2];
pub const RANKS: [u64; 8] = [
    0xFF,
    0xFF00,
    0xFF0000,
    0xFF000000,
    0xFF00000000,
    0xFF0000000000,
    0xFF000000000000,
    0xFF00000000000000,
];
pub const FILES: [u64; 8] = [
    0x101010101010101,
    0x202020202020202,
    0x404040404040404,
    0x808080808080808,
    0x1010101010101010,
    0x2020202020202020,
    0x4040404040404040,
    0x8080808080808080,
];
pub fn get_spst_value(color: Color, piece: Piece, square: Square) -> i8 {
    if piece == Piece::King || piece == Piece::Pawn {
        return 0;
    }
    unsafe {
        return SORT_PIECE_SQ_TABLE[color.to_index()][piece.to_index()][square.to_index()];
    }
}
pub fn get_pst_value(
    color: usize,
    piece: Piece,
    square: usize,
    endgame: f32,
    middle_game: f32,
) -> i16 {
    if piece == Piece::King {
        return (PIECE_SQUARE_TABLES[color][5][square] as f32 * middle_game
            + KING_SQUARE_TABLES[color][square] as f32 * endgame) as i16;
    } else if piece == Piece::Pawn {
        return (PIECE_SQUARE_TABLES[color][0][square] as f32 * middle_game
            + PAWN_SQUARE_TABLES[color][square] as f32 * endgame) as i16;
    } else {
        return PIECE_SQUARE_TABLES[color][piece.to_index()][square];
    }
}
pub fn get_distance_from_center(sq: usize) -> u8 {
    unsafe {
        return DISTANCE_FROM_CENTER[sq];
    }
}
// pub fn get_sq_distance(sq1: usize, sq2: usize) -> u8 {
//     unsafe {
//         return SQUARE_DISTANCE[sq1][sq2];
//     }
// }
pub fn get_orthogonal_distance(sq1: usize, sq2: usize) -> u8 {
    unsafe {
        return ORTHOGONAL_DISTANCE[sq1][sq2];
    }
}
pub fn init() {
    for file in 0..8 {
        for rank in 0..8 {
            let sq: usize = rank as usize * 8 + file as usize;
            let files_from_center = std::cmp::max(3 - file, file - 4);
            let ranks_from_center = std::cmp::max(3 - rank, rank - 4);
            unsafe {
                DISTANCE_FROM_CENTER[sq] = ranks_from_center as u8 + files_from_center as u8;
            }
            for square_b in 0..64 {
                let rank_distance = i32::abs(rank - (square_b >> 3));
                let file_distance = i32::abs(file - (square_b & 7));
                unsafe {
                    ORTHOGONAL_DISTANCE[sq][square_b as usize] =
                        (file_distance + rank_distance) as u8;
                    SQUARE_DISTANCE[sq][square_b as usize] =
                        std::cmp::max(file_distance, rank_distance) as u8;
                }
            }
        }
    }
    unsafe {
        SORT_PIECE_SQ_TABLE =
            PIECE_SQUARE_TABLES.map(|clr| clr.map(|piece| piece.map(|sq| (sq / 5) as i8)));
    }
    for file in 0..8 {
        for rank in 0..8 {
            let i = rank * 8 + file;
            let mut front_ranks: u64 = 0;
            for r in rank + 1..8 {
                front_ranks |= RANKS[r];
            }
            let mut files = FILES[file];
            if file > 0 {
                files |= FILES[file - 1];
            }
            if file < 7 {
                files |= FILES[file + 1];
            }
            unsafe {
                FRONT_SPANS[1][i] = front_ranks & files;
            }
            front_ranks = 0;
            for r in 0..rank {
                front_ranks |= RANKS[r];
            }
            unsafe {
                FRONT_SPANS[0][i] = front_ranks & files;
            }
        }
    }
}
