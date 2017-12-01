use itertools::Itertools;

type Duple = (i64, i64);
type Triplet = (i64, i64, i64);

pub fn is_pythagorean(target: i64, possible: Duple) -> bool {
    match possible {
        (x, y) => {
            let a = x * y;
            let b = (x * x - y * y) / 2;
            let c = (x * x + y * y) / 2;
            a * a + b * b == c * c &&
              a + b + c == target
        }
    }
}

pub fn do_ex() -> Option <Triplet> {
    let space_a = (1..=1000).step_by(2);
    let space_b = (1..=1000).step_by(2);

    space_a.cartesian_product(space_b)
        .filter(|x| match *x {
            (a, b) => a > b
        })
        .find(|x| is_pythagorean(1000, *x))
        .map(|x| match x {
            (a, b) => (a * b, (a * a - b * b) / 2, (a * a + b * b) / 2)
        })
}
