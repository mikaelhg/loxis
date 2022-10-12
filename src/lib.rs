#[allow(unused, dead_code)]
pub mod px_parser {

    use std::fs::File;
    use std::io::{Read, SeekFrom};
    use std::io::prelude::*;
    use std::ptr::null;

    pub struct PxRow {
        pub keyword: PxKeyword,
        pub value: PxValue,
    }

    pub struct PxValue {
        pub number_value: Option<i32>,
        pub string_value: Option<String>,
        pub list_value: Option<Vec<String>>,
    }

    pub struct PxKeyword {
        pub keyword: String,
        pub language: Option<String>,
        pub specifiers: Option<Vec<String>>,
    }

    pub struct RowAccumulator {
        pub keyword:  String,
        pub language: String,
        pub subkey:   String,
        pub subkeys:  Vec<String>,
        pub value:    String,
        pub values:   Vec<String>,
    }

    impl RowAccumulator {
        pub const fn new() -> Self {
            RowAccumulator {
                keyword: "".to_string(),
                language: "".to_string(),
                subkey: "".to_string(),
                subkeys: vec![],
                value: "".to_string(),
                values: vec![],
            }
        }
    }

    pub struct HeaderParseState {
        pub count:               u32,
        pub quotes:              u32,
        pub semicolons:          u32,
        pub equals:              u32,
        pub squarebracket_open:  u32,
        pub squarebracket_close: u32,
        pub parenthesis_open:    u32,
        pub parenthesis_close:   u32,
    }

    impl HeaderParseState {
        pub const fn new() -> Self {
            HeaderParseState {
                count: 0,
                quotes: 0,
                semicolons: 0,
                equals: 0,
                squarebracket_open: 0,
                squarebracket_close: 0,
                parenthesis_open: 0,
                parenthesis_close: 0,
            }
        }
    }

    pub struct Parser<'a> {
        pub file: &'a File,
        pub hps: &'a HeaderParseState,
        pub row: &'a RowAccumulator,
    }

    impl <'a> Parser<'a> {

        pub const fn new(f: &'a File) -> Self {
            Parser {
                file: f,
                hps: &HeaderParseState::new(),
                row: &RowAccumulator::new(),
            }
        }

        pub fn read_u8(&mut self) -> std::io::Result<u8> {
            let mut buffer = [0; 1];
            self.file.read(&mut buffer)?;
            Ok(buffer[0])
        }


    }

    pub fn read_px_metadata<T>(f: &mut T)
                               -> std::io::Result<Vec<PxRow>>
        where T: Read + Seek,
    {
        let mut result: Vec<PxRow> = Vec::new();
        let mut row = PxRow {
            keyword: PxKeyword {
                keyword: "FOO".to_string(),
                language: None,
                specifiers: None
            },
            value: PxValue {
                number_value: Some(123),
                string_value: None,
                list_value: None
            }
        };
        result.push(row);
        let mut buffer = [0; 4096];
        f.seek(SeekFrom::Start(0));

        let size = f.read(&mut buffer)?;
        for i in 0 .. size {
            self.parse_header_character(buffer[i]);
        }


        Ok(result)
    }

    fn read_px_row<T>(f: &mut T)
                      -> Result<PxRow, String>
        where T: Read + Seek,
    {
        Err("nope".to_string())
    }

    fn read_px_keyword<F: Read + Seek>(f: &mut F) -> Result<PxKeyword, String> {
        let in_subkey = false;
        let subkey: Vec<String> = Vec::new();

        Err("nope".to_string())
    }

    fn read_px_value<F: Read + Seek>(f: &mut F) -> Result<PxValue, String> {
        Err("nope".to_string())
    }

}
