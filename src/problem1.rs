// Multiples of 3 and 5
// https://projecteuler.net/problem=1
pub fn problem1() {
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