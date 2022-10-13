#[allow(unused, dead_code)]
pub mod structs {

    #[derive(Clone, Debug)]
    pub struct RowAccumulator {
        pub keyword: Vec<u8>,
        pub language: Vec<u8>,
        pub subkey: Vec<u8>,
        pub subkeys: Vec<Vec<u8>>,
        pub value: Vec<u8>,
        pub values: Vec<Vec<u8>>,
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
                list_value: self.values.into_iter().map(|x| x).collect(),
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

    #[derive(Clone, Debug)]
    pub struct PxRow {
        pub keyword: PxKeyword,
        pub value: PxValue,
    }

    #[derive(Clone, Debug)]
    pub struct PxValue {
        pub number_value: Option<i32>,
        pub string_value: Option<String>,
        pub list_value: Option<Vec<String>>,
    }

    #[derive(Clone, Debug)]
    pub struct PxKeyword {
        pub keyword: String,
        pub language: Option<String>,
        pub subkeys: Option<Vec<String>>,
    }

}