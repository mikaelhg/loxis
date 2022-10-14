pub mod structs;
pub mod cartesian_product;

#[allow(unused, dead_code)]
pub mod px_parser {

    use std::fs::File;
    use std::io::{Read, SeekFrom, BufReader};
    use std::io::prelude::*;
    use std::ptr::null;
    use crate::cartesian_product::CartesianProduct;
    use crate::structs::*;

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
        pub reader: BufReader<File>,
        pub hps: HeaderParseState,
        pub row: RowAccumulator,
        pub headers: Vec<PxRow>,
    }

    impl Parser {

        pub const fn new(r: BufReader<File>) -> Self {
            Self {
                reader: r,
                hps: HeaderParseState::new(),
                row: RowAccumulator::new(),
                headers: vec![],
            }
        }

        pub fn parse_header(&mut self) -> std::io::Result<()> {
            let mut buffer = [0; 1];

            'outer:
            loop {
                match self.reader.read(&mut buffer) {
                    Result::Ok(0) => {
                        break 'outer;
                    },
                    Result::Ok(1) => {
                        if self.parse_header_character(buffer[0]) {
                            break 'outer;
                        }
                    },
                    Result::Ok(_) => {
                        break 'outer;
                    },
                    Result::Err(e) => {
                        break 'outer;
                    },
                }
            }

            Ok(())
        }

        pub fn parse_data_dense(&mut self) -> std::io::Result<()> {
            let header = |kw: &str| self.headers.iter().find(|x| x.keyword == kw).unwrap().values.clone();
            let values_header = |sk: &str| self.headers.iter()
                .find(|x| {x.keyword == "VALUES" && x.subkeys == Some(vec![sk.to_string()])})
                .unwrap().values.clone();

            let stub = header("STUB");
            let stub_values: Vec<Vec<String>> = stub.iter()
                .map(|x| values_header(x)).collect();
            let stub_width = stub.len();

            let heading = header("HEADING");
            let heading_values: Vec<Vec<String>> = heading.iter()
                .map(|x| values_header(x)).collect();
            let heading_width = heading.len();

            let mut heading_flattener = CartesianProduct::new(&heading_values);

            println!("stub: {:?}", stub);
            // println!("stub_values: {:?}", stub_values);
            println!("heading: {:?}", heading);

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
                self.hps.equals += 1;
                if self.row.keyword == C_DATA {
                    return true; // true, nil
                }

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
