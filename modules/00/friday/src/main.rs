fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 || year % 100 == 0) && year % 400 != 0
}

fn num_days_in_month(year: u32, month: u32) -> u32 {
    if is_leap_year(year) && mouth == 2 {
        29
    } else if mouth % 2 == 0 || mouth == 8 {
        31
    } else {
        30
    }
}

fn main() {
    println!("{}", is_leap_year(4));
}
