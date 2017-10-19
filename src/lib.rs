mod eulers;

#[cfg(test)]
mod tests {
    use eulers::one;
    use eulers::two;
    use eulers::three;
    use eulers::four;
    //use eulers::five;

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

    // Test will need to be fixed when answer is known
    #[test]
    fn test_four () {
        let answer = four::ProdPair (913, 993, 906609);
        assert!(four::do_ex() == answer)
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

}
