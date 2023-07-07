use chess::{ChessMove, Square};

pub const FAILED_VALUE: i32 = i32::MIN;
const KEY: u64 = 65535;
const PAWN_KEY: u64 = 4095;
pub enum EntryType {
    Exact = 0,
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
pub struct TranspositionTable {
    table: Vec<PositionEntry>,
    moves: String,
}
impl TranspositionTable {
    #[inline]
    pub fn init() -> TranspositionTable {
        let mut x = TranspositionTable {
            table: Vec::<PositionEntry>::with_capacity(65536),
            moves: String::from(""),
        };
        for i in 0..65536 {
            x.table.push(PositionEntry {
                key: i + 1,
                eval: 0,
                entry_type: EntryType::Exact,
                depth: 0,
                best_move: ChessMove::default(),
            });
        }
        return x;
    }
    pub fn add_move(&mut self, move_str: String) {
        self.moves.push(' ');
        for i in move_str.chars() {
            self.moves.push(i);
        }
    }
    pub fn get_moves(&self) -> &String {
        return &self.moves;
    }
    pub fn get_pos(&self, i: usize) -> Option<&PositionEntry> {
        return self.table.get(i);
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
        self.table.clear();
    }
}
