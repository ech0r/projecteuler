// https://projecteuler.net/problem=17
// Number letter counts

use std::collections::HashMap;

pub fn problem17() {
    let mut num_lookup: HashMap<i32, String> = HashMap::new();
    num_lookup.insert(0, "one".to_string());
    num_lookup.insert(1, "one".to_string());
    num_lookup.insert(2, "two".to_string());
    num_lookup.insert(3, "three".to_string());
    num_lookup.insert(4, "four".to_string());
    num_lookup.insert(5, "five".to_string());
    num_lookup.insert(6, "six".to_string());
    num_lookup.insert(7, "seven".to_string());
    num_lookup.insert(8, "eight".to_string());
    num_lookup.insert(9, "nine".to_string());
    num_lookup.insert(10, "ten".to_string());
    num_lookup.insert(11, "eleven".to_string());
    num_lookup.insert(12, "twelve".to_string());
    num_lookup.insert(13, "thirteen".to_string());
    num_lookup.insert(14, "fourteen".to_string());
    num_lookup.insert(15, "fifteen".to_string());
    num_lookup.insert(16, "sixteen".to_string());
    num_lookup.insert(17, "seventeen".to_string());
    num_lookup.insert(18, "eightteen".to_string());
    num_lookup.insert(19, "nineteen".to_string());
    num_lookup.insert(20, "twenty".to_string());
    num_lookup.insert(30, "thirty".to_string());
    num_lookup.insert(40, "forty".to_string());
    num_lookup.insert(50, "fifty".to_string());
    num_lookup.insert(60, "sixty".to_string());
    num_lookup.insert(70, "seventy".to_string());
    num_lookup.insert(80, "eighty".to_string());
    num_lookup.insert(90, "ninety".to_string());
    num_lookup.insert(100, "hundred".to_string());
    num_lookup.insert(1000, "thousand".to_string());

    fn count_letters_in_number(num: i32) -> i32 {
        let mut ones = num%10;
        let mut tens = ((num - num%10)/10)%10;
        let mut hundreds = ((num - num%100)/100)%10;
        let mut thousands = ((num - num%1000)/1000)%10;
        
        if thousands > 0 {

        }
        if hundreds > 0 {

        }
        if tens > 0 {

        }
        if ones > 0 {

        }
        number.push_strnum_lookup.get_mut()
    }
    count_letters_in_number(4576);
}