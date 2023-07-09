use chess::ChessMove;

const PIECE_CHARS: [char; 12] = ['p', 'P', 'n', 'N', 'b', 'B', 'r', 'R', 'q', 'Q', 'k', 'K'];
const FILES: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
pub struct Board {
    pub board: [u64; 12],
    pub colors: [u64; 2],
    pub full: u64,
    pub turn: u8,
    pub xturn: u8,
    pub ep: u8,
    pub castle: u8,
    pub halfmoves: u8,
    pub fullmoves: u16,
}
pub fn init_board(fen: String) -> Board {
    let mut board: [u64; 12] = [0; 12];
    let mut full: u64 = 0;
    let mut colors: [u64; 2] = [0, 0];
    let mut turn: u8 = 1;
    let mut xturn: u8 = 0;
    let mut castle: u8 = 0;
    let mut index = -0;
    let mut ep = 0;
    let chars = fen.chars();
    let mut half_moves = String::from("");
    let mut full_moves = String::from("");
    let mut row = 7;
    let mut file = 0;
    for c in chars {
        if c == ' ' {
            index += 1;
            continue;
        }
        if index == 0 {
            if c == '/' {
                row -= 1;
                file = 0;
                continue;
            }
            if c.is_digit(10) {
                let num = c.to_digit(10).unwrap();
                file += num;
                continue;
            }
            let sq = (row * 8) + file;
            let square: u64 = 1 << sq;
            for i in 0..12 {
                if PIECE_CHARS[i] == c {
                    board[i] |= square;
                    full |= square;
                    colors[i & 1] |= square;
                    break;
                }
            }
            file += 1;
        } else if index == 1 && c == 'b' {
            turn = 0;
            xturn = 1;
        } else if index == 2 {
            match c {
                'K' => castle |= 1,
                'Q' => castle |= 2,
                'k' => castle |= 4,
                'q' => castle |= 8,
                _ => println!("invalid char is castle string"),
            }
        } else if index == 3 {
            if c == '-' {
                continue;
            }
            if ep == 0 {
                for i in 0..8 {
                    if c == FILES[i] {
                        ep += 1;
                        break;
                    }
                }
                continue;
            }
            ep += (c.to_digit(10).unwrap() - 1) * 8;
        } else if index == 4 {
            half_moves.push(c);
        } else if index == 5 {
            full_moves.push(c);
        }
    }
    let board = Board {
        board: board,
        colors: colors,
        full: full,
        turn: turn,
        xturn: xturn,
        ep: ep as u8,
        castle: castle,
        halfmoves: half_moves.parse::<u8>().unwrap(),
        fullmoves: full_moves.parse::<u16>().unwrap(),
    };
    return board;
}
pub fn print_board(board: Board) {
    for row in (0..8).rev() {
        for file in 0..8 {
            let square: u64 = 1 << (row * 8 + file);
            if board.full & square != 0 {
                for piece in 0..12 {
                    if board.board[piece] & square != 0 {
                        print!("{}", PIECE_CHARS[piece]);
                    }
                }
            } else {
                print!(" ");
            }
            if file < 7 {
                print!(" ,");
            }
        }
        println!();
    }
    println!(
        "turn: {}, xturn: {}, ep: {}, castle: {}, full_moves: {}, half_moves: {}",
        board.turn, board.xturn, board.ep, board.castle, board.fullmoves, board.halfmoves
    );
}
#[allow(dead_code)]
pub fn print_bitboard(bb: u64) {
    for row in (0..8).rev() {
        println!("");
        for file in 0..8 {
            let square: u64 = 1 << (row * 8 + file);
            if (square & bb) != 0 {
                print!("1");
            } else {
                print!("0");
            }
        }
    }
    println!();
}
#[allow(dead_code)]
pub fn print_move_list(moves: &Vec<ChessMove>) {
    for m in moves {
        print!("{}, ", m.to_string());
    }
    println!();
}
