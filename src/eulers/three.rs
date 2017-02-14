use eulers::shared::primes::prime_factors;

fn max_of_vector (nums: Vec <i64>) -> i64 {
    nums.iter().fold(0, |x: i64, y: &i64| { if x > *y { x } else { *y }})
}

fn largest_prime_factor (num: i64) -> i64 {
    max_of_vector (prime_factors (num))
}

pub fn do_ex () -> i64 {
    largest_prime_factor (600851475143)
}
