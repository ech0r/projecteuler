// https://projecteuler.net/problem=7
// Find the 10001st prime number
// Sieve of Eratosthenes
pub fn problem7() {
    let n = 2000000;
    const N: usize = 2000001;
    //let mut primes = Box::new([true; N]);
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
    let mut count = 0;
    for i in 2..primes.len() {
        if primes[i] {
            count += 1;
            if count == 10001 {
                 println!("Problem 7: {}", i); 
            }
        }
    }
}