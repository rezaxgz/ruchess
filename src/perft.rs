use chess::{Board, MoveGen};
use std::str::FromStr;
use std::time::{Duration, Instant};
const POSITIONS: [&str; 5] = [
    "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
    "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",
    "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
    "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
    "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
];
const RESULTS: [[usize; 6]; 5] = [
    [48, 2039, 97862, 4085603, 193690690, 8031647685],
    [14, 191, 2812, 43238, 674624, 11030083],
    [6, 264, 9467, 422333, 15833292, 706045033],
    [44, 1486, 62379, 2103487, 89941194, 3048196529],
    [46, 2079, 89890, 3894594, 164075551, 6923051137],
];
const KEY: u64 = 131071;
struct Entry {
    key: u64,
    depth: usize,
    nodes: usize,
}
struct PerftTable {
    table: Vec<Entry>,
    depth: usize,
}
impl PerftTable {
    #[inline]
    pub fn init(depth: usize) -> PerftTable {
        let mut x = PerftTable {
            table: Vec::<Entry>::with_capacity(131072),
            depth,
        };
        for i in 0..131072 {
            x.table.push(Entry {
                key: i + 1,
                depth: 0,
                nodes: 0,
            });
        }
        return x;
    }
    pub fn look_up(&self, key: u64, depth: usize) -> Option<&Entry> {
        let res = self.table.get((key & KEY) as usize);
        if res.is_some() {
            let data = res.unwrap();
            if data.key != key || data.depth != depth {
                return None;
            }
        }
        return res;
    }
    pub fn set(&mut self, key: u64, nodes: usize, depth: usize) {
        let index = (key & KEY) as usize;
        let prev = self.table.get(index).unwrap();
        if prev.nodes == 0 {
            self.table[(key & KEY) as usize] = Entry { key, nodes, depth };
            return;
        }
        if nodes < 500 {
            return;
        }
        let value = (self.depth - depth) * depth;
        let prev_value = prev.depth * (self.depth - prev.depth);
        self.table[(key & KEY) as usize] = Entry { key, nodes, depth };
    }
}
fn perft(board: Board, depth: usize, tt: &mut PerftTable) -> usize {
    if depth == 1 {
        return MoveGen::new_legal(&board).len();
    }
    let key = board.get_hash();
    let tt_res = tt.look_up(key, depth);
    if tt_res.is_some() {
        return tt_res.unwrap().nodes;
    }
    let iterable = MoveGen::new_legal(&board);
    let mut res: usize = 0;
    for m in iterable {
        res += perft(board.make_move_new(m), depth - 1, tt);
    }
    tt.set(key, res, depth);
    return res;
}
pub fn go_perft(board: &Board, depth: usize) -> usize {
    if depth == 0 {
        println!("0 in 1.0ms");
        return 0;
    }
    let mut tt = PerftTable::init(depth);
    let iterable = MoveGen::new_legal(board);
    let mut res: usize = 0;
    for m in iterable {
        if depth == 1 {
            println!("{}: {}", m.to_string(), 1);
            res += 1;
        } else {
            let n = perft(board.make_move_new(m), depth - 1, &mut tt);
            res += n;
            println!("{}: {}", m.to_string(), n);
        }
    }
    return res;
}
pub fn default_perft(pos: usize, depth: usize) -> usize {
    let board = Board::from_str(POSITIONS[pos]).unwrap();
    let start = Instant::now();
    let res = go_perft(&board, depth);
    let duration = start.elapsed();
    println!(
        "{} in {:?}, match: {}",
        res,
        duration,
        res == RESULTS[pos][depth - 1]
    );
    return res;
}
