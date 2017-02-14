pub fn eratosthenes32 (bound: i32) -> Vec <bool> {
    let sqrtBound = (bound as f32).sqrt() as i32;
    let mut primes = vec![true; (sqrtBound - 2) as usize];
    for divisor in 2..bound {
        let mut factor = 1;
        while factor*divisor < bound {
            factor += 1;
            primes [(factor*divisor) as usize] = false;
        }
    }
    primes
}

// Might have to return pointers for indirection
pub fn eratosthenes64 (bound: i64) -> Vec <bool> {
    let sqrtBound = (bound as f64).sqrt() as i64;
    let mut primes = vec![true; (sqrtBound - 2) as usize];
    for divisor in 2..bound {
        let mut factor = 1;
        while factor*divisor < bound {
            factor += 1;
            primes [(factor*divisor) as usize] = false;
        }
    }
    primes
}

// Similar to above.
pub fn prime_factors (prod: i64) -> Vec <i64> {
    let subprimes = eratosthenes64 (prod);
    let mut factors: Vec <i64> = vec![];
    for i in 2.. ((prod as f64).sqrt() as i64 - 100) {
        if subprimes[(i - 2) as usize] && prod % i == 0 {
            factors.push(i);
        }
    }
    factors
}
