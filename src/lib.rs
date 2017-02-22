mod eulers;

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
    fn test_three () {
        assert!(super::eulers::three::do_ex() == 6857)
    }

    // Test will need to be fixed when answer is known
    #[test]
    fn test_four () {
        let answer = super::eulers::four::ProdPair (6, 7);
        assert!(super::eulers::four::do_ex() != answer)
    }
}
