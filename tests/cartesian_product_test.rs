#[allow(unused)]
#[cfg(test)]
mod cartesian_product_tests {

    use loxis::cartesian_product::CartesianProduct;

    #[test]
    fn test_product() -> std::io::Result<()> {
        let input = vec![
            vec!["a".to_string(), "b".to_string(), "c".to_string()],
            vec!["1".to_string(), "2".to_string(), "3".to_string()]
        ];
        let output = vec![
            vec!["a".to_string(), "1".to_string(),],
            vec!["a".to_string(), "2".to_string(),],
            vec!["a".to_string(), "3".to_string(),],
            vec!["b".to_string(), "1".to_string(),],
            vec!["b".to_string(), "2".to_string(),],
            vec!["b".to_string(), "3".to_string(),],
            vec!["c".to_string(), "1".to_string(),],
            vec!["c".to_string(), "2".to_string(),],
            vec!["c".to_string(), "3".to_string(),],
        ];
        let mut c = CartesianProduct::new(&input);
        let mut values: Vec<&String> = Vec::with_capacity(2);
        c.next(&mut values);

        println!("values: {:?}", &mut values);

        assert_eq!(input, output);
        Ok(())
    }

}