// https://projecteuler.net/problem=19
// Counting Sundays

// function returns true if leap year false if not. year -> YYY e.g. 1989
fn is_leap_year(year: i32) -> bool {
    if year % 4 == 0 {
        if year % 100 == 0 {
            if year % 400 == 0 {
                return true;
            } else {
                return false;
            }
        } else {
            return true;
        }
    } else {
        return false;
    }
}

pub fn problem19() {
    // 1st Jan 1901 = Tuesday
    let years: Vec<i32> = (1901..2001).collect(); // create vector of all years 1901 - 2000
    let mut first_sunday = 6; // first sunday of 1901 was 6 days in
    let mut count = 0;
    for year in years {
        if is_leap_year(year) {
            let mut first_of_month = vec![1, 32, 61, 92, 122, 153, 183, 214, 245, 275, 306, 336];
            first_of_month.retain(|&n| (n - first_sunday) % 7 == 0);
            count += first_of_month.len();  
            first_sunday -= 2;
        } else {
            let mut first_of_month = vec![1, 32, 60, 91, 121, 152, 182, 213, 244, 274, 304, 335];
            first_of_month.retain(|&n| (n - first_sunday) % 7 == 0);
            count += first_of_month.len();
            first_sunday -= 1;
        }
        if first_sunday < 1 {
            first_sunday = 7 + first_sunday;
        }
    }
    println!("Problem 19: {}", count);
}