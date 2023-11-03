use std::fmt;
use std::fmt::Formatter;
use crate::symbol_table::hash_table::MyHashMap;

#[derive(Debug)]
pub enum SymbolTableType {
    ID,
    INT,
    STRING
}

#[derive(Debug)]
pub struct SymbolTable {
    id_hash_table: MyHashMap<String>,
    int_const_hash_table: MyHashMap<i32>,
    string_const_hash_table: MyHashMap<String>
}

impl fmt::Display for SymbolTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ID: {} \n INT: {} \n STRING: {} \n", self.id_hash_table, self.int_const_hash_table, self.string_const_hash_table)
    }
}


impl SymbolTable {
    pub fn new(size: usize) -> Self {
        Self {
            id_hash_table: MyHashMap::new(size),
            int_const_hash_table: MyHashMap::new(size),
            string_const_hash_table: MyHashMap::new(size)
        }
    }

    pub fn add_id(&mut self, name: String) -> (usize, usize) {
        if let Some(val) = self.id_hash_table.get(&name) {
            return val;
        }

        self.id_hash_table.insert(name).unwrap()
    }

    pub fn add_int_const(&mut self, int_const: i32) -> (usize, usize) {
        if let Some(val) = self.int_const_hash_table.get(&int_const) {
            return val;
        }

        self.int_const_hash_table.insert(int_const).unwrap()
    }

    pub fn add_string_const(&mut self, string: String) -> (usize, usize) {
        if let Some(val) = self.string_const_hash_table.get(&string) {
            return val;
        }

        self.string_const_hash_table.insert(string).unwrap()
    }

    pub fn find_id(&self, name: &str) -> Option<(usize, usize)> {
        self.id_hash_table.get(name)
    }

    pub fn find_int(&self, int: &i32) -> Option<(usize, usize)> {
        self.int_const_hash_table.get(int)
    }

    pub fn find_string(&self, string: &str) -> Option<(usize, usize)> {
        self.string_const_hash_table.get(string)
    }

    pub fn get_id_by_position(&self, position: &(usize, usize)) -> Option<&String> {
        self.id_hash_table.get_by_position(position)
    }

    pub fn get_int_by_position(&self, position: &(usize, usize)) -> Option<&i32> {
        self.int_const_hash_table.get_by_position(position)
    }

    pub fn get_string_by_position(&self, position: &(usize, usize)) -> Option<&String> {
        self.string_const_hash_table.get_by_position(position)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn before_each() -> SymbolTable {
        let st = SymbolTable::new(16);
        st
    }

    #[test]
    fn test_id() {
        let mut st = before_each();
        st.add_id("a".to_owned()).unwrap();

        assert!(st.find_id("a").is_some());
        let pos = st.find_id("a").unwrap();
        assert_eq!(st.get_id_by_position(&pos).unwrap(), "a");
    }

    #[test]
    fn test_int() {
        let mut st = before_each();
        st.add_string_const("a".to_owned()).unwrap();

        assert!(st.find_string("a").is_some());
        let pos = st.find_string("a").unwrap();
        assert_eq!(st.get_string_by_position(&pos).unwrap(), "a");
    }

    #[test]
    fn test_string() {
        let mut st = before_each();
        st.add_int_const(1).unwrap();

        assert!(st.find_int(&1).is_some());
        let pos = st.find_int(&1).unwrap();
        assert_eq!(st.get_int_by_position(&pos).unwrap().to_owned(), 1);
    }
}