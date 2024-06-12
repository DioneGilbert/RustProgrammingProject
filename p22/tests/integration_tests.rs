use p22::calc::{celsius2farenheit, farenheit2celsius, fibonacci_loop, fibonacci_rec};

#[test]
fn tests_celsius2farenheit2celsius() {
    let celsius: i32 = 40;
    let farenheit: i32 = 104;
    assert_eq!(farenheit, celsius2farenheit(farenheit2celsius(farenheit)));
    assert_eq!(celsius, farenheit2celsius(celsius2farenheit(celsius)));
}

#[test]
fn tests_fibonacci_loop_rec() {
    assert_eq!(34, fibonacci_rec(9));
    assert_eq!(34, fibonacci_loop(9));
    assert_eq!(fibonacci_loop(5), fibonacci_rec(5));
}
