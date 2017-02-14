/*
 * Calculates term-th fibonacci number.
 */
pub fn fibs (term: i32) -> i32 {
    fn term_seq (a: i32, b: i32, j: i32) -> (i32, i32) {
        if j > 0 {
            term_seq (b, a + b, j - 1)
        } else {
            (b, a + b)
        }
    }
    term_seq (0, 1, term).1
}
