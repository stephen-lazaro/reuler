use eulers::shared::primes::primes_under_x;

/**
 * Calculates the LCM of 1..20
 */

// IIRC the trick here is to take the max over the prime factor exponents for each prime_factors
// dividing each factor. Then, each factor will evenly divide that product.
// So, we need to prime factorize each number 1..20 and count the exponents as well.
// We can reuse prior art from 4 in order to do this, but let's note that there's some
// let us say, tech debt involved therein in the sense that _really_ our method should just
// in one go return both, since this is essentially low cost to calculate in one pass
// but more expensive in two passes.

// Only valid for prime factors
pub fn divisor_power (prod: i64, fact: i64) -> i64 {
    let mut power = 0;
    let mut a = prod;
    while a > 1 {
        power += 1;
        a = a / fact;
    }
    power
}

pub fn do_ex() -> Vec <i64> {
    let relevant: i64 = vec![2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20]
        .iter()
        .fold(1, |x, y| x*y);

    println!("we managed to compute relevant");
    primes_under_x (20)
        .iter()
        .map(|x| {
            println!("relevant prime: {}", x.to_string());
           (x, divisor_power(relevant, *x))
        }).into_iter()
        .map(|tuple| match tuple {
            (a, b) => {
                println!("computing power: {}", a.to_string());
                a.pow(b as u32)
            }
        })
        .collect()
}
