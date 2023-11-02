use crate::symbol_table::symbol_table_position::SymbolTablePosition;

pub struct PIFEntry {
    pub token: isize,
    pub st_pos: Option<SymbolTablePosition>
}

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