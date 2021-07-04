// greatest prime factor
// https://projecteuler.net/problem=3
pub fn problem3() {
    let mut p = 2; 
    let mut number: i64 = 600851475143;
    loop {
        if number % p == 0 {
            number = number/p;
        } else {
            p += 1;
        }
        if number < p*p { break; }
    } 
    println!("Problem 3: {}", number)
}