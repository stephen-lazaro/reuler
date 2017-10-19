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

fn divisor_power (prod: i64, fact: i64) -> i64 {
    let mut power = 0;
    let mut a = prod;
    while a > 0 {
        power += 1;
        a = a / fact;
    }
    power
}
