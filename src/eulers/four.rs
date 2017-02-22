 // no deriving show :(
 #[derive(Debug)]
pub struct ProdPair <a> (a, a);

fn is_palindrome (num: i64) -> bool {
    // Eventually, I'll need this to work
    true
}

fn palindromic_product (x: i64, y: i64) -> bool {
    is_palindrome (x*y)
}

fn min (x: i64, y: i64) -> i64 {
    if x > y { y } else { x }
}

fn max (x: i64, y: i64) -> i64 {
    if x > y { x } else { y }
}

pub fn do_ex () -> ProdPair <i64> {
    let mut pair = ProdPair (0, 0);
    for i in 999..1 {
        // By symmetry, only need those less than i
        for j in i..1 {
            if palindromic_product(i, j) {
                // return lesser on left
                pair = ProdPair (min (j, i), max (j, i));
                break;
            }
        }
        break;
    }
    pair
}
