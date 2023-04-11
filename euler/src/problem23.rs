// https://projecteuler.net/problem=23
// Non-abundant sums


fn get_proper_divisors(n: u32) -> Vec<u32> {
    let mut divisors: Vec<u32> = vec![1];
    let sqrt = (n as f64).sqrt() as u32;
    for x in 2..=sqrt {
        if n % x == 0 {
            divisors.push(x);
            if x != n / x  {
                divisors.push(n / x);
            }
        }
    }
    divisors.sort();
    divisors
}

fn is_abundant(n: u32) -> bool {
    if n < 12 {
        false 
    } else {
        get_proper_divisors(n).iter().sum::<u32>() > n
    }
}

pub fn problem23() {
    let all_nums:Vec<u32> = (1..=28123).collect();
    let abundant_nums: Vec<&u32> = all_nums.iter().filter(|x| is_abundant(**x)).collect();
    let mut abundant_sums: Vec<bool> = vec![false; 28124];
    for (_i, v) in abundant_nums.iter().enumerate() {
        for (_j, x) in abundant_nums.iter().enumerate() {
            let s = **v + **x;
            if s <= 28123 { 
                abundant_sums[s as usize] = true;
            } else {
                break;
            }
        }
    }
    let answer: u32 = all_nums.iter().filter(|x| !abundant_sums[**x as usize]).sum();
    println!("Problem 23: {:?}", answer);
}
