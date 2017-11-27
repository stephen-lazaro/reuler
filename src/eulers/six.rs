pub fn square(x: u64) -> u64 {
    x*x
}

pub fn do_ex() -> u64 {
    let answer_data: (u64, u64) =
        (1..=100)
          .into_iter()
          .map(|x| (x, square(x)))
          .fold((0,0), |x, y| match (x, y) {
              ((sum_l, square_l), (sum_r, square_r)) =>
                  (sum_l + sum_r, square_l + square_r)
          });

    match answer_data {
        (sum, sum_of_squares) =>
            sum*sum - sum_of_squares
    }
}
