use std::collections::HashSet;
use eulers::shared::primes::primes_under;

const TWO_MILL: u64 = 2000000;

pub fn do_ex() -> u64 {
    let primes: HashSet<u64> = primes_under (TWO_MILL);
    println!("{:?}", primes.iter().max());
    primes.iter().sum()
}
