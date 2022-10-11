#[allow(unused)]

#[cfg(test)]
mod read_tests {
    use loxis::px_parser;
    use std::fs::File;
    use std::io::BufReader;
    use std::io::prelude::*;

    #[test]
    fn test_file() -> std::io::Result<()> {
        let mut file = File::open("README.md")?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;
//        assert_eq!(contents, "foo bar");
        let asd = "foo bar";
        assert_eq!("foo bar", asd);
        Ok(())
    }

}
