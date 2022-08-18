// https://projecteuler.net/problem=9
pub fn problem9() {
    // Fibonacci's method for generating Pythagorean triples, isn't exhaustive.
    /*
    fn integersqrt(n: i32) -> i32 {
        if n < 2 {
            return n;
        }
        let small: i32 = integersqrt(n >> 2) << 1;
        let large: i32 = small + 1;
        if large * large > n {
            return small;
        } else {
            return large;
        }
    }
    let mut i = 1;
    let mut n = 1;
    let mut sum = 0;
    loop {
        let sqrt: i32 = integersqrt(i+2);
        sum += i;
        if sqrt*sqrt == i+2 { // we know we have a perfect square
            let c_squared = sum + i + 2;
            let c = integersqrt(c_squared);
            let a = integersqrt(i+2);
            println!("{}", a+n+c);
            if a + n + c == 1000 {
                println!("{}", a*n*c); 
            }
        }
        i += 2;
        n += 1;
    } */
    
    // Euclid's method
    let mut m = 2;
    'outer: loop {
        for n in 1..m-1 {
            let a = m*m - n*n;
            let b = 2*m*n;
            let c = m*m + n*n;
            if a + b + c == 1000 {
                println!("Problem 9: {}", a*b*c);
                break 'outer;
            }
        }
        m += 1;
    }
}