// https://projecteuler.net/problem=25
// 1000 Digit Fibonacci Number

fn fibonacci_index(digits: usize) -> usize {
    let mut f1: Vec<u32> = vec![1];
    let mut f2: Vec<u32> = vec![1];
    let mut index = 3;

    if digits == 1 {
        return 1;
    }

    loop {
        let f3 = add(&f1, &f2);
        if f3.len() >= digits {
            return index;
        }
        f1 = f2;
        f2 = f3;
        index += 1;
    }
}

fn add(a: &[u32], b: &[u32]) -> Vec<u32> {
    let mut result: Vec<u32> = vec![];
    let mut carry = 0;

    for (i, j) in a.iter().rev().zip(b.iter().rev()) {
        let sum = i + j + carry;
        result.push(sum % 10);
        carry = sum / 10;
    }

    for i in a.len()..b.len() {
        let sum = b[b.len() - i - 1] + carry;
        result.push(sum % 10);
        carry = sum / 10;
    }

    if carry > 0 {
        result.push(carry);
    }

    result.reverse();
    result
}

pub fn problem25() {
    
    let digits = 1000;
    let index = fibonacci_index(digits);
    println!("Problem 25: {:?}", index) ;
}
