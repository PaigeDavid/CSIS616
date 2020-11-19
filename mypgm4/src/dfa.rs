//! CSIS-616 - Program #4
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

#[derive(Debug, Deserialize)]
pub struct PDA {

    //The state to transition to
    pub state: usize,

    //The symbol to pop off the stack
    pub pop: char,

    //The symbol to push on to the stack
    pub push: char,
}


// *********************************************************************
/// # Deterministic Finite Automata Structure
/// 
/// Create a structure that the YAML files will be deserialized into.
/// Note the use of the `Deserialize` trait
/// 
#[derive(Debug, Deserialize)]
pub struct DFA {

    /// The set of characters comprising the input
    pub alphabet: Vec<char>,

    /// State number (1 relative) for the start state
    pub start: usize,

    /// Set of accept states (1 relative)
    pub accept: Vec<usize>,

    /// Matrix of transitions, rows are states, columns characters in the input
    pub transitions: Vec<Vec<PDA>>,
    
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

        // The number of characters in the input should match the number
        // of columns in each state row

        for (rnum, row) in self.transitions.iter().enumerate() {

            if row.len() != self.alphabet.len() {
                return Err(format!("Wrong number of columns({}) in row {}, should be {}",
                                    row.len(), rnum + 1, self.alphabet.len() ))
            }

        }

        // Validate that all states in the transition table are valid
        for (rnum, row) in self.transitions.iter().enumerate() {
            for (cnum, pda) in row.iter().enumerate() {
                if pda.state as usize >  self.transitions.len() {
                    return Err(format!("Invalid transition state({:?}) in row {}, column {}",
                                        pda, rnum + 1, cnum + 1 ))
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
}