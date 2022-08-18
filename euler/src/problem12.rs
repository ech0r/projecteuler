// https://projecteuler.net/problem=12
pub fn problem12() {
    fn prime_factorizer(mut n: i32, mut p: i32, factors: &mut Vec<i32>) {
        // println!("{:p}", &n); print memory address to calculate stack frame size
        // I was going to do this recursively but Rust doesn't handle deep recursion well. I kept running into a stack overflow error.
        /* 
        if n < 2 {
            return;
        }
        loop {
        */
        while n > 2 {
            if n % p == 0 {
                n = n/p;
                factors.push(p);
                //return prime_factorizer(n, p, factors);
            } else {
                p += 1;
                //return prime_factorizer(n, p, factors);
            }
        }
    }
    let mut n = 1;
    let mut triangle = 1;
    loop {
        n += 1;
        triangle += n;
        let mut prime_factors: Vec<i32> = vec![];
        prime_factorizer(triangle, 2, &mut prime_factors);
        let mut unique_factors = prime_factors.clone();
        unique_factors.dedup();
        let mut div_count = 1;
        for x in unique_factors.iter() {
            let mut freq = 0;
            for y in prime_factors.iter() {
                if x == y {
                    freq += 1;
                }
            }
            div_count *= freq + 1;
        }
        if div_count > 500 {
            println!("Problem 12: {}", triangle);
            break;
        }
    }
}