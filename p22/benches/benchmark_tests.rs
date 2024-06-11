#![feature(test)]
extern crate test;
use p22::calc::{fibonacci_loop, fibonacci_rec};
use std::hint::black_box;
use test::Bencher;
#[bench]
fn benchmark_fibonacci_loop(b: &mut Bencher) {
    b.iter(|| black_box(fibonacci_loop(6)));
}
#[bench]
fn benchmark_fibonacci_loop2(b: &mut Bencher) {
    b.iter(|| {
        for i in 0..10 {
            black_box(fibonacci_loop(i));
        }
    });
}
#[bench]
fn benchmark_fibonacci_rec(b: &mut Bencher) {
    b.iter(|| black_box(fibonacci_rec(6)));
}
#[bench]
fn benchmark_fibonacci_rec2(b: &mut Bencher) {
    b.iter(|| {
        for i in 0..10 {
            black_box(fibonacci_rec(i));
        }
    });
}
