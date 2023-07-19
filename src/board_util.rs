use chess::{Board, Color, Piece, Square};
fn piece_char(piece: Piece) -> char {
    match piece {
        Piece::King => 'k',
        Piece::Queen => 'q',
        Piece::Rook => 'r',
        Piece::Bishop => 'b',
        Piece::Knight => 'n',
        Piece::Pawn => 'p',
    }
}
pub fn print_board(board: &Board) {
    for row in (0..8).rev() {
        for file in 0..8 {
            let square = row * 8 + file;
            unsafe {
                let sq = Square::new(square);
                if board.piece_on(sq).is_some() {
                    if board.color_on(sq).unwrap() == Color::White {
                        print!("{}", piece_char(board.piece_on(sq).unwrap()).to_uppercase());
                    } else {
                        print!("{}", piece_char(board.piece_on(sq).unwrap()));
                    }
                } else {
                    print!(" ");
                }
            }
            if file < 7 {
                print!(" ,");
            }
        }
        println!();
    }
    println!(
        "{} to move",
        if board.side_to_move() == Color::White {
            "white"
        } else {
            "black"
        }
    )
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
