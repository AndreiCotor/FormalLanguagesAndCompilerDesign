use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;
use crate::finite_automata::fa::FiniteAutomaton;
use crate::pif::pif::{PIF, PIFEntry};
use crate::scanner::char_type::CharType;
use crate::symbol_table::symbol_table::{SymbolTable, SymbolTableType};
use crate::symbol_table::symbol_table_position::SymbolTablePosition;
use crate::token::token_manager::TokenManager;

const SEPARATOR_CHARS: &str = "()[]{}:; .";
const OPERATOR_CHARS: &str = "+-*/%=<>!";

fn scan_source_code(file_name: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(file_name).unwrap();
    Ok(io::BufReader::new(file).lines())
}

fn is_separator_char(ch: char) -> bool {
    SEPARATOR_CHARS.contains(ch)
}

fn is_operator_char(ch: char) -> bool {
    OPERATOR_CHARS.contains(ch)
}

fn extract_tokens_from_line(line: &str) -> Vec<String> {
    let mut rez = Vec::new();
    let mut last_token = String::new();
    let mut last_char_type = CharType::OTHER;
    for ch in line.chars() {
        let current_char_type = if is_separator_char(ch) {
            CharType::SEPARATOR
        } else if is_operator_char(ch) {
            CharType::OPERATOR
        } else {
            CharType::OTHER
        };

        if current_char_type == last_char_type
            && (current_char_type != CharType::SEPARATOR || ch == '.') {
            last_token.push(ch);
        }
        else {
            if !last_token.is_empty() && last_token != " " {
                rez.push(last_token.clone());
            }

            last_token.clear();
            last_token.push(ch);
            last_char_type = current_char_type;
        }
    }

    if !last_token.is_empty() && last_token != " " {
        rez.push(last_token);
    }

    rez
}

fn is_int_constant(token: &str) -> bool {
    if token == "0" {
        return true;
    }
    let re = Regex::new(r"^-?[1-9]\d*$").unwrap();
    re.is_match(token)
}

fn is_string_constant(token: &str) -> bool {
    let re = Regex::new(r#"^".*"$"#).unwrap();
    re.is_match(token)
}

fn is_id(token: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z_]+$").unwrap();
    re.is_match(token)
}

fn add_id_or_constant_to_pif(pif: &mut PIF, token: &str, symbol_table: &mut SymbolTable, line_num: i32, int_const_fa: &FiniteAutomaton, id_fa: &FiniteAutomaton) {
    let st_pos = if int_const_fa.check_match(token).unwrap() {
        let pos = symbol_table.add_int_const(token.parse().unwrap());
        SymbolTablePosition {
            tp: SymbolTableType::INT,
            bucket: pos.0,
            item: pos.1,
        }
    }
    else if is_string_constant(token) {
        let pos = symbol_table.add_string_const(token.to_owned());
        SymbolTablePosition {
            tp: SymbolTableType::STRING,
            bucket: pos.0,
            item: pos.1,
        }
    }
    else if id_fa.check_match(token).unwrap() {
        let pos = symbol_table.add_id(token.to_owned());
        SymbolTablePosition {
            tp: SymbolTableType::ID,
            bucket: pos.0,
            item: pos.1,
        }
    }
    else {
        panic!("Lexical error on line {}!", line_num);
    };

    pif.add(PIFEntry {
        token: -1,
        st_pos: Some(st_pos)
    });
}

pub fn process_source_code(file_name: &str, token_manager: &TokenManager, int_const_fa: &FiniteAutomaton, id_fa: &FiniteAutomaton) {
    let mut res = PIF::new();
    let mut symbol_table = SymbolTable::new(10);

    if let Ok(lines) = scan_source_code(file_name) {
        let mut line_num = 1;
        for line in lines {
            if let Ok(ip) = line {
                let tokens = extract_tokens_from_line(&ip);

                for token in tokens {
                    match token_manager.get_token_code(&token) {
                        None => add_id_or_constant_to_pif(&mut res, &token, &mut symbol_table, line_num, int_const_fa, id_fa),
                        Some(code) =>
                            res.add(PIFEntry{
                                token: code,
                                st_pos: None,
                            })
                    }
                }
            }
            line_num += 1;
        }
    }

    println!("PIF:");
    println!("{}", res);
    println!();
    println!("Symbol table:");
    println!("{}", symbol_table);
}

