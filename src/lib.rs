mod eulers;
use eulers::shared;
#[cfg(test)]
mod tests {
    #[test]
    fn test_one () {
        assert!(super::eulers::one::do_ex() == 233168)
    }

    #[test]
    fn test_two () {
        assert!(super::eulers::two::do_ex() == 4613732)
    }

    #[test]
    fn test_shared_one () {
        assert!(shared::prime_factors(5) == [5])
    }
    #[test]
    fn test_shared_two () {
        assert!(shared::prime_factors(6) == [2, 3])
    }
}
