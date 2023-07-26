use chess::{Board, ChessMove};

const NUM_OF_POSITIONS: usize = 0x400000;
const KEY: u64 = NUM_OF_POSITIONS as u64 - 1;

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
    // pawns: Vec<PawnEntry>,
}
impl TranspositionTable {
    #[inline]
    pub fn init() -> TranspositionTable {
        let mut x = TranspositionTable {
            table: Vec::<PositionEntry>::with_capacity(NUM_OF_POSITIONS),
            // pawns: Vec::<PawnEntry>::with_capacity(NUM_OF_PAWNENTRIES),
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
        // for i in 0..NUM_OF_PAWNENTRIES {
        //     x.pawns.push(PawnEntry {
        //         key: i as u64 + 1,
        //         eval: 0,
        //         closed: 0,
        //         semi_open_w: 0,
        //         semi_open_b: 0,
        //         open: 0,
        //         wpassers: 0,
        //         bpassers: 0,
        //     });
        // }
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
        //TODO: replacement strategy
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
        // self.pawns.clear();
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
}
