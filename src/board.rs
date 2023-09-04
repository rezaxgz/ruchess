use std::str::FromStr;

use chess::{BitBoard, Board, BoardStatus, CastleRights, ChessMove, Color, Piece, Square};

use crate::data::{get_pst_value, PAWN_ZOBRIST};
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Position {
    pub board: Board,
    pst_values: i16,
    pawn_hash: u64,
}
impl Position {
    pub fn make_move_new(&self, mv: ChessMove) -> Position {
        let piece = self.board.piece_on(mv.get_source()).unwrap();
        let captured = self.board.piece_on(mv.get_dest());
        let from = mv.get_source().to_index();
        let to = mv.get_dest().to_index();
        let is_capture = captured.is_some();
        let turn = self.board.side_to_move().to_index();
        let xturn = turn ^ 1;
        let promotion = mv.get_promotion();
        let mut pst = 0;
        if piece != Piece::Pawn && piece != Piece::King {
            pst += get_pst_value(turn, piece, to) - get_pst_value(turn, piece, from);
        }
        if promotion.is_some() {
            pst += get_pst_value(turn, promotion.unwrap(), to);
        }
        if is_capture && captured.unwrap() != Piece::Pawn {
            pst -= get_pst_value(xturn, captured.unwrap(), to);
        }
        let is_en_pessant = !is_capture
            && piece == Piece::Pawn
            && mv.get_dest().get_file() != mv.get_source().get_file();
        let mut hash = 0;
        if piece == Piece::Pawn {
            hash ^= PAWN_ZOBRIST[turn][from];
            if promotion.is_none() {
                hash ^= PAWN_ZOBRIST[turn][to];
            }
            if is_en_pessant {
                hash ^= PAWN_ZOBRIST[xturn][if turn == 0 { to - 8 } else { to + 8 }];
            }
        }
        if captured.is_some() && captured.unwrap() == Piece::Pawn {
            hash ^= PAWN_ZOBRIST[xturn][to];
        }
        return Position {
            board: self.board.make_move_new(mv),
            pst_values: self.pst_values + pst,
            pawn_hash: self.pawn_hash ^ hash,
        };
    }
    pub fn color_combined(&self, color: Color) -> &BitBoard {
        return self.board.color_combined(color);
    }
    pub fn get_hash(&self) -> u64 {
        return self.board.get_hash();
    }
    pub fn get_pawn_hash(&self) -> u64 {
        return self.pawn_hash;
    }
    pub fn pieces(&self, piece: Piece) -> u64 {
        return self.board.pieces(piece).0;
    }
    pub fn checkers(&self) -> &BitBoard {
        return self.board.checkers();
    }
    pub fn piece_on(&self, sq: Square) -> Option<Piece> {
        return self.board.piece_on(sq);
    }
    pub fn side_to_move(&self) -> Color {
        return self.board.side_to_move();
    }
    pub fn king_square(&self, color: Color) -> Square {
        return self.board.king_square(color);
    }
    pub fn get_pst_values(&self) -> i16 {
        return self.pst_values;
    }
    pub fn status(&self) -> BoardStatus {
        return self.board.status();
    }
    pub fn castle_rights(&self, color: Color) -> CastleRights {
        return self.board.castle_rights(color);
    }
    #[inline]
    pub fn new(fen: &str) -> Position {
        let board = Board::from_str(fen).unwrap();
        let mut pst_values: i16 = 0;
        let mut hash = 0;
        for color in 0..2 {
            let mut pieces = board
                .color_combined(if color == 0 {
                    Color::White
                } else {
                    Color::Black
                })
                .0;
            while pieces != 0 {
                let sq = pieces.trailing_zeros() as u8;
                unsafe {
                    let piece = board.piece_on(Square::new(sq)).unwrap();
                    if piece != Piece::Pawn && piece != Piece::King {
                        pst_values += get_pst_value(color, piece, sq as usize);
                    }
                    if piece == Piece::Pawn {
                        hash ^= PAWN_ZOBRIST[color][sq as usize];
                    }
                }
                pieces &= pieces - 1;
            }
        }
        return Position {
            board,
            pst_values,
            pawn_hash: hash,
        };
    }
    #[inline]
    pub fn default() -> Position {
        return Position::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    }
}
