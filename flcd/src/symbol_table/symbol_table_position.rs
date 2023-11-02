use crate::symbol_table::symbol_table::SymbolTableType;

pub struct SymbolTablePosition {
    pub tp: SymbolTableType,
    pub bucket: usize,
    pub item: usize
}