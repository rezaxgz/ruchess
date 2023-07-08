use std::time::{Duration, Instant};
#[allow(dead_code)]
pub fn benchmark(n: usize) -> Duration {
    let start = Instant::now();
    for _ in 0..n {
        //do sth
    }
    let duration = start.elapsed();
    println!("{:?}", duration);
    return duration;
}
