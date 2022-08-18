// largest palindrome product of two three digit numbers
// https://projecteuler.net/problem=4
pub fn problem4() {
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