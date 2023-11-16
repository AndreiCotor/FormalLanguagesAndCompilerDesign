use std::collections::HashSet;
use std::fs::read_to_string;
use crate::finite_automata::fa::FiniteAutomaton;

fn parse_truple(truple: &&str) -> (char, char, char) {
    let item_string: Vec<&str> = truple.split(',').collect();
    let items: Vec<char> = item_string.iter().map(|x| x.chars().nth(0).unwrap()).collect();
    (items[0], items[1], items[2])
}

pub fn parse_fa(file_name: &str) -> FiniteAutomaton {
    let lines: Vec<String> = read_to_string(file_name)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let states_string: Vec<&str> = lines[0].split(',').collect();
    let states: Vec<char> = states_string.iter().map(|x| x.chars().nth(0).unwrap()).collect();

    let final_states_string: Vec<&str> = lines[2].split(',').collect();
    let final_states: HashSet<char> = final_states_string.iter().map(|x| x.chars().nth(0).unwrap()).collect();

    let alphabet_string: Vec<&str> = lines[3].split(',').collect();
    let alphabet: HashSet<char> = alphabet_string.iter().map(|x| x.chars().nth(0).unwrap()).collect();

    let transitions_string: Vec<&str> = lines[4].split(';').collect();
    let transitions: Vec<(char, char, char)> = transitions_string.iter().map(parse_truple).collect();

    FiniteAutomaton::new(states, lines[1].chars().nth(0).unwrap(), final_states, alphabet, transitions)
}