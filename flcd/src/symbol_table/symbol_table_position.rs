use crate::symbol_table::symbol_table::SymbolTableType;

#[derive(Debug)]
pub struct SymbolTablePosition {
    pub tp: SymbolTableType,
    pub bucket: usize,
    pub item: usize
}