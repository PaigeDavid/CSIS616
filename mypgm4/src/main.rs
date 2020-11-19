//! CSIS-616 - Program #4
//! 
//! Most of the program was originally made by: Ralph W. Crosby PhD.
//! Edited and added to by: Paige Peck
//! 
//! Process a yaml format deterministic finite automaton producing
//! - A textual representation of the internal state graph
//! - A Graphviz `.dot` file representing the graph
//! 
//! 
//! # Usage
//! 
//! ```
//! idomatic_dfa filename
//! ```
//! or
//! ```
//! cargo run filename
//! ```
//! a
//! where: `filename` is a yaml file containing the PDA definition
//! 
//! # Output
//! 
//! To `stderr`: Debug display of the internal graph structure
//! 
//! To `stdout`: Graphviz definitions of the graph structure
use std::io;
use std::io::prelude::*;
use std::io::Write;

mod dfa;

//State based representation of the PDA
struct StateGraph {

    /// The set of characters comprising the alphabet
    alphabet: Vec<char>,

    /// State number for the start state
    start_state: usize,

    /// Vector of state objects
    states: Vec<Box<State>>

}

//Definition of a single state
struct State {

    //Is this an accept state
    accept_state: bool,

    //Set of transitions
    transitions: Vec<dfa::PDA>
}

// *********************************************************************
fn main() {

    // Get and validate the filename and input string on the command line
    let filename = get_arguments(std::env::args());

    // Load the yaml file getting a Box pointing to a DFA
    // instance on the heap
    let dfa = dfa::DFA::new_from_file(&filename);

    // Validate the DFA
    dfa.validate().expect("Validation Failure:");

    // Get a state structure for the DFA
    let state_graph = StateGraph::new_from_dfa(&dfa);

    //eprintln!("{:?}", state_graph);

    state_graph.write_graphviz();


    // Process through the input until end of file (cntl-z) is encountered
    state_graph.process();



}

// *********************************************************************
/// Return the filename and input string passed as the first and second parameter
fn get_arguments(args: std::env::Args) -> String {

    // Get the arguments as a vector
    let args: Vec<String> = args.collect();

    // Make sure only one argument was passed
    if args.len() != 2 {
        writeln!(std::io::stderr(), "Usage: cargo run sample.yaml")
            .unwrap();
        std::process::exit(1);
    }
    
    args[1].to_string()
    
}

// *********************************************************************
// Implement the methods of the PDA structure
impl StateGraph<> {

    /// Create a state graph from a DFA structure
    fn new_from_dfa(dfa: &dfa::DFA) -> Box<StateGraph> {

        // Create an empty graph object
        let mut graph = Box::new(StateGraph{alphabet: dfa.alphabet.clone(), 
                                            start_state: dfa.start - 1,
                                            states: vec!() });

        // Look through the transition table building state objects
        for row in dfa.transitions.iter() {
            let mut v = Box::new(State{accept_state: false, transitions: vec!()});
            for col in row {
                let p = dfa::PDA{state: col.state-1, pop: col.pop, push: col.push};
                v.transitions.push(p);
            } 
            graph.states.push(v);
        }    

        // Set the accept states
        for astate in dfa.accept.iter() {
            graph.states[*astate - 1].accept_state = true;
        }

        graph

    }

    // *********************************************************************
    /// Generate the Graphviz structure
    fn write_graphviz(&self) {

        // Write the header
        println!("digraph {{");
        println!("\trankdir=LR;");
        println!("\tnode [shape=point]; start;");
        
        // Write accept states and other states
        for (n, state) in self.states.iter().enumerate() {
            if state.accept_state {
                println!("\tnode [shape=doublecircle]; q{};", n+1);
            }
        }
        println!("\tnode [shape=circle];");

        // Write the edges
        println!("\tstart -> q{}", self.start_state+1);
        for (n, state) in self.states.iter().enumerate() {

            for (i, ch) in self.alphabet.iter().enumerate() {
                println!("\tq{} -> q{} [label=\"{}, {} -> {}\"]", n+1, state.transitions[i].state + 1, ch, state.transitions[i].pop, state.transitions[i].push);
            }

        }
        println!("}}");

    }

    //Process the input string, print out the processing steps, print whether it accepts or rejects the string
    fn process(&self) {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {

            // Get the line out of the Result, should never error
            let sentence = &line.unwrap();
            println!("Processing sentence <{}>", sentence);

            match self.test_sentence(sentence) {
                Ok(b) => println!("{}", 
                                  if b {"Accept"} else {"Reject"}),
                Err(s) => println!("Error processing sentence: {}", s)
            }

        }
    }

