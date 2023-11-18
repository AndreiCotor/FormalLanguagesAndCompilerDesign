use std::io;
use crate::scanner::source_code_scanner::process_source_code;
use crate::token::token_manager::TokenManager;

mod symbol_table;
mod token;
mod scanner;
mod pif;
mod finite_automata;

fn parse_source_code() {
    let token_manager = TokenManager::new("token.in");
    let int_const_fa = finite_automata::parser::parse_fa("int.in");
    let id_fa = finite_automata::parser::parse_fa("id.in");
    process_source_code("test.txt", &token_manager, &int_const_fa, &id_fa);
}

fn main() {
    println!("Choose an option:");
    println!("1. FA menu");
    println!("2. Scanner");

    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();

    match trimmed {
        "1" => finite_automata::menu::main_menu(),
        "2" => parse_source_code(),
        _ => println!("Invalid option!")
    }
}
