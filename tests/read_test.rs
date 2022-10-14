#[allow(unused)]
#[cfg(test)]
mod read_tests {
    use loxis::px_parser;
    use std::fs::File;
    use std::io::BufReader;
    use std::io::prelude::*;
    use loxis::px_parser::Parser;
    use loxis::structs::structs::*;

    #[test]
    fn test_file() -> std::io::Result<()> {
        let mut file = File::open("../gpcaxis/data/010_kats_tau_101.px")?;
        let mut parser = Parser::new(file);
        // assert_eq!("NOTE", note.keyword);

        parser.parse_header();
        println!("headers: {:?}", parser.headers);

        Ok(())
    }

}
