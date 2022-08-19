// https://projecteuler.net/problem=22
// Names scores


use std::fs;


fn alphabet_score(c: char) -> u32 {
    match c {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        'D' => 4,
        'E' => 5,
        'F' => 6,
        'G' => 7,
        'H' => 8,
        'I' => 9,
        'J' => 10,
        'K' => 11,
        'L' => 12,
        'M' => 13,
        'N' => 14, 
        'O' => 15,
        'P' => 16,
        'Q' => 17,
        'R' => 18,
        'S' => 19,
        'T' => 20,
        'U' => 21,
        'V' => 22,
        'W' => 23,
        'X' => 24,
        'Y' => 25,
        'Z' => 26,
        _ => 0,
    }
}

pub fn problem22() {
    let names = fs::read_to_string("./euler/inputs/names.txt").expect("failed to parse names.txt for problem 22.");
    let mut names_vec: Vec<Vec<char>> = names.split(",").map(|s| s.to_owned().replace("\"","").chars().collect()).collect();
    names_vec.sort();
    let mut sum = 0u32;
    for (i, name) in names_vec.into_iter().enumerate() {
        let mut name_score = 0;
        for c in name {
            name_score += alphabet_score(c);
        }
        sum += name_score*(i as u32 + 1);
    }
    println!("Problem 22: {}", sum);
}