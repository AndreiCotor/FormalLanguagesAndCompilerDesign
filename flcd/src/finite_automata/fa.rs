use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::rc::Rc;

#[derive(Debug)]
pub struct Edge {
    destination: Rc<RefCell<Node>>,
    transition: char
}

#[derive(Debug, Default)]
pub struct Node {
    pub id: char,
    pub transitions: Vec<Edge>,
    pub is_final_state: bool
}

#[derive(Debug)]
pub struct FiniteAutomaton {
    pub nodes: Vec<Rc<RefCell<Node>>>,
    pub start: Rc<RefCell<Node>>,
    pub alphabet: HashSet<char>
}

impl FiniteAutomaton {
    pub fn new(states: Vec<char>,
               start_state: char,
               final_states: HashSet<char>,
               alphabet: HashSet<char>,
               transitions: Vec<(char, char, char)>) -> Self {

        let mut nodes = Vec::new();
        let mut start = Vec::new();
        let mut nodes_id_map = HashMap::new();
        for state in states {
            let node = Node {
                id: state,
                transitions: Vec::new(),
                is_final_state: final_states.contains(&state)
            };

            let node_wrapper = Rc::new(RefCell::new(node));
            nodes.push(node_wrapper.clone());
            nodes_id_map.insert(state, node_wrapper.clone());

            if start_state == state {
                start.push(node_wrapper);
            }
        }

        for transition in transitions {
            let source = nodes_id_map.get(&transition.0).unwrap().clone();
            let destination = nodes_id_map.get(&transition.1).unwrap().clone();

            if !alphabet.contains(&transition.2) {
                panic!("Transition not in alphabet!");
            }

            let edge = Edge {
                destination,
                transition: transition.2,
            };

            source.borrow_mut().transitions.push(edge);
        }

        FiniteAutomaton {
            nodes,
            alphabet,
            start: start[0].clone()
        }
    }

    pub fn is_dfa(&self) -> bool {
        for node in &self.nodes {
            let mut transition_set = HashSet::new();
            for edge in &node.borrow().transitions {

                if transition_set.contains(&edge.transition) {
                    return false;
                }

                transition_set.insert(edge.transition);
            }
        }

        true
    }

    pub fn check_match(&self, label: &str) -> Result<bool, NotDFAError> {
        let mut node = self.start.clone();
        for ch in label.chars() {
            let mut matching = 0;
            let mut next_node = Default::default();
            for edge in &node.borrow().transitions {
                if edge.transition == ch {
                    matching += 1;
                    next_node = edge.destination.clone();
                }
            }

            if matching == 0 {
                return Ok(false);
            }

            if matching > 1 {
                return Err(NotDFAError)
            }

            node = next_node;
        }

        Ok(node.clone().borrow().is_final_state)
    }
}

impl Display for FiniteAutomaton {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut res = String::new();
        res.push_str("States: ");
        for node in &self.nodes {
            res.push(node.borrow().id);
            res.push_str(" ");
        }

        res.push_str("\n");
        res.push_str("Input state: ");
        res.push(self.start.borrow().id);

        res.push_str("\nOutput states: ");
        for node in &self.nodes {
            let borrow = node.borrow();
            if borrow.is_final_state {
                res.push(borrow.id);
                res.push_str(" ");
            }
        }

        res.push_str("\nAlphabet: ");
        for ch in &self.alphabet {
            res.push(*ch);
            res.push_str(" ");
        }

        res.push_str("\nTransitions:\n");
        for node in &self.nodes {
            for edge in &node.borrow().transitions {
                res.push_str(&format!("{} -{}-> {}\n",
                                      node.borrow().id,
                                      edge.transition,
                                      edge.destination.borrow().id
                ))
            }
        }

        write!(f, "{}", res)
    }
}

#[derive(Debug)]
pub struct NotDFAError;
