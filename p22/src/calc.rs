// Conversion from celsius to  farenheit:
/// Given a value in celsius, return the corresponding value in farenheit
///
/// # Example
///
/// ```
/// assert_eq!(104, p22::calc::celsius2farenheit(40));
/// ```

pub fn celsius2farenheit(celsius: i32) -> i32 {
    (celsius * 9 / 5) + 32
}

// Conversion from farenheit to celsius:
/// Given a value in farenheit, return the corresponding value in celsius
///
/// # Example
///
/// ```
/// assert_eq!(25, p22::calc::farenheit2celsius(77));
/// ```

pub fn farenheit2celsius(farenheit: i32) -> i32 {
    let celsius: i32 = (farenheit - 32) * 5 / 9;
    celsius
}

// Compute fibonnacci of a given integer:
///
/// # Example
///
/// ```
/// assert_eq!(21, p22::calc::fibonacci_loop(8));
/// ```
pub fn fibonacci_loop(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let mut fibonacci_number: u64 = 1;
    let mut f0: u64 = 0;
    let mut f1: u64 = 1;
    let mut mycounter: u32 = 1;
    while mycounter < n {
        fibonacci_number = f0 + f1;
        f0 = f1;
        f1 = fibonacci_number;
        mycounter += 1;
    }
    fibonacci_number
}

// Compute fibonnacci of a given integer recursively:
///
/// # Example
///
/// ```
/// assert_eq!(21, p22::calc::fibonacci_rec(8));
/// ```
pub fn fibonacci_rec(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_rec(n - 2) + fibonacci_rec(n - 1),
    }
}

#[cfg(test)]

mod tests {
    use super::{celsius2farenheit, farenheit2celsius, fibonacci_loop, fibonacci_rec};
    #[test]
    fn tests_celsius2farenheit() {
        let celsius: i32 = 40;
        let farenheit: i32 = 104;
        assert_eq!(farenheit, celsius2farenheit(celsius));
    }
    #[test]
    fn tests_farenheit2celsius() {
        let celsius: i32 = 25;
        let farenheit: i32 = 77;
        assert_eq!(celsius, farenheit2celsius(farenheit));
    }

    #[test]
    fn tests_fibonacci_loop() {
        let number: u32 = 8;
        let fib_number: u64 = 21;
        assert_eq!(fib_number, fibonacci_loop(number));
        assert_eq!(13, fibonacci_loop(7));
    }

    #[test]
    fn tests_fibonacci_rec() {
        let number: u32 = 8;
        let fib_number: u64 = 21;
        assert_eq!(fib_number, fibonacci_rec(number));
        assert_eq!(13, fibonacci_loop(7));
    }
}
