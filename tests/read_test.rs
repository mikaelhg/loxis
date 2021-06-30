#[cfg(test)]
mod read_tests {
//    use super::*;
    use loxis::add;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

}
