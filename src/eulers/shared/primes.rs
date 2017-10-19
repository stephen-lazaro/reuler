// It should be possible to generalize over integer types
// extern crate num;
// use num::Integral;
//
// pub fn eratosthenes <A : Integral> (bound: A) -> Vec <bool> {
//     let sqrt_bound = (bound as f32) .sqrt () as i32;
//     let mut primes = vec! [true; (sqrt_bound - 2) as usize];
//     for divisor in 2 .. sqrt_bound {
//         let mut factor = 1;
//         while factor*divisor < sqrt_bound {
//             primes [(factor*divisor - 2) as usize] = false;
//             factor += 1;
//         }
//     }
//     primes
// }

pub fn eratosthenes32 (bound: i32) -> Vec <bool> {
    let sqrt_bound = (bound as f32) .sqrt () as i32;
    let mut primes = vec! [true; (sqrt_bound - 2) as usize];
    for divisor in 2 .. sqrt_bound {
        let mut factor = 1;
        while factor*divisor < sqrt_bound {
            primes [(factor*divisor - 2) as usize] = false;
            factor += 1;
        }
    }
    primes
}

pub fn eratosthenes64 (bound: i64) -> Vec <bool> {
    let sqrt_bound = (bound as f64) .sqrt () as i64;
    let mut primes = vec! [true; (sqrt_bound - 2) as usize];
    for divisor in 2 .. sqrt_bound {
        let mut factor = 2;
        while factor*divisor < sqrt_bound {
            primes [(factor*divisor - 2) as usize] = false;
            factor += 1;
        }
    }
    primes
}

pub fn prime_factors32 (prod: i32) -> Vec <i32> {
    let subprimes = eratosthenes32 (prod);
    let mut factors: Vec <i32> = vec! [];
    for i in 2..((prod as f32).sqrt() as i32) {
        if subprimes[(i - 2) as usize] && prod % i == 0 {
            factors.push(i);
        }
    }
    factors
}

pub fn prime_factors (prod: i64) -> Vec <i64> {
    let subprimes = eratosthenes64 (prod);
    let mut factors: Vec <i64> = vec! [];
    for i in 2..((prod as f64).sqrt() as i64) {
        if subprimes[(i - 2) as usize] && prod % i == 0 {
            factors.push(i);
        }
    }
    factors
}

pub fn primes_under_x (bound: i64) -> Vec <i64> {
    // arbitrarily chosen correction factor to guarantee sufficient primes
    let subprimes = eratosthenes64 (100*bound);
    let mut under_bound: Vec <i64> = vec! [];
    for i in 2..bound {
        if subprimes[(i - 2) as usize] {
            println!("{}", i.to_string());
            under_bound.push(i)
        }
    }
    under_bound
}
