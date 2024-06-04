#![feature(test)]

extern crate test;

use p14::add;

use test::{black_box, Bencher};

#[bench]

fn bench_add(b: &mut Bencher) {
    let x = black_box(1);
    let y = black_box(2);
    let z = add(x, y);
    black_box(z);
}
