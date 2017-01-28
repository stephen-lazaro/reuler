/*
 * Sums numbers divisible by a or b under bound
 */
fn divisors_under_bound (a: i32, b: i32, bound: i32) -> i32 {
    let mut result = 0;
    for i in 1..bound {
        result +=
            if i % a == 0 || i % b == 0 { i }
            else { 0 }
    }
    result
}

pub fn do_ex () -> i32 {
    divisors_under_bound (3, 5, 1000)
}
