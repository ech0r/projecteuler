// https://projecteuler.net/problem=17
// Number letter counts

use std::collections::HashMap;

pub fn problem17() {
    fn count_letters_in_number(num: i32) -> i32 {
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
        num_lookup.insert(18, "eighteen".to_string());
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

        let ones = num%10;
        let tens = (num - num%10)%100;
        let hundreds = ((num - num%100)/100)%10;
        let thousands = ((num - num%1000)/1000)%10;
        let mut number = String::from("");
        /*
        println!("Ones: {}", ones);
        println!("Tens: {}", tens);
        println!("Hundreds: {}", hundreds);
        println!("Thousands: {}", thousands);
        */
        if thousands > 0 {
            number.push_str(&num_lookup.get_mut(&thousands).unwrap());
            number.push_str(&num_lookup.get_mut(&1000).unwrap());
        }
        if hundreds > 0 {
            number.push_str(&num_lookup.get_mut(&hundreds).unwrap());
            number.push_str(&num_lookup.get_mut(&100).unwrap());
            if tens > 0 || ones > 0 {
                number.push_str("and");
            }
        }
        if tens > 0 {
            if tens  != 10 {
                number.push_str(&num_lookup.get_mut(&tens).unwrap());
            } else {
                number.push_str(&num_lookup.get_mut(&(tens + ones)).unwrap());
            }
        }
        if ones > 0 {
            if tens != 10 {
                number.push_str(&num_lookup.get_mut(&ones).unwrap());
            }
        }
        //println!("Number: {}", number);
        //println!("i: {}, number of letters: {}", num,number.len());
        //println!("==============================");
        number.len() as i32
    }
    let mut i = 1;
    let mut sum = 0;
    loop {
        sum += count_letters_in_number(i);
        if i == 1000 {
            break;
        }
        i += 1;
    }
    println!("Problem 17: {}", sum);
    //println!("115 {}", count_letters_in_number(115));
    //println!("342 {}", count_letters_in_number(342));
    //println!("1000 {}", count_letters_in_number(1000));
}