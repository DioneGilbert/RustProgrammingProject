#![no_main]
// #[macro_use] extern crate libfuzzer_sys;

use fuzz::parse_complex;

use libfuzzer_sys::fuzz_target;

// extern crate example_rust;
// pub fn test_fuzz(:)

// fuzz_target!(|data: &[u8]| {
//     // fuzzed code goes here
// });


fuzz_target!(|data: &[u8]| {
    // let _ = example_rust::parse_complex(&data);
    let _ = parse_complex(&data);
});
