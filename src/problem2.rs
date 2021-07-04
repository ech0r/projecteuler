// sum of even fibonacci terms under 4000000
// https://projecteuler.net/problem=2
pub fn problem2() {
    fn fibonacci(n: i32) -> i32 {
        match n {
            0 => 1,
            1 => 1,
            _ => fibonacci(n - 1) + fibonacci(n - 2)
        }
    }
    let mut n = 0;
    let mut fib = fibonacci(n);
    let mut sum = 0;
    while fib < 4000000 {
        if fib % 2 == 0 {
            sum += fib;
        }
        n += 1;
        fib = fibonacci(n);
    }
    println!("Problem 2: {}", sum);
}