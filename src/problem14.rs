// Longest Collatz Sequence
// https://projecteuler.net/problem=14
// lame brute force solution - need to do some caching
pub fn problem14() {
    fn collatz (n: i64) -> i64 {
        return if n ^ 1 == n + 1 { n/2 } else { 3*n + 1 }        
    }
    let mut i = 1;
    let mut greatest = 0;
    let mut answer = 0;
    loop {
        let mut count = 1;
        let mut ii = i;
        loop {
            ii = collatz(ii);
            count += 1;
            if ii == 1 {
                break;
            }
        }
        if i == 1000000 {
            println!("Problem 14: {}", answer);
            break;
        }
        if count > greatest {
            greatest = count;
            answer = i;
        }
        i += 1;
    }
}