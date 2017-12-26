use eulers::shared::divisors::divisors;
use eulers::shared::shapenum::triangular;

pub fn do_ex() -> u64 {
    let mut answer = 0;
    let mut triangular_num = Some(6);
    let mut j = 3;
    while answer <= 500 {
        triangular_num = triangular(j);
        answer = triangular_num.map(divisors).unwrap_or(0);
        println!("{:?} has {}", triangular_num, answer);
        j += 1;
    }
    triangular(j - 1).unwrap_or(0)
}
