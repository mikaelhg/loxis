#[allow(unused)]

#[cfg(test)]
mod read_tests {
    use loxis::px_parser;
    use std::fs::File;
    use std::io::BufReader;
    use std::io::prelude::*;
    use loxis::px_parser::Parser;

    #[test]
    fn test_file() -> std::io::Result<()> {
        let mut file = File::open("../gpcaxis/data/010_kats_tau_101.px")?;
        let parser = Parser::new(&file);
        /*
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;
//        assert_eq!(contents, "foo bar");
        let asd = "foo bar";
        assert_eq!("foo bar", asd);
        */

        Ok(())
    }

}
