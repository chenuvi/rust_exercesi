use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range(2..p)
}

fn modular_exponentiation(base: u128, exp: u64, modular: u64) -> u64 {
    let mut e = exp;
    let mut b = base;
    let mut result = 1;
    while e > 0 {
        if e % 2 == 0 {
            result = (result * b) % modular as u128;
        }
        b = (b * b) % modular as u128;
        e = e / 2;
    }
    result as u64
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_exponentiation(g as u128, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_exponentiation(b_pub as u128, a, p)
}
