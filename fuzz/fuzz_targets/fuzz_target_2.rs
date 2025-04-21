#![no_main]

use libfuzzer_sys::fuzz_target;
use fuzz::parse_complex;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    let _ = parse_complex(&data);
});
