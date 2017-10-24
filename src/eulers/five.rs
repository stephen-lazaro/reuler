use eulers::shared::primes::primes_under_x;

/**
 * Calculates the LCM of 1..20
 */

pub fn gcd (a: u64, b: u64) -> u64 {
    println!("{}", b.to_string());
    if a > b {
        gcd_helper (b, a)
    } else {
        gcd_helper (a, b)
    }
}

fn gcd_helper (a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd_helper (b, a % b)
    }
}

pub fn divisor_power (prod: u64, fact: u64) -> u64 {
    let mut power = 0;
    let a = fact;
    while a.pow(power) < prod {
        power += 1;
    }
    power as u64
}

pub fn do_ex() -> u64 {
    let relevant: u64 = vec![2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20]
        .iter()
        .fold(1, |x, y| x*y);
    relevant

    // What you actually want to do
    // is count the highest power for each prime of each number in the product
    // then go from there.
    // This is a zip with because subset of the product space.
}
