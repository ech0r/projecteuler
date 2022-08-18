// https://projecteuler.net/problem=6
// Find difference between the sum of squares of 1 -> 100 and the squared sum of 1 -> 100
pub fn problem6() {
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