#![cfg_attr(feature = "nightly", feature(const_fn_trait_bound))]


#[allow(unused, dead_code, const_fn_trait_bound)]
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

    pub struct Parser {
        pub file: File
    }

    impl Parser {
        pub const fn new(f: File) -> Self {
            Parser { file: f }
        }
        pub fn read_u8() -> std::io::Result<u8> {
            let mut buffer = [0; 1];
            f.read(&mut buffer)?;
            Ok(buffer[0])
        }
    }

    pub fn read_px_metadata<F: Read + Seek>(f: &mut F) -> std::io::Result<Vec<PxRow>> {
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
        let mut buffer = [0; 10];
        f.seek(SeekFrom::Start(0));
        f.read(&mut buffer)?;
        Ok(result)
    }

    fn read_px_row<F: Read + Seek>(f: &mut F) -> Result<PxRow, String> {
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
