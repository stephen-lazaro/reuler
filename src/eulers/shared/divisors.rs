use eulers::shared::primes::prime_factors;

/**
 * Let's calculate some features of the
 * divisor function.
 */
pub fn divisors(n: u64) -> u64 {
    sigma_0(n)
}

// Assumes fact is a prime dividing prod
pub fn divisor_power (prod: u64, fact: u64, acc: u64) -> u64 {
    if fact == 0 || fact == 1 {
       0
    } else {
        match (prod <= 1, prod.checked_div(fact)) {
          (false, Some(rem)) => divisor_power(rem, fact, acc + 1),
          (true, _) =>
              if prod == 1 { acc } else { acc - 1 },
          _ => 0
        }
    }
}

fn sigma_0(n: u64) -> u64 {
    prime_factors(n)
        .iter()
        .map(|x| { println!("Prime: {}", x); x})
        .map(|x| divisor_power(n, *x, 0) + 1)
        .map(|x| { println!("Power: {}", x); x})
        .product()
}
