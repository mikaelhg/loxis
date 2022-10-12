#[allow(unused, dead_code)]
pub mod structs {

    pub struct RowAccumulator {
        pub keyword: Vec<u8>,
        pub language: Vec<u8>,
        pub subkey: Vec<u8>,
        pub subkeys: Vec<String>,
        pub value: Vec<u8>,
        pub values: Vec<String>,
    }

    impl RowAccumulator {
        pub const fn new() -> Self {
            Self {
                keyword: vec![],
                language: vec![],
                subkey: vec![],
                subkeys: vec![],
                value: vec![],
                values: vec![],
            }
        }
        pub fn to_value(&self) -> PxValue {
            PxValue {
                number_value: None,
                string_value: None,
                list_value: None,
            }
        }
        pub fn to_keyword(&self) -> PxKeyword {
            PxKeyword {
                keyword: "".to_string(),
                language: None,
                subkeys: None,
            }
        }
        pub fn to_row(&self) -> PxRow {
            PxRow {
                keyword: self.to_keyword(),
                value: self.to_value(),
            }
        }
    }

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
        pub subkeys: Option<Vec<String>>,
    }

}