    /// Execute the graph on a sentence
    /// Return Err if a character not in the alphabet is encountered
    /// Return Ok and a bool indicating accept (true) or reject (false)
    fn test_sentence(&self, sentence: &str) -> Result<bool, String> {

        let mut state = self.start_state;

        let mut stack: Vec<char> = Vec::new();
        
        //Check if epsilon has a transition from the first state
        let n = match self.alphabet.iter().position(|v| *v == 'Ɛ') {
            Some(t) => t,
            None => return Err(format!("Ɛ does not have a transition"))
        };

        //Accept the empty string
        if sentence.len() == 0 {
            return Ok(self.states[state].accept_state);
        }

        //Push the stack symbol onto the stack
        //Traverse to state 2 and print out the steps
        stack.push('$');
        print!("δ(q{}, {}) → ", state+1, 'Ɛ');
        state = self.states[state].transitions[n].state;
        println!("(q{}), stack: {:?}", state+1, stack);

        //For determining when to start popping off the stack
        let half_length = sentence.len() / 2;

        //Go throug hthe sentence, pushing onto the stack the symbols associated with the symbol
        //based on the PDA from the yaml file generation. Once halfway through the sentence,
        // move to the next state (do an extra character read if odd length), then start popping off the stack.
        //If a symbol is read without a matching transition, reject. 
        for (i, ch) in sentence.chars().enumerate() {

            if i == half_length {

                let p = match self.alphabet.iter().position(|v| *v == 'Ɛ') {
                    Some(t) => t,
                    None => return Err(format!("Ɛ does not have a transition"))
                };

                //Handle odd number palindromes
                if (sentence.len() % 2) > 0 {
                    println!("here");
                    let d = match self.alphabet.iter().position(|v| *v == '0' || *v == '1') {
                        Some(t) => t,
                        None => return Err(format!("Ɛ does not have a transition"))
                    };

                    stack.push(ch);
                    print!("δ(q{}, {}) → ", state+1, ch);
                    state = self.states[state].transitions[d].state;
                    println!("(q{}), stack: {:?}", state+1, stack);
                }


                //stack.push('$');
                print!("δ(q{}, {}) → ", state+1, 'Ɛ');
                state = self.states[state].transitions[p].state;
                println!("(q{}), stack: {:?}", state+1, stack);
            }

            //Check if character is a word character.
            let state_no = match self.alphabet.iter().position(|v| *v == ch) {
                Some(t) => t,
                None => return Err(format!("Character <{}> does not have a transition", ch))
            };
            
            if self.states[state].transitions[state_no].pop != 'Ɛ' {
                if self.states[state].transitions[state_no].pop == stack[stack.len()-1] {
                    stack.remove(stack.len()-1);
                }
                else {
                    return Err(format!("Top of stack ({}) doesn't match pop character ({})", stack[stack.len()-1], self.states[state].transitions[state_no].pop));
                }

            }

            if self.states[state].transitions[state_no].push != 'Ɛ' {
                stack.push(self.states[state].transitions[state_no].push);
            }
            print!("δ(q{}, {}) → ", state+1, ch);
            state = self.states[state].transitions[state_no].state;
            println!("(q{}), stack: {:?}", state+1, stack);

        }

        //Check if stack symbol is only remaining character on stack, if so accept, otherwise reject. 
        if stack[stack.len()-1] == '$' {
            stack.remove(stack.len()-1);
            print!("δ(q{}, {}) → ", state+1, 'Ɛ');
            state = self.states[state].transitions[n].state;
            println!("(q{}), stack: {:?}", state+1, stack);
        }
        else {
            return Err(format!("Top of stack is not $"));
        }

        Ok(self.states[state].accept_state)
    }
}


#[cfg(test)]
mod test {

    use super::*;

    //This test is used to make sure that it fails when it can't find the filename given. 
    #[test]
    #[should_panic]
    fn test1() {
        dfa::DFA::new_from_file("testFile_that_isnt_real.dot");
    }  

    //This test is used to validate the PDA
    #[test]
    fn test2() {
        let pda = dfa::DFA::new_from_file("sample.yaml");
        
        // Validate the DFA
        pda.validate().expect("Validation Failure:");
    }
}