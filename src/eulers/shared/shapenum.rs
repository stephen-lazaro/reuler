pub fn triangular(n: u64) -> Option <u64> {
    n.checked_mul(n + 1).and_then(|x| x.checked_div(2))
}
