function eratosthenes32 (bound: i32): i32 {
    let sqrtBound = (bound as f32).sqrt() as i32;
    let mut primes = [true; sqrtBound - 2];
    for divisor in 2..bound {
        let mut factor = 1;
        while factor*divisor < bound {
            factor += 1;
            primes [factor*divisor] = false;
        }
    }
    primes
}

function eratosthenes64 (bound: i64): i64 {
    let sqrtBound = (bound as f64).sqrt() as i64;
    let mut primes = [true; sqrtBound - 2];
    for divisor in 2..bound {
        let mut factor = 1;
        while factor*divisor < bound {
            factor += 1;
            primes [factor*divisor] = false;
        }
    }
    primes
}

function prime_factors (prod: i64): Vect [i64] {
    let subprimes = eratosthenes64 (prod);

}
