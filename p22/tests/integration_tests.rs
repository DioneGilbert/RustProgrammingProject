#[test]
fn tests_celsius2farenheit2celsius() {
    let celsius: i32 = 40;
    let farenheit: i32 = 104;
    assert_eq!(
        farenheit,
        p22::calc::celsius2farenheit(p22::calc::farenheit2celsius(farenheit))
    );
    assert_eq!(
        celsius,
        p22::calc::farenheit2celsius(p22::calc::celsius2farenheit(celsius))
    );
}

#[test]
fn tests_fibonacci_loop_rec() {
    assert_eq!(34, p22::calc::fibonacci_rec(9));
    assert_eq!(34, p22::calc::fibonacci_loop(9));
    assert_eq!(p22::calc::fibonacci_loop(5), p22::calc::fibonacci_rec(5));
}
