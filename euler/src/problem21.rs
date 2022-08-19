// https://projecteuler.net/problem=21
// Amicable numbers

fn sum_factors(n: u32) -> u32 {
    let mut result = vec![];
    let upper_limit = ((n as f64).sqrt() as u32) + 1;
    for i in 1..upper_limit {
        if n % i == 0 {
            result.push(i);
            result.push(n/i);
        }
    }
    result.sort();
    result.dedup();
    result.retain(|&x| x != n);
    return result.into_iter().sum();
}

fn amicable_pairs(n: u32) -> Vec<(u32, u32)> {
    let mut result = vec![];
    for x in 1..n+1 {
        let y = sum_factors(x);
        if sum_factors(y) == x && x != y {
            if y >= x {
                result.push((x,y));
            } else {
                result.push((y,x));
            }
        }
    }
    result.dedup();
    result
}

fn sum_amicable_pairs(vec: Vec<(u32, u32)>) -> u32 {
    let mut sum = 0;
    for x in vec {
        sum += x.0 + x.1;
    }
    sum
}

pub fn problem21() {
    println!("Problem 21: {}", sum_amicable_pairs(amicable_pairs(10000)));
}