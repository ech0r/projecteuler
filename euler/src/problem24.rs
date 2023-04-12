
// https://projecteuler.net/problem=24
// Lexicographic Permutations

fn factorial(n: usize) -> usize {
    (1..=n).product()
}

fn get_digit_and_factorial(remaining: &mut usize, len: usize) -> Option<(usize, usize)> {
    if *remaining == 0 || len == 0 {
        None
    } else {
        let fact = factorial(len - 1);
        let index = (*remaining - 1) / fact;
        *remaining -= index * fact;
        Some((index, index))
    }
}

pub fn problem24() {
    let mut digits = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut target_permutation = 1_000_000;
    let mut result = String::new();

    while let Some((digit, fact)) = get_digit_and_factorial(&mut target_permutation, digits.len()) {
        result.push_str(&digits[digit].to_string());
        digits.remove(fact);
    }

    for d in digits {
        result.push_str(&d.to_string());
    }

    println!("Problem 24: {}", result);
}
