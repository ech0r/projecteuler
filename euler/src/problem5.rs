// smallest multiple
// https://projecteuler.net/problem=5
pub fn problem5() {
    // function to return a sorted vector of prime factors of any i32
    fn prime_factorizer(mut n: i32, mut p: i32, mut factors: Vec<i32>) -> Vec<i32> {
        if n < 2 {
            return factors;
        }
        if n % p == 0 {
            n = n/p;
            factors.push(p);
            return prime_factorizer(n, p, factors);
        } else {
            p += 1;
            return prime_factorizer(n, p, factors);
        }
    }
    // accumulate all prime factors of numbers from 1 -> n
    let mut total_primes: Vec<i32> = Vec::new();
    for i in 1..21 {
        let mut prime_factors: Vec<i32> = Vec::new(); 
        prime_factors = prime_factorizer(i, 2, prime_factors);
        total_primes.append(&mut prime_factors);
    }
    let mut counts: Vec<(i32,u32)> = Vec::new();
    // counts highest occurence of each prime factor of each number n in the series.
    for i in 0..total_primes.len() {
        let mut largest = 1;
        let mut count = 0;
        for ii in 0..total_primes.len() {
            if total_primes[ii] == total_primes[i] {
                count += 1;
                if count > largest {
                    largest = count;
                }
            } else {
                count = 0;
            }
        }
        counts.push((total_primes[i], largest));
    }
    counts.sort();
    counts.dedup();
    // calculates the LCM of the series.
    let mut lcm = 1;
    for i in 0..counts.len() {
        lcm *= counts[i].0.pow(counts[i].1);
    } 
    println!("Problem 5: {}", lcm);
}