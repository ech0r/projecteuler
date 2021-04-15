// Multiples of 3 and 5
// https://projecteuler.net/problem=1
fn problem1() {
    let mut number = 0;
    let mut sum = 0;
    while number < 1000 {
        if number % 3 == 0 || number % 5 == 0 {
            sum += number
        }
        number += 1;
    }
    println!("Problem 1: {}", sum);
}

// sum of even fibonacci terms under 4000000
// https://projecteuler.net/problem=2
fn fibonacci(n: i32) -> i32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn problem2() {
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

// greatest prime factor
// https://projecteuler.net/problem=3
fn problem3() {
    let mut p = 2; 
    let mut number: i64 = 600851475143;
    loop {
        if number % p == 0 {
            number = number/p;
        } else {
            p += 1;
        }
        if number < p*p { break; }
    } 
    println!("Problem 3: {}", number)
}

// largest palindrome product of two three digit numbers
// https://projecteuler.net/problem=4
fn problem4() {
    let mut largest_palindrome: i32 = 0;
    for i in 900..1000 {
        for ii in 900..1000 { 
            let prod: i32 = i * ii;
            let prod_str: String = prod.to_string(); 
            let prod_str_rev: String = prod_str.chars().rev().collect();
            if prod_str == prod_str_rev && prod > largest_palindrome { // we have a palindrome
                largest_palindrome = prod;
            }
        }
    }
    println!("Problem 4: {}", largest_palindrome);
}

// smallest multiple
// https://projecteuler.net/problem=5

fn problem5() {
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

// https://projecteuler.net/problem=6
// Find difference between the sum of squares of 1 -> 100 and the squared sum of 1 -> 100
fn problem6() {
    let mut sum_of_squares: i32 = 0;
    let mut sum_to_square: i32 = 0;
    for i in 1..101 {
        let n: i32 = i;
        sum_of_squares += n.pow(2);
        sum_to_square += n;
    }
    let difference: i32 = sum_to_square.pow(2) - sum_of_squares;
    println!("Problem 6: {}", difference);
}

// https://projecteuler.net/problem=7
// Find the 10001st prime number
// Sieve of Eratosthenes
fn problem7() {
    let n = 2000000;
    const N: usize = 2000001;
    let mut primes: [bool; N] = [true; N];
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

// https://projecteuler.net/problem=8
fn problem8() {
    let number = "7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450";
    let num_chars: Vec<char> = number.chars().collect();
    let mut largest:u64 = 0;
    for i in 0..num_chars.len() {
        let j = i + 13;
        let mut product:u64 = 1;
        for x in i..j {
            product *= num_chars[x] as u64 & 15;
        }
        if product > largest {
            largest = product;
        }
        if j == num_chars.len() {
            break;
        }
    }
    println!("Problem 8: {}", largest);
}

// https://projecteuler.net/problem=9
fn problem9() {
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


// https://projecteuler.net/problem=10
// Sieve of Eratosthenes
fn problem10() {
    let n = 2000000;
    const N: usize = 2000001;
    let mut primes: [bool; N] = [true; N];
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

// https://projecteuler.net/problem=11
fn problem11() {
    let grid_str = "08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08 49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00 81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65 52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91 22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80 24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50 32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70 67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21 24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72 21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95 78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92 16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57 86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58 19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40 04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66 88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69 04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36 20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16 20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54 01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48";
    let mut grid: [[i32; 20]; 20] = [[0; 20];20];
    let mut x = 0;
    let mut y = 0;
    // get grid from string into 2D array
    for n in grid_str.split(" ") {
        grid[y][x] = n.parse::<i32>().unwrap();
        if x == 19 {
            y += 1; 
            x = 0;
        } else {
            x += 1;
        }
    }
    let mut largest: i32 = 0;
    // check horizontal and vertical direction
    for y in 0..20 {
        for x in 0..20 {
            if x + 3 > 19 {
                break;
            } 
            let product_h = grid[y][x]*grid[y][x+1]*grid[y][x+2]*grid[y][x+3];
            let product_v = grid[x][y]*grid[x+1][y]*grid[x+2][y]*grid[x+3][y];
            if product_h > largest {
                largest = product_h;
            }
            if product_v > largest {
                largest = product_v;
            }
        }
    }
    // check diagonal
    for x in 3..20 {
        let mut diagonal_for1: Vec<i32> = Vec::new(); 
        let mut diagonal_for2: Vec<i32> = Vec::new();
        let mut diagonal_bak1: Vec<i32> = Vec::new();
        let mut diagonal_bak2: Vec<i32> = Vec::new();
        for y in 0..x+1 {
            diagonal_for1.push(grid[y][x-y]);
            diagonal_for2.push(grid[19-y][19-x+y]);
            diagonal_bak1.push(grid[y][19-x+y]);
            diagonal_bak2.push(grid[19-x+y][y]);
        }
        // get sliding window of 4, pretty cool! :)
        let dfor1 = diagonal_for1.windows(4);
        let dfor2 = diagonal_for2.windows(4);
        let dbak1 = diagonal_bak1.windows(4);
        let dbak2 = diagonal_bak2.windows(4);
        // multiply each segment of 4
        // These multiplications could be done inside the above loop, but I was checking out
        // windows in Rust :)
        for n in dfor1 {
            let product_df1 = n.iter().product::<i32>();           
            if product_df1 > largest {
                largest = product_df1;
            }
        }
        for n in dfor2 {
            let product_df2 = n.iter().product::<i32>();           
            if product_df2 > largest {
                largest = product_df2;
            }
        }
        for n in dbak1 {
            let product_db1 = n.iter().product::<i32>();           
            if product_db1 > largest {
                largest = product_db1;
            }
        }
        for n in dbak2 {
            let product_db2 = n.iter().product::<i32>();           
            if product_db2 > largest {
                largest = product_db2;
            }
        }
    }
    println!("Problem 11: {}", largest);
}

// https://projecteuler.net/problem=12
fn problem12() {
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
    let mut n = 3;
    let mut t = 3; 
    loop {
        t += n;
        n += 1;
        let mut prime_factors: Vec<i32> = Vec::new();
        prime_factors = prime_factorizer(t, 2, prime_factors);
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
        if div_count > 500  {
            println!("Problem 12: {}", t); 
            break;
        }
    }
}

// https://projecteuler.net/problem=13
fn problem13() {

}

fn main() {
    println!("======= PROJECT EULER =======");
    problem1();
    problem2();
    problem3();
    problem4();
    problem5();
    problem6();
    problem7();
    problem8();
    problem9();
    problem10();
    problem11();
    problem12();
}
