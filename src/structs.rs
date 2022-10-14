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
        pub fn to_row(&self) -> PxRow {
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
            let lv: Vec<String> = self.values.iter()
                .map(|bl| bl.iter().map(|cu| char::from(*cu)).collect::<String>())
                .collect();
            PxRow {
                keyword: kw,
                language: lng,
                subkeys: sks,
                values: lv,
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct PxRow {
        pub keyword: String,
        pub language: Option<String>,
        pub subkeys: Option<Vec<String>>,
        pub values: Vec<String>,
    }

}