// https://projecteuler.net/problem=18
// Maximum path sum I

use std::fs;

// function to return the greater of two inputs
fn max(x: i32, y: i32) -> i32 {
    if x >= y { return x; } y
}

pub fn problem18() {
    // Build data structure to hold triangle
    let data: Vec<String> = fs::read_to_string("./inputs/problem18.txt")
    .expect("Failed to read input")
    .split("\n")
    .map(|s| s.to_string())         // Convert &str to String
    .collect();
    let mut numbers: Vec<Vec<i32>> = vec![];
    for line in data {
        numbers.push(line.split(" ").map(|n| n.parse::<i32>().unwrap()).collect());
    }
    // find max path sum through triangle -> we're going bottom up, 'retracing' our steps
    let start = numbers.len() - 2;
    let mut i = start;
    loop {
        let mut ii = 0;
        loop {
            numbers[i][ii] += max(numbers[i+1][ii], numbers[i+1][ii+1]);
            if ii == i {
                break;
            }
            ii += 1;
        }
        if i == 0 {
            break;
        }
        i -= 1;
    }
    println!("Problem 18: {}", numbers[0][0]);
}
