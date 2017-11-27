use eulers::shared::primes::ordered_primes_under;

pub fn do_ex() -> u64 {
    let mut answer: Option<u64> = None;
    let mut bound = 100;
    while answer.is_none() {
        let primes = ordered_primes_under(bound);
        bound *= 10;
        answer = primes.get(10000).map(|x| *x);
    }

    match answer {
        Some(x) => x,
        None => 0
    }
}
