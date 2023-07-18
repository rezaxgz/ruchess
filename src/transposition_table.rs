use chess::ChessMove;

const NUM_OF_POSITIONS: usize = 0x400000;
const KEY: u64 = NUM_OF_POSITIONS as u64 - 1;

// const NUM_OF_PAWNENTRIES: usize = 8192;
// const PAWN_KEY: u64 = NUM_OF_PAWNENTRIES as u64 - 1;
pub enum EntryType {
    Exact = 0,
    #[allow(dead_code)]
    LowerBound = 1,
    UpperBound = 2,
}
pub struct PositionEntry {
    pub key: u64,
    pub eval: i16,
    pub entry_type: EntryType,
    pub depth: u8,
    pub best_move: ChessMove,
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
                best_move: ChessMove::default(),
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
        let i = key & KEY;
        let res = self.table.get(i as usize);
        if res.is_some() {
            if res.unwrap().key != key {
                return None;
            }
        }
        return res;
    }
    pub fn set_pos(
        &mut self,
        key: u64,
        eval: i16,
        entry_type: EntryType,
        depth: u8,
        best_move: ChessMove,
    ) {
        //TODO: replacement strategy
        self.table[(key & KEY) as usize] = PositionEntry {
            key,
            eval,
            entry_type,
            depth,
            best_move,
        }
    }
    pub fn clear(&mut self) {
        for i in 0..NUM_OF_POSITIONS {
            self.table[i] = PositionEntry {
                key: i as u64 + 1,
                eval: 0,
                entry_type: EntryType::Exact,
                depth: 0,
                best_move: ChessMove::default(),
            };
        }
        // self.pawns.clear();
    }

    // pub fn look_up_pawns(&self, key: u64) -> Option<&PawnEntry> {
    //     let i = key & PAWN_KEY;
    //     let res = self.pawns.get(i as usize);
    //     if res.is_some() && res.unwrap().key != key {
    //         return None;
    //     }
    //     return res;
    // }
    // pub fn set_pawns(&mut self, key: u64, entry: PawnEntry) {
    //     //TODO: replacement strategy
    //     self.pawns[(key & PAWN_KEY) as usize] = entry
    // }
}
