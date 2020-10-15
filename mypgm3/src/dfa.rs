//! CSIS-616 - Program #3
//! 
//! Most of the program was originally made by: Ralph W. Crosby PhD.
//! Edited and added to by: Paige Peck
//!
//! Newly added functions:
//! process - takes in string from command line, and checks if it is accepted or rejected by the DFA
//! 
//! Definition and methods associated with the yaml format dfa structure.
//! 
//! Ralph W. Crosby PhD.
//! 
use serde::{Deserialize};

// *********************************************************************
/// # Deterministic Finite Automata Structure
/// 
/// Create a structure that the YAML files will be deserialized into.
/// Note the use of the `Deserialize` trait
/// 
#[derive(Debug, Deserialize)]
pub struct DFA {

    /// The set of characters comprising the alphabet
    pub alphabet: Vec<char>,

    /// State number (1 relative) for the start state
    pub start: usize,

    /// Set of accept states (1 relative)
    pub accept: Vec<usize>,

    /// Matrix of transitions, rows are states, columns characters in the alphabet
    pub transitions: Vec<Vec<usize>>,
    
}

// *********************************************************************
/// Implement the methods of the DFA structure
impl DFA {

    /// Create and return a DFA on the heap
    /// 
    /// Load the .yaml file specified into a DFA structure
    /// on the heap and return a point to it via a Box.

    pub fn new_from_file(filename: &str) -> Box<DFA> {

        let f = std::fs::File::open(filename)
                    .expect("Unable to open input");

        // Deserialize into the heap and return the pointer
        Box::new(serde_yaml::from_reader(f)
                    .expect("Unable to parse yaml") )

    }

    /// Validate the correctness of the DFA
    pub fn validate(&self) -> Result<&DFA, String> {

        // The number of characters in the alphabet should match the number
        // of columns in each state row

        for (rnum, row) in self.transitions.iter().enumerate() {

            if row.len() != self.alphabet.len() {
                return Err(format!("Wrong number of columns({}) in row {}, should be {}",
                                    row.len(), rnum + 1, self.alphabet.len() ))
            }

        }

        // Validate that all states in the transition table are valid
        for (rnum, row) in self.transitions.iter().enumerate() {
            for (cnum, state) in row.iter().enumerate() {

                if *state as usize >  self.transitions.len() {
                    return Err(format!("Invalid transition state({}) in row {}, column {}",
                                        state, rnum + 1, cnum + 1 ))
                }
    
            }
        }

        // The start and accept states must be valid
        if self.start as usize > self.transitions.len() {
            return Err(format!("Start state({}), is not valid", self.start))
        }

        for acc_state in self.accept.iter() {
            if *acc_state as usize  > self.transitions.len() {
                return Err(format!("Accept state({}), is not valid", acc_state))
            }
        }

        Ok(self)
    }

    //Process the input string, print out the processing steps, print whether it accepts or rejects the string
    pub fn process(&self, inputstring: &str) -> Result<&DFA, String> {
        //Get the current state, which is the start state. Set next as current for future use. 
        let mut current = self.start;
        let mut next = current;

        //Iterate through the input string
        //Check each character to make ssure if is in the alphabet
        //Set the next state by iterating over the current transition, getting the input and next state
        //If the input matches the symbol, set the next state based on the if statement results
        //Print out the transition process
        //Set current as next, then repeat
        for symbol in inputstring.chars() {
            if !self.alphabet.contains(&symbol) {
                return Err(format!("{} is not a valid symbol in the language. Rejected.", symbol))
            }

            //next state selection
            for (input, state) in self.transitions[current-1].iter().enumerate() {
                //check which symbol matches the alphabet, 
                //then set next as the state if it matches
                if self.alphabet[input] == symbol {
                    next = *state;
                }
            }

            println!{"𝛿(q{},{}) → q{}", current, symbol, next};

            //set current state as next state
            current = next;
        }
        
        //If the final state is in an accept state, accept. Otherwise, reject.
        if self.accept.contains(&current) {
            println!("{} is accepted.", inputstring);
        }
        else {
            println!("{} is rejected.", inputstring);
        }
        Ok(self)
    }

}