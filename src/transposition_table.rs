use chess::{ChessMove, Piece, Square};

use crate::board::Position;

pub const NUM_OF_POSITIONS: usize = 0x400000;
pub const NUM_OF_PAWNS: usize = 0x40000;
const KEY: u64 = NUM_OF_POSITIONS as u64 - 1;
const PAWN_KEY: u64 = NUM_OF_PAWNS as u64 - 1;
const KILLERS_PER_PLY: usize = 3;
const KILLER_PLIES: usize = 20;
pub type Killers = [ChessMove; KILLERS_PER_PLY];

#[derive(PartialEq, Clone, Copy)]
pub enum EntryType {
    None,
    Exact,
    LowerBound,
    UpperBound,
}
#[derive(Clone, Copy)]
pub struct PositionEntry {
    pub key: u64,
    pub eval: i16,
    pub entry_type: EntryType,
    pub depth: u8,
    pub best_move: ChessMove,
    pub piece: Piece,
}
#[derive(Clone, Copy, Default)]
pub struct PawnEntry {
    pub hash: u64,
    pub w_filesets: u8,
    pub b_filesets: u8,
    pub w_pst: (i16, i16),
    pub b_pst: (i16, i16),
    // pub unhealthy_pawns_count: (u8, u8),
    pub eval: i16,
}
pub struct TranspositionTable {
    table: Vec<PositionEntry>,
    pawn_table: Vec<PawnEntry>,
    killers: [Killers; KILLER_PLIES],
    pub default_killers: Killers,
    pub history: [[[i32; 64]; 6]; 2],
    pub bf_history: [[[i32; 64]; 6]; 2],
    pub counters: [[[ChessMove; 64]; 6]; 2],
}
impl TranspositionTable {
    #[inline]
    pub fn init() -> TranspositionTable {
        let mut x = TranspositionTable {
            table: Vec::<PositionEntry>::with_capacity(NUM_OF_POSITIONS),
            pawn_table: Vec::<PawnEntry>::with_capacity(NUM_OF_PAWNS),
            killers: [[ChessMove::default(); KILLERS_PER_PLY]; KILLER_PLIES],
            default_killers: [ChessMove::default(); KILLERS_PER_PLY],
            history: [[[0; 64]; 6]; 2],
            bf_history: [[[1; 64]; 6]; 2],
            counters: [[[ChessMove::default(); 64]; 6]; 2],
        };
        for _i in 0..NUM_OF_POSITIONS {
            x.table.push(PositionEntry {
                key: 0,
                eval: 0,
                entry_type: EntryType::None,
                depth: 0,
                best_move: ChessMove::default(),
                piece: Piece::King,
            });
        }
        for _i in 0..NUM_OF_PAWNS {
            x.pawn_table.push(PawnEntry::default());
        }
        return x;
    }
    pub fn look_up_pos(&self, key: u64, board: &Position) -> Option<PositionEntry> {
        let res = self.table[(key & KEY) as usize];
        if (res.entry_type == EntryType::None)
            || (res.key != key)
            || (board.piece_on(res.best_move.get_source()) != Some(res.piece))
        {
            return None;
        }
        return Some(res);
    }
    pub fn set_pos(
        &mut self,
        key: u64,
        eval: i16,
        entry_type: EntryType,
        depth: u8,
        best_move: ChessMove,
        piece: Piece,
    ) {
        self.table[(key & KEY) as usize] = PositionEntry {
            key,
            eval,
            entry_type,
            depth,
            best_move,
            piece,
        };
        return;
    }
    pub fn look_up_pawn_structure(&self, key: u64) -> Option<PawnEntry> {
        let res = self.pawn_table[(key & PAWN_KEY) as usize];
        if res.hash == key {
            return Some(res);
        }
        return None;
    }
    pub fn set_pawn_struct(
        &mut self,
        hash: u64,
        w_filesets: u8,
        b_filesets: u8,
        w_pst: (i16, i16),
        b_pst: (i16, i16),
        eval: i16,
    ) {
        self.pawn_table[(hash & PAWN_KEY) as usize] = PawnEntry {
            hash,
            w_filesets,
            b_filesets,
            w_pst,
            b_pst,
            eval,
        };
    }
    pub fn clear(&mut self) {
        //pawn table isn't cleared because of low collision probability
        self.table = vec![
            PositionEntry {
                key: 0,
                eval: 0,
                entry_type: EntryType::None,
                depth: 0,
                best_move: ChessMove::default(),
                piece: Piece::King,
            };
            NUM_OF_POSITIONS
        ];
        self.killers = [[ChessMove::default(); KILLERS_PER_PLY]; KILLER_PLIES];
        self.history = [[[0; 64]; 6]; 2];
        self.bf_history = [[[1; 64]; 6]; 2];
        self.counters = [[[ChessMove::default(); 64]; 6]; 2];
    }
    pub fn get_pv(&self, board: &Position) -> Vec<ChessMove> {
        let mut pv = Vec::<ChessMove>::new();
        let mut hash = board.get_hash();
        let mut b = *board;
        loop {
            let res = self.look_up_pos(hash, &b);
            if res.is_none() {
                break;
            }
            b = b.make_move_new(res.unwrap().best_move);
            hash = b.get_hash();
            pv.push(res.unwrap().best_move);
        }
        return pv;
    }
    pub fn get_killers(&self, ply: usize) -> &Killers {
        if ply >= KILLER_PLIES {
            return &self.default_killers;
        }
        return &self.killers[ply];
    }
    pub fn store_killer(
        &mut self,
        ply: usize,
        depth: i32,
        mv: ChessMove,
        piece: Piece,
        color: usize,
        prev_piece: Piece,
        prev_sq: Square,
    ) {
        //set move for history heuristic
        self.history[color][piece.to_index()][mv.get_dest().to_index()] +=
            (1 << depth) / (self.bf_history[color][piece.to_index()][mv.get_dest().to_index()]);
        //set move for counter move heuristic
        self.counters[color][prev_piece.to_index()][prev_sq.to_index()] = mv;
        if ply >= KILLER_PLIES || self.killers[ply].contains(&mv) {
            return;
        }
        for i in 1..KILLERS_PER_PLY {
            self.killers[ply][i] = self.killers[ply][i - 1];
        }
        self.killers[ply][0] = mv;
    }
}
