use std::collections::HashMap;
use crate::token::token_scanner;

pub struct TokenManager {
    hash_table: HashMap<String, isize>
}

impl TokenManager {
    pub fn new(file_name: &str) -> TokenManager {
        let lines = token_scanner::scan_token_file(file_name).unwrap();

        let mut hash_table = HashMap::new();
        let mut idx = 0;
        for line in lines {
            hash_table.insert(line, idx);
            idx += 1;
        }

        Self {
            hash_table
        }
    }

    pub fn get_token_code(&self, token: &str) -> Option<isize> {
        self.hash_table.get(token).cloned()
    }
}