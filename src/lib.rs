mod structs;

#[allow(unused, dead_code)]
pub mod px_parser {

    use std::fs::File;
    use std::io::{Read, SeekFrom};
    use std::io::prelude::*;
    use std::ptr::null;
    use crate::structs::structs::*;

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
        pub hps: HeaderParseState,
        pub row: RowAccumulator,
    }

    impl <'a> Parser<'a> {

        pub const fn new(f: &'a File) -> Self {
            Self {
                file: f,
                hps: HeaderParseState::new(),
                row: RowAccumulator::new(),
            }
        }

        pub fn read_u8(&mut self) -> std::io::Result<u8> {
            let mut buffer = [0; 1];
            self.file.read(&mut buffer)?;
            Ok(buffer[0])
        }

        pub fn read_px_metadata<T>(&mut self, f: &mut T)
                                   -> std::io::Result<Vec<PxRow>>
            where T: Read + Seek,
        {
            let mut result: Vec<PxRow> = vec![];

            let mut buffer = [0; 4096];
            f.seek(SeekFrom::Start(0));

            let size = f.read(&mut buffer)?;
            for i in 0 .. size {
                self.parse_header_character(buffer[i]);
            }

            Ok(result)
        }

        fn parse_header_character(&self, c: u8) {
            todo!()
        }

    }

}
