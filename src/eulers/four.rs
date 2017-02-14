fn is_palindrome (num: i64) -> bool {
    // Eventually, I'll need this to work
    true
}

fn palindromic_product (x: i64, y: i64) -> bool {
    is_palindrome (x*y)
}

pub fn do_ex () -> i64 {
    let pair = (0, 0);
    for i in 999..1 {
        for j in 999..1 {
            if palindromic_product(i*j) {
                pair = (i, j);
                break;
            }
        }
        break;
    }
    pair
}
