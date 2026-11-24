use std::collections::HashSet;

struct Transition {
    symbol: char,
    destination: usize,
}

struct FiniteAutomaton {
    alphabet: HashSet<char>,
    transitions: Vec<Vec<Transition>>,
    initial_state: usize,
    accepting_states: Vec<bool>,
}
