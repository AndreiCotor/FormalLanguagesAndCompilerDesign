use std::fmt;
use std::fmt::{Formatter};
use crate::symbol_table::symbol_table_position::SymbolTablePosition;

#[derive(Debug)]
pub struct PIFEntry {
    pub token: isize,
    pub st_pos: Option<SymbolTablePosition>
}

impl fmt::Display for PIFEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {:?})", self.token, self.st_pos)
    }
}

#[derive(Debug)]
pub struct PIF {
    pif: Vec<PIFEntry>
}

impl PIF {
    pub fn new() -> Self {
        Self {
            pif: Vec::new()
        }
    }

    pub fn add(&mut self, entry: PIFEntry) {
        self.pif.push(entry);
    }
}

impl fmt::Display for PIF {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut res = String::new();
        for el in &self.pif {
            res.push_str(&format!("{}\n", el));
        }

        write!(f, "{}", res)
    }
}