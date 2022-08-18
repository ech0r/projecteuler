// Longest Collatz Sequence
// https://projecteuler.net/problem=14
// Optimized solution :)
pub fn problem14() {
    // function to return next collatz value for any number, n
    fn count_collatz (n: usize, values: &mut Vec<i64>) -> i64 {
        if values[n] > 1 {
            return values[n];
        }
        let mut num = n; // num = n to start
        loop {
            if num ^ 1 == num + 1 {
                values[n] += 1; 
                num = num/2;
            } else {
                values[n] += 2;
                num = (3*num+1)/2;
            }
            if num == 1 {
                break;
            }
            //println!("num: {} values[n]: {}", num, values[n]);
        }
        return values[n];
    }
    let mut collatz_sequence_lengths: Vec<i64> = vec![1; 1000000];
    let mut longest_chain = 0;
    let mut answer = 0;
    let mut i = 500000;
    loop {
        let count = count_collatz(i, &mut collatz_sequence_lengths);
        if count > longest_chain {
            longest_chain = count;
            answer = i;
        }
        if i == 999999 {
            break;
        }
        i += 1;
    }
    println!("Problem 14: {}", answer);
}
 
/*
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
*/