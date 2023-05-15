use chess::{Board, Color, Piece};
pub fn evaluate(board: &Board) -> i32 {
    let mut white_eval: u32 = 0;
    let mut black_eval: u32 = 0;
    white_eval += (board.pieces(Piece::Pawn) & board.color_combined(Color::White)).popcnt() * 100;
    white_eval += (board.pieces(Piece::Knight) & board.color_combined(Color::White)).popcnt() * 300;
    white_eval += (board.pieces(Piece::Bishop) & board.color_combined(Color::White)).popcnt() * 320;
    white_eval += (board.pieces(Piece::Rook) & board.color_combined(Color::White)).popcnt() * 500;
    white_eval += (board.pieces(Piece::Queen) & board.color_combined(Color::White)).popcnt() * 900;
    black_eval += (board.pieces(Piece::Pawn) & board.color_combined(Color::Black)).popcnt() * 100;
    black_eval += (board.pieces(Piece::Knight) & board.color_combined(Color::Black)).popcnt() * 300;
    black_eval += (board.pieces(Piece::Bishop) & board.color_combined(Color::Black)).popcnt() * 320;
    black_eval += (board.pieces(Piece::Rook) & board.color_combined(Color::Black)).popcnt() * 500;
    black_eval += (board.pieces(Piece::Queen) & board.color_combined(Color::Black)).popcnt() * 900;
    if board.side_to_move() == Color::White {
        return (white_eval as i32) - (black_eval as i32);
    }
    return (black_eval as i32) - (white_eval as i32);
}
