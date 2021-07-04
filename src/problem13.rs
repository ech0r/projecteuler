
// https://projecteuler.net/problem=13
use std::fs;

pub fn problem13() {
    // assumes UTF-8 encoding
    fn char_to_digit (n: char) -> i32 {
        return n as i32 ^ 0x30;
    }
    fn digit_to_char (n: u8) -> char {
        return (n | 0x30) as char;
    }
    // Parse input numbers into a vector of strings
    let numbers: Vec<String> = fs::read_to_string("./inputs/problem13.txt")
        .expect("Failed to read input")
        .split("\n")
        .map(|s| s.to_string())         // Convert &str to String
        .collect();
    // Add up all the numbers starting with the 1's place
    let mut i = 49;
   
    let mut carry = 0;
    let mut answer: Vec<char> = vec![];
    loop {
        let mut sum = 0;
        sum += carry;
        for number in numbers.iter() {
            let digit_char: char = number.chars().nth(i).unwrap();
            let digit: i32 = char_to_digit(digit_char);
            sum += digit;
            //println!("{}", digit);
        }
        if i == 0 {
            answer.push(digit_to_char((sum%10) as u8));
            answer.push(digit_to_char((((sum - sum%10)/10)%10) as u8));
            answer.push(digit_to_char(((((sum - sum%10)/10) - ((sum - sum%10)/10)%10)/10) as u8));
            break;
        } 
        answer.push(digit_to_char((sum%10) as u8));
        carry = (sum - sum%10)/10;
        i -= 1;
    }
    answer.reverse();
    let first_ten: String = answer[0..9].into_iter().collect();
    println!("Problem 13: {}", first_ten);
}