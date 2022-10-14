#[allow(unused)]
#[cfg(test)]
mod read_tests {
    use loxis::px_parser;
    use std::fs::File;
    use std::io::BufReader;
    use std::io::prelude::*;
    use loxis::px_parser::Parser;
    use loxis::structs::*;

    #[test]
    fn test_file() -> std::io::Result<()> {
        let mut file = File::open("../gpcaxis/data/010_kats_tau_101.px")?;
        let mut reader = BufReader::new(file);
        let mut parser = Parser::new(reader);

        parser.parse_header();
        parser.parse_data_dense();

        // println!("headers: {:?}", parser.headers);
        println!("headers.len: {:?}", parser.headers.len());

        assert_eq!(81, parser.headers.len());

        Ok(())
    }

}
