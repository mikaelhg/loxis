pub mod structs;

#[allow(unused, dead_code)]
pub mod px_parser {

    use std::fs::File;
    use std::io::{Read, SeekFrom};
    use std::io::prelude::*;
    use std::ptr::null;
    use crate::structs::structs::*;

    const C_DATA: [u8; 4] = [b'D', b'A', b'T', b'A'];

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
            Self {
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

    pub struct Parser {
        pub file: File,
        pub hps: HeaderParseState,
        pub row: RowAccumulator,
        pub headers: Vec<PxRow>,
    }

    impl Parser {

        pub const fn new(f: File) -> Self {
            Self {
                file: f,
                hps: HeaderParseState::new(),
                row: RowAccumulator::new(),
                headers: vec![],
            }
        }

        pub fn parse_header(&mut self) -> std::io::Result<()> {
            let mut buffer = [0; 4096];
            self.file.seek(SeekFrom::Start(0));

            loop {
                match self.file.read(&mut buffer) {
                    Result::Ok(0) => {
                        break;
                    },
                    Result::Ok(size) => {
                        for i in 0 .. size {
                            if self.parse_header_character(buffer[i]) {
                                break;
                            }
                        }
                    },
                    Result::Err(e) => {
                        break;
                    }
                }
            }

            Ok(())
        }

        fn parse_header_character(&mut self, c: u8) -> bool {
            let in_quotes = self.hps.quotes % 2 == 1;
            let in_parenthesis = self.hps.parenthesis_open > self.hps.parenthesis_close;
            let in_key = self.hps.semicolons == self.hps.equals;
            let in_language = in_key && self.hps.squarebracket_open > self.hps.squarebracket_close;
            let in_subkey = in_key && in_parenthesis;

            if c == b'"' {
                self.hps.quotes += 1;

            } else if (c == b'\n' || c == b'\r') && in_quotes {
                return true; // true, errors.New("there can't be newlines inside quoted strings")

            } else if (c == b'\n' || c == b'\r') && !in_quotes {
                return false; // false, nil

            } else if c == b'[' && in_key && !in_quotes {
                self.hps.squarebracket_open += 1;

            } else if c == b']' && in_key && !in_quotes {
                self.hps.squarebracket_close += 1;

            } else if c == b'(' && in_key && !in_quotes {
                self.hps.parenthesis_open += 1;

            } else if c == b'(' && !in_key && !in_quotes {
                // TLIST opening quote
                self.hps.parenthesis_open += 1;
                self.row.value.push(c);

            } else if c == b')' && in_key && !in_quotes {
                self.hps.parenthesis_close += 1;
                self.row.subkeys.push(self.row.subkey.clone());
                self.row.subkey = vec![];

            } else if c == b')' && !in_key && !in_quotes {
                // TLIST closing quote
                self.hps.parenthesis_close += 1;
                self.row.value.push(c);

            } else if c == b',' && in_subkey && !in_quotes {
                self.row.subkeys.push(self.row.subkey.clone());
                self.row.subkey = vec![];

            } else if c == b',' && !in_key && !in_quotes && !in_parenthesis {
                self.row.values.push(self.row.value.clone());
                self.row.value = vec![];

            } else if c == b'=' && !in_key && !in_quotes {
                return true; // true, errors.New("found a second equals sign without a matching semicolon, unexpected keyword terminator")

            } else if c == b'=' && in_key && !in_quotes {
                if self.row.keyword == C_DATA {
                    return true; // true, nil
                }
                self.hps.equals += 1;

            } else if c == b';' && in_key && !in_quotes {
                return true; // true, errors.New("found a semicolon without a matching equals sign, value terminator without keyword terminator")

            } else if c == b';' && !in_key && !in_quotes {
                if self.row.value.len() > 0 {
                    self.row.values.push(self.row.value.clone());
                }
                self.hps.semicolons += 1;
                self.headers.push(self.row.to_row());
                self.row = RowAccumulator::new();
                return false; // false, nil

            } else if in_subkey {
                self.row.subkey.push(c);

            } else if in_language {
                self.row.language.push(c);

            } else if in_key {
                self.row.keyword.push(c);

            } else {
                self.row.value.push(c);
            }

            return false;
        }

    }

}
