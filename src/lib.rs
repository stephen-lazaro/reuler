#![feature(inclusive_range_syntax)]
#![feature(iterator_step_by)]
#[macro_use] extern crate itertools;
#[allow(dead_code)]
#[allow(unused_import)]
mod eulers;


#[cfg(test)]
mod tests {
    extern crate quickcheck;
    use self::quickcheck::Testable;

    // Tagging structs for quickcheck testing
    pub struct Valid <A : Testable> (A);
    pub struct Invalid <A : Testable> (A);

    trait Unwrap {
        type Item;
        fn unwrap (self) -> Self::Item;
    }

    impl <A : Testable> Unwrap for Valid <A> {
        type Item = A;
        fn unwrap (self) -> Self::Item {
            let Valid (underlying) = self;
            underlying
        }
    }

    impl <B : Testable> Unwrap for Invalid <B> {
        type Item = B;
        fn unwrap (self) -> Self::Item {
            let Invalid (underlying) = self;
            underlying
        }
    }

    use eulers::one;
    use eulers::two;
    use eulers::three;
    use eulers::four;
    use eulers::five;
    use eulers::six;
    use eulers::seven;
    use eulers::eight;
    use eulers::nine;
    use eulers::ten;
    use eulers::shared;

    #[test]
    fn test_one () {
        assert!(one::do_ex() == 233168)
    }

    #[test]
    fn test_two () {
        assert!(two::do_ex() == 4613732)
    }

    #[test]
    fn test_three () {
        assert!(three::do_ex() == 6857)
    }

    #[test]
    fn test_four () {
        let answer = four::ProdPair (913, 993, 906609);
        assert!(four::do_ex() == answer)
    }

    #[test]
    fn test_five () {
        let answer = 232792560;
        assert!(five::do_ex() == answer)
    }

    #[test]
    fn test_six () {
        let answer = 25164150;
        assert!(six::do_ex() == answer)
    }

    #[test]
    fn test_seven () {
        let answer = 104743;
        assert!(seven::do_ex() == answer)
    }

    #[test]
    fn test_eight () {
        let answer = 23514624000;
        assert!(eight::do_ex() == answer)
    }

    #[test]
    fn test_nine () {
        let answer = (375, 200, 425);
        assert!(match nine::do_ex(){
            Some(x) => x == answer,
            None => false
        })
    }

    #[test]
    fn test_ten () {
        let answer = 142913828922;
        assert!(ten::do_ex() == answer)
    }

    #[test]
    fn test_palindrome_1 () {
        let mock = vec![0;5];
        let answer = four::is_palindrome(mock);
        assert!(answer)
    }

    #[test]
    fn test_palindrome_2 () {
        let mock = vec![1,2,3,2,1];
        assert!(four::is_palindrome(mock))
    }

    #[test]
    fn test_palindrome_3 () {
        let mock = vec![1,1,3,4,5];
        assert!(!four::is_palindrome(mock))
    }

    #[test]
    fn test_digits_1 () {
        let mock = 12;
        assert!(four::digits(mock) == vec![2, 1])
    }

    #[test]
    fn test_digits_2 () {
        let mock = 334;
        assert!(four::digits(mock) == vec![4, 3, 3])
    }

    #[test]
    fn test_divisor_power () {
        let mock = 27;
        assert!(shared::divisors::divisor_power(mock, 3, 0) == 3)
    }

    #[test]
    fn test_divisor_power_2 () {
        let mock = 2432902008176640000;
        println!("{}", shared::divisors::divisor_power(mock, 2, 0).to_string());
        assert!(shared::divisors::divisor_power(mock, 2, 0) > 0)
    }

    #[test]
    fn test_gcd () {
        let mock = five::gcd(2, 3);
        assert!(mock == 1)
    }

    #[test]
    fn test_gcd_2 () {
        let mock = five::gcd(27, 9);
        assert!(mock == 9);
    }

    #[test]
    fn test_gcd_3 () {
        let mock = five::gcd(32, 12);
        assert!(mock == 4);
    }
}
