use std::collections::HashSet;
use eulers::shared::primes::prime_factors;
/**
 * Finds largest prime factor
 */
// Really this could be done over `IntoIterator` or something
fn max_of_set (nums: HashSet <u64>) -> u64 {
    nums.iter().fold(0, |x: u64, y: &u64| { if x > *y { x } else { *y }})
}

fn largest_prime_factor (num: u64) -> u64 {
    max_of_set (prime_factors (num))
}

pub fn do_ex () -> u64 {
    largest_prime_factor (600851475143)
}
