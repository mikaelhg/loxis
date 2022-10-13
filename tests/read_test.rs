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
        let mut note = PxKeyword {
            keyword: "NOTE".to_string(),
            language: Some("fi".to_string()),
            subkeys: None,
        };
        println!("note: {:?}[{:?}]", note.keyword, note.language);
        assert_eq!("NOTE", note.keyword);

        parser.read_px_metadata();
        println!("headers: {:?}", parser.headers);

        Ok(())
    }

}
