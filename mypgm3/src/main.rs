//! CSIS-616 - Program #3
//! 
//! Most of the program was originally made by: Ralph W. Crosby PhD.
//! Edited and added to by: Paige Peck
//!
//! Newly added functions:
//! Test cases
//! dfa.process()
//! 
//! 
//! Process a yaml format deterministic finite automaton producing
//! - A textual representation of the internal state graph
//! - A Graphviz `.dot` file representing the graph
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
//! 
//! where: `filename` is a yaml file containing the DFA definition
//! 
//! # Output
//! 
//! To `stderr`: Debug display of the internal graph structure
//! 
//! To `stdout`: Graphviz definitions of the graph structure
use std::io::Write;

mod dfa;
mod graph;

// *********************************************************************
fn main() {

    // Get and validate the filename and input string on the command line
    let (filename, inputstring) = get_arguments(std::env::args());

    // Load the yaml file getting a Box pointing to a DFA
    // instance on the heap
    let dfa = dfa::DFA::new_from_file(&filename);

    // Validate the DFA
    dfa.validate().expect("Validation Failure:");

    // Get a state structure for the DFA
    let graph = graph::Graph::new_from_dfa(&dfa);

    // Write the debug version of the graph to stderr
    eprintln!("{:?}", graph);

    // Write the Graphviz version of the graph to stdout
    println!{"{}", graph};

    //Process the string, print out the processing steps, and print out whether it is accepted or rejected.
    dfa.process(&inputstring).expect("Processing Failure:");
}

// *********************************************************************
/// Return the filename and input string passed as the first and second parameter
fn get_arguments(args: std::env::Args) -> (String, String) {

    // Get the arguments as a vector
    let args: Vec<String> = args.collect();

    // Make sure only one argument was passed
    if args.len() != 3 {
        writeln!(std::io::stderr(), "Usage: hw1 dfafile")
            .unwrap();
        std::process::exit(1);
    }
    
    (args[1].to_string(), args[2].to_string())
    
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
    
    //This test is used to make sure that it accepts an acceptable language.
    #[test]
    fn test2() {
        let dfa = dfa::DFA::new_from_file("sample.yaml");
        
        dfa.process("xyxxyx").expect("Processing Failure:");
    }

    //This test is used to make sure that it rejects a language with incorrect symbols
    #[test]
    #[should_panic]
    fn test3() {
        let dfa = dfa::DFA::new_from_file("sample.yaml");
        
        dfa.process("xxy101yx").expect("Processing Failure:");
    }
}