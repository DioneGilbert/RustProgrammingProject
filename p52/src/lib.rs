use std::thread;
use std::thread::JoinHandle;
// use std::time::Duration;

pub fn map_sum1(vector: Vec<u32>, f: fn(u32) -> u64, number_of_threads: usize) -> u64 {
    let mut guards: Vec<JoinHandle<u64>> = vec![];

    // The number of threads is equal to the number of slices
    // split into slices
    // Each slice will contain (vector.len() / number_of_threads) elements
    for chunk in vector.chunks(vector.len() / number_of_threads) {
        let chunk = chunk.to_owned();
        let mut chunk1: Vec<u64> = chunk.into_iter().map(f).collect();
        chunk1 = chunk1.to_owned();
        let res: u64 = chunk1.into_iter().sum();
        // Each thread will return the sum of a given vector chunk
        let g = thread::spawn(move || res);
        guards.push(g);
    }

    // collect the results
    let mut result: Vec<u64> = vec![];
    for g in guards {
        result.push(g.join().unwrap());
    }
    // Total sum of the vector consisting of the sum of the chunks
    result.into_iter().sum()
}
