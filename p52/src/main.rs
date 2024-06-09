// use std::thread;
// use std::time::Duration;

use p52::map_sum1;

fn main() {
    let myvector: Vec<u32> = vec![2, 3, 4, 5, 6, 5, 15];
    let number_of_threads: usize = 4;

    let f = |x| x as u64;
    let res: u64 = map_sum1(myvector, f, number_of_threads);
    println!("--Sum= {:?}", res)
}
