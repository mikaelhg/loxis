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
            let lv: Vec<String> = self.values.iter()
                .map(|bl| bl.iter().map(|cu| char::from(*cu)).collect::<String>())
                .collect();
            PxValue {
                number_value: None,
                string_value: None,
                list_value: Some(lv),
            }
        }
        pub fn to_keyword(&self) -> PxKeyword {
            let kw = self.keyword.iter().map(|cu| char::from(*cu)).collect::<String>();
            let lng: Option<String> = if self.language.is_empty() {
                None
            } else {
                Some(self.language.iter().map(|cu| char::from(*cu)).collect::<String>())
            };
            let sks: Option<Vec<String>> = if self.subkeys.is_empty() {
                None
            } else {
                Some(self.subkeys.iter()
                    .map(|bl| bl.iter().map(|cu| char::from(*cu)).collect::<String>())
                    .collect())
            };
            PxKeyword {
                keyword: kw,
                language: lng,
                subkeys: sks,
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