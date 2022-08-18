// https://projecteuler.net/problem=15
// Lattice paths

pub fn problem15 () {
    // 40!/(20!*20!) or 40 choose 20 - this is a combinatorics problem!
    let mut result: i64 = 1;
    let mut i = 1;
    let n = 20;
    loop {
        result = result * (n + i)/i;
        if i == n {
            break;
        }
        i += 1;
    }
    println!("Problem 15: {}", result);
}