use chess::{Board, ChessMove};

const NUM_OF_POSITIONS: usize = 0x400000;
const KEY: u64 = NUM_OF_POSITIONS as u64 - 1;
const KILLERS_PER_PLY: usize = 3;
const KILLER_PLIES: usize = 20;
pub type Killers = [ChessMove; KILLERS_PER_PLY];
// const NUM_OF_PAWNENTRIES: usize = 8192;
// const PAWN_KEY: u64 = NUM_OF_PAWNENTRIES as u64 - 1;
#[derive(PartialEq)]
pub enum EntryType {
    Exact,
    LowerBound,
    UpperBound,
}
pub struct PositionEntry {
    pub key: u64,
    pub eval: i16,
    pub entry_type: EntryType,
    pub depth: u8,
    ply: u8,
    pub best_move: ChessMove,
    age: u16,
}
// pub struct PawnEntry {
//     pub key: u64,
//     pub eval: i16,
//     pub closed: u8,
//     pub semi_open_w: u8,
//     pub semi_open_b: u8,
//     pub open: u8,
//     pub wpassers: u64,
//     pub bpassers: u64,
// }
pub struct TranspositionTable {
    table: Vec<PositionEntry>,
    killers: [Killers; KILLER_PLIES],
    pub default_killers: Killers,
    // pawns: Vec<PawnEntry>,
}
impl TranspositionTable {
    #[inline]
    pub fn init() -> TranspositionTable {
        let mut x = TranspositionTable {
            table: Vec::<PositionEntry>::with_capacity(NUM_OF_POSITIONS),
            killers: [[ChessMove::default(); KILLERS_PER_PLY]; KILLER_PLIES],
            default_killers: [ChessMove::default(); KILLERS_PER_PLY],
        };
        for i in 0..NUM_OF_POSITIONS {
            x.table.push(PositionEntry {
                key: i as u64 + 1,
                eval: 0,
                entry_type: EntryType::Exact,
                depth: 0,
                ply: 0,
                best_move: ChessMove::default(),
                age: 0,
            });
        }
        return x;
    }
    pub fn look_up_pos(&self, key: u64) -> Option<&PositionEntry> {
        let res = self.table.get((key & KEY) as usize);
        if res.unwrap().key != key {
            return None;
        }
        return res;
    }
    pub fn set_pos(
        &mut self,
        key: u64,
        eval: i16,
        entry_type: EntryType,
        depth: u8,
        ply: u8,
        best_move: ChessMove,
        age: u16,
    ) {
        let prev_entry = self.table.get((key & KEY) as usize).unwrap();
        if prev_entry.key == (key & KEY) + 1 {
            self.table[(key & KEY) as usize] = PositionEntry {
                key,
                eval,
                entry_type,
                depth,
                ply,
                best_move,
                age,
            };
            return;
        }
        let value = (depth * ply) as u16 + age;
        let prev_value = (prev_entry.depth * prev_entry.ply) as u16 + prev_entry.age;
        if value >= prev_value {
            self.table[(key & KEY) as usize] = PositionEntry {
                key,
                eval,
                entry_type,
                depth,
                ply,
                best_move,
                age,
            };
        }
    }
    pub fn clear(&mut self) {
        for i in 0..NUM_OF_POSITIONS {
            self.table[i] = PositionEntry {
                key: i as u64 + 1,
                eval: 0,
                entry_type: EntryType::Exact,
                depth: 0,
                ply: 0,
                best_move: ChessMove::default(),
                age: 0,
            };
        }
        self.killers = [[ChessMove::default(); KILLERS_PER_PLY]; KILLER_PLIES];
    }
    pub fn get_pv(&self, board: &Board) -> Vec<ChessMove> {
        let mut pv = Vec::<ChessMove>::new();
        let mut hash = board.get_hash();
        let mut b = *board;
        loop {
            let res = self.look_up_pos(hash);
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
    pub fn store_killer(&mut self, ply: usize, mv: ChessMove) {
        if ply >= KILLER_PLIES || self.killers[ply].contains(&mv) {
            return;
        }
        for i in 1..KILLERS_PER_PLY {
            self.killers[ply][i] = self.killers[ply][i - 1];
        }
        self.killers[ply][0] = mv;
    }
}
