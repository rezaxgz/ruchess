use chess::{Color, Piece, Square};

static mut DISTANCE_FROM_CENTER: [u8; 64] = [0; 64];
static mut SQUARE_DISTANCE: [[i16; 64]; 64] = [[0; 64]; 64];
static mut ORTHOGONAL_DISTANCE: [[u8; 64]; 64] = [[0; 64]; 64];
#[rustfmt::skip]
pub const PIECE_SQUARE_TABLES: [[[i16; 64]; 6]; 2] = [
    [
        [//white pawns
            0 ,   0,   0,   0,   0,   0,   0,   0, 
            5 ,  10,  10, -20, -20,  10,  10,   5, 
            5 ,  -5, -10,   0,   0, -10,  -5,   5,
            0 ,   0,   0,  20,  20,   0,   0,   0, 
            5 ,   5,  10,  25,  25,  10,   5,   5, 
            10,  10,  20,  30,  30,  20,  10,  10,
            50,  50,  50,  50,  50,  50,  50,  50, 
            0 ,   0,   0,   0,   0,   0,   0,   0,
        ],
        [//white knights
            -50, -40, -30, -30, -30, -30, -40, -50, 
            -40, -20,   0,   5,   5,   0, -20, -40, 
            -30,   5,  10,  15,  15,  10,   5, -30, 
            -30,   0,  15,  20,  20,  15,   0, -30, 
            -30,   5,  15,  20,  20,  15,   5, -30,
            -30,   0,  10,  15,  15,  10,   0, -30, 
            -40, -20,   0,   0,   0,   0, -20, -40, 
            -50, -40, -30, -30, -30, -30, -40, -50,
        ],
        [//white bishops
            -20, -10, -10, -10, -10, -10, -10, -20, 
            -10,   5,   0,   0,   0,   0,   5, -10, 
            -10,  10,  10,  10,  10,  10,  10, -10, 
            -10,   0,  10,  10,  10,  10,   0, -10, 
            -10,   5,   5,  10,  10,   5,   5, -10, 
            -10,   0,   5,  10,  10,   5,   0, -10, 
            -10,   0,   0,   0,   0,   0,   0, -10, 
            -20, -10, -10, -10, -10, -10, -10, -20,
        ],
        [//white rooks
            0,  0,  0,  5,  5,  2,  0,  0, 
           -5,  0,  0,  0,  0,  0,  0, -5, 
           -5,  0,  0,  0,  0,  0,  0, -5, 
           -5,  0,  0,  0,  0,  0,  0, -5, 
           -5,  0,  0,  0,  0,  0,  0, -5, 
           -5,  0,  0,  0,  0,  0,  0, -5, 
            5, 10, 10, 10, 10, 10, 10,  5, 
            0,  0,  0,  0,  0,  0,  0,  0,
        ],
        [//white queen
            -20, -10, -10,  -5,  -5, -10, -10, -20, 
            -10,   0,   0,   0,   0,   5,   0, -10, 
            -10,   0,   5,   5,   5,   5,   5, -10, 
            -5,    0,   5,   5,   5,   5,   0,   0, 
            -5,    0,   5,   5,   5,   5,   0,  -5, 
            -10,   0,   5,   5,   5,   5,   0, -10,
            -10,   0,   0,   0,   0,   0,   0, -10, 
            -20, -10, -10,  -5,  -5, -10, -10, -20,
        ],
        [
             20,  30,  10,   0,   0,  10,  30,  20, 
             20,  20,  -5,  -5,  -5,  -5,  20,  20, 
            -10, -20, -20, -20, -20, -20, -20, -10, 
            -20, -30, -30, -40, -40, -30, -30, -20, 
            -30, -40, -40, -50, -50, -40, -40, -30, 
            -40, -50, -50, -60, -60, -50, -50, -40, 
            -60, -60, -60, -60, -60, -60, -60, -60, 
            -80, -70, -70, -70, -70, -70, -70, -80,
        ],
    ],
    [
        [//black pawns
              0,   0,   0,   0,   0,   0,   0,   0, 
            -50, -50, -50, -50, -50, -50, -50, -50, 
            -10, -10, -20, -30, -30, -20, -10, -10, 
             -5,  -5, -10, -25, -25, -10,  -5,  -5, 
              0,   0,   0, -20, -20,   0,   0,   0, 
             -5,   5,  10,   0,   0,  10,   5,  -5,
             -5, -10, -10,  20,  20, -10, -10,  -5, 
              0,   0,   0,   0,   0,   0,   0,   0,
        ],
        [//black knights
             50,  40,  30,  30,  30,  30,  40,  50, 
             40,  20,   0,   0,   0,   0,  20,  40, 
             30,   0, -10, -15, -15, -10,   0,  30, 
             30,  -5, -15, -20, -20, -15,  -5,  30, 
             30,   0, -15, -20, -20, -15,   0,  30,
             30,  -5, -10, -15, -15, -10,  -5,  30, 
             40,  20,   0,  -5,  -5,   0,  20,  40, 
             50,  40,  30,  30,  30,  30,  40,  50,
        ],
        [//black bishops
             20,  10,  10,  10,  10,  10,  10,  20, 
             10,   0,   0,   0,   0,   0,   0,  10, 
             10,   0,  -5, -10, -10,  -5,   0,  10, 
             10,  -5,  -5, -10, -10,  -5,  -5,  10, 
             10,   0, -10, -10, -10, -10,   0,  10, 
             10, -10, -10, -10, -10, -10, -10,  10, 
             10,  -5,   0,   0,   0,   0,  -5,  10, 
             20,  10,  10,  10,  10,  10,  10,  20,
        ],
        [//black rooks
              0,   0,   0,   0,   0,   0,   0,   0, 
             -5, -10, -10, -10, -10, -10, -10,  -5, 
              5,   0,   0,   0,   0,   0,   0,   5, 
              5,   0,   0,   0,   0,   0,   0,   5, 
              5,   0,   0,   0,   0,   0,   0,   5, 
              5,   0,   0,   0,   0,   0,   0,   5, 
              5,   0,   0,   0,   0,   0,   0,   5, 
              0,   0,   0,  -5,  -5,  -2,   0,   0,
        ],
        [//black queens
             20,  10,  10,   5,   5,  10,  10,  20, 
             10,   0,   0,   0,   0,   0,   0,  10, 
             10,   0,  -5,  -5,  -5,  -5,   0,  10, 
              5,   0,  -5,  -5,  -5,  -5,   0,   5,  
              0,   0,  -5,  -5,  -5,  -5,   0,   5, 
             10,  -5,  -5,  -5,  -5,  -5,   0,  10, 
             10,   0,  -5,   0,   0,   0,   0,  10, 
             20,  10,  10,   5,   5,  10,  10,  20,
        ],
        [//black king
             80,  70,  70,  70,  70,  70,  70,  80, 
             60,  60,  60,  60,  60,  60,  60,  60, 
             40,  50,  50,  60,  60,  50,  50,  40, 
             30,  40,  40,  50,  50,  40,  40,  30, 
             20,  30,  30,  40,  40,  30,  30,  20,  
             10,  20,  20,  20,  20,  20,  20,  10, 
            -20, -20,   5,   5,   5,   5, -20, -20, 
            -20, -30, -10,   0,   0, -10, -30, -20,
        ],
    ],
];
#[rustfmt::skip]
const KING_SQUARE_TABLES: [[i16; 64]; 2] = [
    [
        -50, -30, -30, -30, -30, -30, -30, -50, 
        -30, -25,   0,   0,   0,   0, -25, -30, 
        -25, -20,  20,  25,  25,  20, -20, -25, 
        -20, -15,  30,  40,  40,  30, -15, -20, 
        -15, -10,  35,  45,  45,  35, -10, -15, 
        -10,  -5,  20,  30,  30,  20,  -5, -10, 
         -5,   0,   5,   5,   5,   5,   0,  -5, 
        -20, -10, -10, -10, -10, -10, -10, -20,
    ],
    [
        20,  10,   10,  10,  10,  10,  10,  20, 
         5,   0,   -5,  -5,  -5,  -5,   0,   5, 
        10,   5,  -20, -30, -30, -20,   5,  10, 
        15,  10,  -35, -45, -45, -35,  10,  15, 
        20,  15,  -30, -40, -40, -30,  15,  20, 
        25,  20,  -20, -25, -25, -20,  20,  25, 
        30,  25,    0,   0,   0,   0,  25,  30, 
        50,  30,   30,  30,  30,  30,  30,  50,
    ],
];
#[rustfmt::skip]
pub const PAWN_SQUARE_TABLES: [[i16; 64]; 2] = [
    [
         0,  0,  0,  0,  0,  0,  0,  0, 
        10, 10, 10, 10, 10, 10, 10, 10, 
        10, 10, 10, 10, 10, 10, 10, 10, 
        20, 20, 20, 20, 20, 20, 20, 20, 
        30, 30, 30, 30, 30, 30, 30, 30, 
        50, 50, 50, 50, 50, 50, 50, 50,
        80, 80, 80, 80, 80, 80, 80, 80, 
         0,  0,  0,  0,  0,  0,  0,  0,
    ],
    [
          0,   0,   0,   0,   0,   0,   0,   0, 
        -80, -80, -80, -80, -80, -80, -80, -80, 
        -50, -50, -50, -50, -50, -50, -50, -50, 
        -30, -30, -30, -30, -30, -30, -30, -30, 
        -20, -20, -20, -20, -20, -20, -20, -20, 
        -10, -10, -10, -10, -10, -10, -10, -10, 
        -10, -10, -10, -10, -10, -10, -10, -10,
         0 ,   0,   0,   0,   0,   0,   0,   0,
    ],
];
pub const PAWN_ZOBRIST: [[u64; 64]; 2] = [
    [
        8085185598151760447,
        13946325585335384165,
        2968751616440035579,
        7041022842525770343,
        12917422304011698271,
        10052360050776251686,
        13699342445398323639,
        506573857888973691,
        11886874439810951006,
        13321128211373863361,
        8759922935711139237,
        6272023614444296008,
        10763222970733438174,
        16100965241052210334,
        7335507063161343948,
        4437339628074423870,
        8394786845659756955,
        17278309931297955272,
        15344764256493080466,
        5060076868008773000,
        12887984606205686529,
        10378348301582331723,
        16654624460559881141,
        5562172382807043852,
        5092287325892671854,
        13396853578277207988,
        14844752070250998485,
        1373098290062978218,
        6329027450582441694,
        7087920059585399402,
        10551797782285515443,
        9615121390358484351,
        709948133425849678,
        6423043237130300195,
        3008006698062651143,
        233213343324799528,
        7690788824206336646,
        18417575893370114286,
        4512104827508390788,
        13109509118233219187,
        14903015139044807688,
        3522116723393850159,
        11182691149835678343,
        14407773952884059449,
        2181417584272620251,
        17961847337695662156,
        14098576545695010209,
        15671501844129536668,
        10510435711806366109,
        5675830999581763190,
        7376863761877281391,
        17635353422420823681,
        14720315539620897069,
        7458002836843723420,
        2368817349156128604,
        18060931465287773585,
        15563011342426703550,
        7775944634769904345,
        3426351430552521287,
        10311865472744934538,
        11781042725034006691,
        4905555050952954108,
        4333776149487132986,
        2463367108216104083,
    ],
    [
        1974119971942181497,
        12122172008208551821,
        6062452120433435846,
        18396934563637245408,
        8044283600526533767,
        11399642828105987255,
        8987578966063644515,
        11056374512524529789,
        2095926713241145349,
        15632575041811929556,
        14766238854338441338,
        17890133514240121960,
        4376355677015825915,
        890288126683688722,
        6939599344405662403,
        18244768664859276680,
        3536795021499152893,
        12730147489584249157,
        6591604030498980534,
        3332675066383947135,
        6699201019113945888,
        9282854365328689576,
        9271666704537739704,
        6713588716128219818,
        12875847909595158622,
        2844291356417016583,
        13761885081363751180,
        17215576610366625388,
        14594744591378946410,
        2712865253957514908,
        15602993547219531013,
        5876508857414472499,
        2263777162867217488,
        14064797817611449044,
        1639959676191337423,
        12462746736495809728,
        17679389401087310828,
        5728471850377755580,
        834894166465115306,
        5025494886860419275,
        13888363611479590573,
        2683611855720127261,
        3382535446547267230,
        10049447372213223528,
        6992881842228855765,
        6718851911638198654,
        7944752306398536041,
        6557650052889617593,
        11550969363163114495,
        3607248707049579728,
        15951684984257691760,
        3342710414128087565,
        16532053739358337567,
        13350120864672994269,
        12060557997550275081,
        1940730362432768231,
        4892474443799080794,
        18363889940509909703,
        14286311574710607121,
        898492672058578451,
        9766764379781634153,
        6260715823505312240,
        539984552968257160,
        16594171728389449529,
    ],
];
pub const KING_ATTACKS_BITBOARD: [u64; 64] = [
    460551,
    986895,
    2039583,
    4079166,
    8158332,
    16316664,
    15790320,
    14737632,
    117901063,
    252645135,
    522133279,
    1044266558,
    2088533116,
    4177066232,
    4042322160,
    3772834016,
    30182672135,
    64677154575,
    133666119455,
    267332238910,
    534664477820,
    1069328955640,
    1034834473200,
    965845508320,
    7726764066560,
    16557351571200,
    34218526580480,
    68437053160960,
    136874106321920,
    273748212643840,
    264917625139200,
    247256450129920,
    1978051601039360,
    4238682002227200,
    8759942804602880,
    17519885609205760,
    35039771218411520,
    70079542436823040,
    67818912035635200,
    63297651233259520,
    506381209866076160,
    1085102592570163200,
    2242545357978337280,
    4485090715956674560,
    8970181431913349120,
    17940362863826698240,
    17361641481122611200,
    16204198715714437120,
    506381209748635648,
    1085102592318504960,
    2242545357458243584,
    4485090714916487168,
    8970181429832974336,
    17940362859665948672,
    17361641477096079360,
    16204198711956340736,
    506381179683864576,
    1085102527893995520,
    2242545224314257408,
    4485090448628514816,
    8970180897257029632,
    17940361794514059264,
    17361640446303928320,
    16204197749883666432,
];
pub const KNIGHT_MOVES: [u64; 64] = [
    132096,
    329728,
    659712,
    1319424,
    2638848,
    5277696,
    10489856,
    4202496,
    33816580,
    84410376,
    168886289,
    337772578,
    675545156,
    1351090312,
    2685403152,
    1075839008,
    8657044482,
    21609056261,
    43234889994,
    86469779988,
    172939559976,
    345879119952,
    687463207072,
    275414786112,
    2216203387392,
    5531918402816,
    11068131838464,
    22136263676928,
    44272527353856,
    88545054707712,
    175990581010432,
    70506185244672,
    567348067172352,
    1416171111120896,
    2833441750646784,
    5666883501293568,
    11333767002587136,
    22667534005174272,
    45053588738670592,
    18049583422636032,
    145241105196122112,
    362539804446949376,
    725361088165576704,
    1450722176331153408,
    2901444352662306816,
    5802888705324613632,
    11533718717099671552,
    4620693356194824192,
    288234782788157440,
    576469569871282176,
    1224997833292120064,
    2449995666584240128,
    4899991333168480256,
    9799982666336960512,
    1152939783987658752,
    2305878468463689728,
    1128098930098176,
    2257297371824128,
    4796069720358912,
    9592139440717824,
    19184278881435648,
    38368557762871296,
    4679521487814656,
    9077567998918656,
];
static mut SORT_PIECE_SQ_TABLE: [[[i8; 64]; 6]; 2] = [[[0; 64]; 6]; 2];
static mut FRONT_SPANS: [[u64; 64]; 2] = [[0; 64]; 2];
static mut ADJACENT_FILES: [u64; 8] = [0; 8];
static mut SUPPORTING_PAWNS: [u64; 64] = [0; 64];
static mut FILESETS: [u64; 256] = [0; 256];
pub const ADJACENT_FILESETS: [u8; 8] = [
    0b11, 0b111, 0b1110, 0b11100, 0b111000, 0b1110000, 0b11100000, 0b11000000,
];
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
pub const DARK_SQUARES: u64 = 0xAA55AA55AA55AA55;
pub const LIGHT_SQUARES: u64 = 0x55AA55AA55AA55AA;
pub const SEVENTH_RANK: u64 = RANKS[6];
pub const SECOND_RANK: u64 = RANKS[1];
pub fn get_spst_value(color: Color, piece: Piece, square: Square) -> i8 {
    if piece == Piece::King || piece == Piece::Pawn {
        return 0;
    }
    unsafe {
        return SORT_PIECE_SQ_TABLE[color.to_index()][piece.to_index()][square.to_index()];
    }
}
pub fn get_pst_value(color: usize, piece: Piece, square: usize) -> i16 {
    return PIECE_SQUARE_TABLES[color][piece.to_index()][square];
}
pub fn calc_king_pst(color: usize, square: usize, endgame: f32, middle_game: f32) -> i16 {
    return (PIECE_SQUARE_TABLES[color][5][square] as f32 * middle_game
        + KING_SQUARE_TABLES[color][square] as f32 * endgame) as i16;
}
pub fn get_distance_from_center(sq: usize) -> u8 {
    unsafe {
        return DISTANCE_FROM_CENTER[sq];
    }
}
//gets a bitboard of left and right files
pub fn get_adjacent_files(file: usize) -> u64 {
    unsafe {
        return ADJACENT_FILES[file];
    }
}
pub fn get_front_spans(color: usize, sq: usize) -> u64 {
    unsafe {
        return FRONT_SPANS[color][sq];
    }
}
pub fn get_fileset_bb(fileset: u8) -> u64 {
    unsafe {
        return FILESETS[fileset as usize];
    }
}
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
                        std::cmp::max(file_distance, rank_distance) as i16;
                }
            }
        }
    }
    unsafe {
        SORT_PIECE_SQ_TABLE =
            PIECE_SQUARE_TABLES.map(|clr| clr.map(|piece| piece.map(|sq| (sq / 5) as i8)));
    }
    for file in 0..8 {
        let mut files = 0;
        if file > 0 {
            files |= FILES[file - 1];
        }
        if file < 7 {
            files |= FILES[file + 1];
        }
        unsafe {
            ADJACENT_FILES[file] = files;
        }
        let adjacent_files = files;
        files |= FILES[file];
        for rank in 0..8 {
            let i = rank * 8 + file;
            let mut front_ranks: u64 = 0;
            for r in rank + 1..8 {
                front_ranks |= RANKS[r];
            }
            unsafe {
                FRONT_SPANS[0][i] = front_ranks & files;
                SUPPORTING_PAWNS[i] = adjacent_files & (!front_ranks);
            }
            front_ranks = 0;
            for r in 1..rank {
                front_ranks |= RANKS[r];
            }
            unsafe {
                FRONT_SPANS[1][i] = front_ranks & files;
            }
        }
    }
    for i in 0..256 {
        for j in 0..8 {
            if (i >> j) & 1 == 1 {
                unsafe {
                    FILESETS[i] |= FILES[j];
                }
            }
        }
    }
}
