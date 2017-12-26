use std::collections::HashSet;
use std::collections::HashMap;
use eulers::shared::primes::primes_under;
use eulers::shared::divisors::divisor_power;

/**
 * Calculates the LCM of 1..20
 */

pub fn gcd (a: u64, b: u64) -> u64 {
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

pub fn do_ex() -> u64 {
    let relevant: HashSet<u64> = vec![2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20]
        .iter()
        .map(|x| *x)
        .collect();

    let primes: HashMap<u64, u64> = primes_under (20)
        .iter()
        .map(|prime| 
             (
              *prime,
              relevant
                .iter()
                .map(|num| 
                    match num.checked_rem(*prime).map(|rem| rem == 0) {
                      Some(true) => divisor_power(*num, *prime, 0),
                      Some(false) => 0,
                      None => 0,
                    })
                .fold(0, |x, y| x.max(y))
             )
        ).collect();

    primes
      .iter()
      .fold(1, |x, y| x * match y {
          (base, exp) => base.pow(*exp as u32)
      })
}
