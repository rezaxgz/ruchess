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
fn perft(board: Board, depth: usize) -> usize {
    let iterable = MoveGen::new_legal(&board);
    if depth == 1 {
        return iterable.len();
    }
    let mut res: usize = 0;
    for m in iterable {
        res += perft(board.make_move_new(m), depth - 1);
    }
    return res;
}
pub fn go_perft(board: &Board, depth: usize) -> usize {
    if depth == 0 {
        println!("0");
        return 0;
    }
    let iterable = MoveGen::new_legal(board);
    let mut res: usize = 0;
    for m in iterable {
        if depth == 1 {
            println!("{}: {}", m.to_string(), 1);
            res += 1;
        } else {
            let n = perft(board.make_move_new(m), depth - 1);
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
    println!("{} in {:?}", res, duration);
    return res;
}
