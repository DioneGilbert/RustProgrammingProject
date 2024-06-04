#[test]
fn tests_celsius2farenheit() {
    let celsius: i32 = 40;
    let farenheit: i32 = 104;
    assert_eq!(farenheit, p22::calc::celsius2farenheit(celsius));
}
#[test]
fn tests_farenheit2celsius() {
    let celsius: i32 = 25;
    let farenheit: i32 = 77;
    assert_eq!(celsius, p22::calc::farenheit2celsius(farenheit));
}

#[test]
fn tests_fibonacci_loop() {
    let number: u32 = 8;
    let fib_number: u64 = 21;
    assert_eq!(fib_number, p22::calc::fibonacci_loop(number));
    assert_eq!(13, p22::calc::fibonacci_loop(7));
}

#[test]
fn tests_fibonacci_rec() {
    let number: u32 = 8;
    let fib_number: u64 = 21;
    assert_eq!(fib_number, p22::calc::fibonacci_rec(number));
    assert_eq!(13, p22::calc::fibonacci_loop(7));
}