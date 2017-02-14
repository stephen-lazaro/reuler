/*
 * Sums even fibonaccis under bound
 */
fn even_fib_sum (bound: i32) -> i32 {
    fn accum (a: i32, b: i32, acc: i32, bound: i32) -> i32 {
        if b > bound {
            acc
        } else {
            let inc = if b % 2 == 0 { b } else { 0 };
            accum (b, a + b, acc + inc, bound)
        }
    }
    accum (0, 1, 0, bound)
}

pub fn do_ex () -> i32 {
    even_fib_sum (4000000)
}
