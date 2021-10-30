#[allow(unused, dead_code)]

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub mod px_parser {
    use std::fs::File;
    use std::io::Read;
    use std::io::prelude::*;

    pub struct PxRow {
        pub keyword: PxKeyword,
        pub value: PxValue
    }

    pub struct PxValue {
        pub number_value: Option<i32>,
        pub string_value: Option<String>,
        pub list_value: Option<Vec<String>>
    }

    pub struct PxKeyword {
        pub keyword: String,
        pub language: Option<String>,
        pub specifiers: Option<Vec<String>>
    }

    pub fn read_px_metadata(f: &mut File) -> Result<Vec<PxRow>, String> {
        f.read_u8();
        Err("nope".to_string())
    }

    fn read_px_row() -> Result<PxRow, String> {
        Err("nope".to_string())
    }

    fn read_px_keyword() -> Result<PxKeyword, String> {
        Err("nope".to_string())
    }

    fn read_px_value() -> Result<PxValue, String> {
        Err("nope".to_string())
    }

}
