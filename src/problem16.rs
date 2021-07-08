// https://projecteuler.net/problem=16
// Power digit sum
// What is the sum of the digits of the number 2^1000?

pub fn problem16() {
    // doubles a an arbitrarily large number in the form of a String
    fn big_double(num: String) -> String {
        let mut num_vec: Vec<char> = num.chars().collect();
        num_vec.reverse();
        //println!("{:?}", num_vec);
        let mut carry = 0;
        let mut answer: String = "".to_owned();
        for n in num_vec {
            let digit = n.to_digit(10).unwrap();
            let sum = digit + digit + carry;
            answer.push_str(&(sum % 10).to_string());
            carry = (sum - (sum % 10))/10;
        }
        if carry > 0 {
            answer = format!("{}{}", answer, carry.to_string());
        }
        answer = answer.chars().rev().collect();
        return answer;
    }
    // return an "arbitrarily large" power of 2, limited to 2^(2^31)
    fn big_power_of_two(exp: i32) -> String {
        let mut i = 1;
        let mut answer: String = "1".to_owned();
        //answer = big_double(answer);
        loop {
            answer = big_double(answer);
            //println!("{}", answer);
            if i == exp {
                break;
            }
            i += 1;
        }
        return answer;
    }
    // sum the digits of a large numeric string, in this case 2^1000
    let nums: Vec<char> = big_power_of_two(1000).chars().collect();
    let mut sum: u32 = 0;
    for n in nums {
        sum += n.to_digit(10).unwrap();
    }
    println!("Problem 16: {}", sum);
}