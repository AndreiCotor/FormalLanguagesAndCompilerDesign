use std::io;
use crate::finite_automata::fa::FiniteAutomaton;
use crate::finite_automata::parser::parse_fa;

fn read_fa() -> FiniteAutomaton {
    println!("File path: ");

    let mut input_text= String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();

    parse_fa(trimmed)
}

fn check_label(fa: &FiniteAutomaton) {
    if !fa.is_dfa() {
        println!("This is not a DFA");
        return;
    }

    println!("Label: ");

    let mut input_text= String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    if fa.check_match(trimmed).unwrap() {
        println!("It's a match!");
    }
    else {
        println!("Not a match :(");
    }
}

pub fn main_menu() {
    let mut fa = None;
    loop {
        println!("Choose an option:");
        println!("1. Read FA");
        println!("2. Display FA");
        println!("3. For a DFA, verifies if a sequence is accepted by the FA");

        let mut input_text = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");

        let trimmed = input_text.trim();

        match trimmed {
            "1" => fa = Some(read_fa()),
            "2" => println!("{}", fa.as_ref().unwrap()),
            "3" => check_label(fa.as_ref().unwrap()),
            _ => println!("Invalid option!")
        }
    }
}