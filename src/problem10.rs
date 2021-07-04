// https://projecteuler.net/problem=10
// Sieve of Eratosthenes
pub fn problem10() {
    let n = 2000000;
    const N: usize = 2000001;
    let mut primes = vec![true; N];
    let mut i = 2;
    while  i*i <= n {
        if primes[i] {
            let mut ii = i*2;
            while ii <= n {
                primes[ii] = false;
                ii += i;
            }
        }
        i += 1;
    }
    let mut sum_of_primes = 0;
    for i in 2..primes.len() {
        if primes[i] {
            sum_of_primes += i
        }
    }
    println!("Problem 10: {}", sum_of_primes);
}