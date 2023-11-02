use crate::scanner::source_code_scanner::process_source_code;
use crate::token::token_manager::TokenManager;

mod symbol_table;
mod token;
mod scanner;
mod pif;

fn main() {
    let token_manager = TokenManager::new("token.in");
    process_source_code("test.txt", &token_manager);
}
