extern crate num;
use self::num::traits::Num;

// I legit hate this type class derivation syntax...
# [derive (Debug, PartialEq, Eq)]
pub struct ProdPair <A : Num> (pub A, pub A, pub A);

pub fn is_palindrome <A : Clone + PartialEq> (digits: Vec <A>) -> bool {
    let mut reversed: Vec <A> = digits .clone ();
    reversed .reverse ();
    digits == reversed
}

pub fn digits (a: i64) -> Vec <i64> {
    let mut digits = Vec :: new ();
    let mut inter = a;
    while inter != 0 {
        digits .push (inter % 10);
        inter = inter / 10;
    }
    digits
}

fn is_palindrome_pair (num: i64) -> bool {
    is_palindrome (digits (num))
}

fn palindromic_product (x: i64, y: i64) -> bool {
    is_palindrome_pair (x * y)
}

fn min (x: i64, y: i64) -> i64 {
    if x > y { y } else { x }
}

fn max (x: i64, y: i64) -> i64 {
    if x > y { x } else { y }
}

pub fn do_ex () -> ProdPair <i64> {
    let mut pair = ProdPair (0, 0, 0);
    for i in (100 .. 1000).rev() {
        // By symmetry, only need those less than i
        println!("i: {0}", i.to_string());
        for j in (100 .. 1000).rev() {
            println!("j: {0}", j.to_string());
            if palindromic_product (i, j) && i*j > pair.2 {
                // return lesser on left
                pair = ProdPair (min (j, i), max (j, i), i * j);
            }
        }
    }
    pair
}
