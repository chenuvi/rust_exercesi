pub fn is_armstrong_number(num: u32) -> bool {
    if num < 10 {
        return true;
    }

    let digit_count = num.to_string().len() as u32;
    let mut sum: u64 = 0;

    let mut n = num as u64;

    while n > 0 {
        println!("n: {}", n);
        sum += (n % 10).pow(digit_count);
        n /= 10;
    }
    sum == num as u64
}
