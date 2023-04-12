// https://projecteuler.net/problem=26
// Reciprocal Cycles 

// returns the quotient of a dividend and divisor to unbounded precision.
fn unbounded_division(mut dividend: u32, divisor: u32) -> Vec<u32> {
    let original_dividend = dividend.clone();
    let mut dividend_history: Vec<u32> = vec![];
    let mut quotient: Vec<u32> = vec![];
    loop {
        if dividend == 0 {
            break;
        }
        if dividend == original_dividend && quotient.len() > 0 { // we've come back around to a repeating sequence, we can stop.
            break;
        }
        let mut count = 0;
        while dividend < divisor { // order of magnitude increase so regular division works
            dividend *= 10;
            if count > 0 {
                quotient.push(0);
            }
            count += 1;
        }
        if dividend_history.contains(&dividend) {
            break;
        }
        let quotient_digit = dividend / divisor;
        quotient.push(quotient_digit);
        dividend_history.push(dividend);
        dividend = dividend - (quotient_digit * divisor);
    }
    quotient
}

pub fn problem26() {
    let mut longest = 0;
    let mut d = 0;
    for i in 1..1000 {
        let result = unbounded_division(1 , i);
        if result.len() > longest {
            longest = result.len();
            d = i;
        }
    }
    println!("Problem 26: {:?}", d);
}
