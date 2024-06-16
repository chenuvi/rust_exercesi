fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false;
    }
    if num == 2 {
        return true; // 2 is the only even prime number
    }
    if num % 2 == 0 {
        return false; // Other even numbers are not primes
    }
    let limit = (num as f64).sqrt() as u64 + 1;
    for i in (3..limit).step_by(2) {
        if num % i == 0 {
            return false;
        }
    }
    true
}

pub fn nth(n: u64) -> u64 {
    println!("nth n: {}", n);
    let mut count = 0;
    let mut candidate = 2;
    while count <= n {
        println!("count : {}", count);
        if is_prime(candidate) {
            println!("candidate is_prime : {}", candidate);
            if count == n {
                return candidate;
            }
            count += 1;
        }
        println!("candidate be  candidate += : {}", candidate);
        candidate += 1;
        println!("candidate after  candidate += : {}", candidate);
    }
    0
}

fn main() {
    println!("{}", nth(1));
}
