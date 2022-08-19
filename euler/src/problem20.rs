// https://projecteuler.net/problem=20
// Factorial digit sum

#[derive(Clone)]
struct BigInt {
    value: Vec<u8>,
}

impl BigInt {
    fn new(initial_value: u32) -> Self {
        // convert initial value to vector of single digits
        let digits: Vec<u8> = initial_value.to_string().chars().map(|d| d.to_digit(10).unwrap() as u8).collect();
        BigInt {
            value: digits,
        }
    }

    fn add(&self, other: &Self) -> Self {
        let mut num1 = self.value.iter().rev();
        let mut num2 = other.value.iter().rev();
        let mut sum: Vec<u8> = vec![];
        let mut carry = 0u8;
        loop {
            let s;
            match(num1.next(), num2.next()) {
                (Some(x), Some(y)) => {
                    s = x + y + carry;
                    carry = (x + y + carry)/10 % 10; 
                },
                (Some(x), None) => {
                    s = x + carry;
                    carry = (x + carry)/10 % 10;
                },
                (None, Some(y)) => {
                    s = y + carry;
                    carry = (y + carry)/10 % 10;
                },
                (None, None) => {
                    if carry > 0 {
                        sum.push(carry);
                    }
                    break;
                },
            }
            if carry > 0 {
                sum.push(s % 10);
            } else {
                sum.push(s);
            }
        }
        sum.reverse();
        BigInt {
            value: sum
        }
    }

    fn multiply(&self, other: u32) -> Self {
        let mut current_product: Self = BigInt::new(0);
        for _x in 0..other {
            current_product = current_product.add(self);
        }
        current_product
    }

    fn factorial(factorial: u32) -> Self {
        let mut current_product = Self::new(factorial);
        for x in (1..factorial).rev() {
            current_product = current_product.multiply(x);
        }
        current_product
    }

    fn sum_digits(&self) -> u32 {
        let sum: u32 = self.value.iter().map(|n| *n as u32).sum();
        sum
    }
}

pub fn problem20() {
    println!("Problem 20: {}", BigInt::factorial(100).sum_digits());
}