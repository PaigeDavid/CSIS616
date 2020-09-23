//! CSIS-616 Homework #2
//! 
//! Paige Peck (paigepeck@hotmail.com)

use std::io::Write;
use serde::{Deserialize};

// The program does the following tasks:
//
//	1. Reads in a YAML format DFA.
//
//	2. Function "check_for_errors" checks the DFA data structure for:
//		a. The transitions are defined for all symbols and states
//		b. All states referenced in the transition table are valid / existing states
//		c. The start and accept states are valid, i.e. esixting states based on the transition table
//	The Function then returns a result object containing an empty OK if valid or an Error with a message if invalid
//	The main program exits on an error.
//
//	3. Function "write_to_file" writes a DOT file to display the DFA with the following:
//		a. Interconnected set of nodes
//		b. Empty node that points to the first state
//		c. Each node containts a collection of symbols and references to the targets of those symbols
//		d. Accept states that are double circled.
//	4. Function "print_graph_debug_contents" prints out the contents of the graph in debug format
//
//	
//	To run program, use the following command in the correct directory in the terminal:
//	
//	cargo run sample.yaml

//Struct for generating the DFA implementation using serde
#[derive(Debug, Deserialize)]
struct DFA {
	alphabet: Vec<char>,
	start: u32,
	accept: Vec<u32>,
	transitions: Vec<Vec<u32>>,

	#[serde(default)]
	n_states: usize
}

fn main() {

	//Get the filename argument as a String
	let filename = get_filename(std::env::args());

	//Load the yaml file getting a Box pointing to a DFA
	// instance on the heap
	let mut d = DFA::new_from_file(&filename);

	//Call the "check_for_errors" function
	//Exit main if Error is returned and print error, otherwise continue. 
	match d.check_for_errors() {
		Ok(()) => println!(""),
		Err(e) => {
			println!("{:?}", e);
			std::process::exit(1);
		}
	}

	//Compute the total number of states. Then print out to the terminal the contents of the graph.
	d.compute_states();
	d.print("Graph: ");

	//Call the function to write the Graphviz definition and print it out to a .dot file
	d.write_to_file();

}

//Return the filename passed as the first parameter
fn get_filename(args: std::env::Args) -> String {
	
	//Get the arguments as a vector
	let args: Vec<String> = args.collect();

	//Make sure only one argument was passed
	if args.len() != 2 {
		writeln!(std::io::stderr(), "Usage: hw1 dfafile").unwrap();
		std::process::exit(1);

	}

	args[1].to_string()
}

impl DFA {
	//create and return a DFA on the heap
	//
	//Load the .yaml file specified into a DFA structure
	//and return it on the heap

	fn new_from_file(filename: &str) -> Box<DFA> {

		let f = std::fs::File::open(filename).expect("Unable to open input");

		Box::new(serde_yaml::from_reader(f).expect("Unable to open yaml"))
	}

	fn compute_states(&mut self) {
		self.n_states = self.transitions.len();
	}

	fn print(&self, s: &str) {
		println!("{}: {:?}", s, self);
	}


	fn check_for_errors(&mut self) -> Result<(), &'static str> {
		//Check if transitions are defined for all symbols and states. If there are more or less
		//		symbols than states, then it is not valid. Return an error
		let total_symbols = self.alphabet.len();
		for transition in self.transitions.iter() {
			if transition.len() != total_symbols
			{
				return Err("Error. Transitions are not defined for all symbols.");
			}
		}

		//Check if all states referenced in the transition table are valid (they refer to an existing state)
		//		If there is a state that is a higher value than the number of transitions, or a value of 0 or less
		//		return an error
		let state_count = self.transitions.len() as u32;
		for transition in self.transitions.iter() {
			for state in transition.iter() {
				if state > &state_count || state <= &(0 as u32) {
					return Err("Error. A state in the transition table is not valid.");
				}
			}
		}

		//Check if the start state and accept states are valid. If the start/accept states values are higher
		//		than the total amount of states, or less than or equal to 0, return an error.
		if self.start > state_count || self.start <= 0  {
			return Err("Error. The start state is not a valid state.");
		}

		for accept_state in self.accept.iter() {
			if accept_state > &state_count || accept_state <= &(0 as u32) {
				return Err("Error. One or more accept states are not valid states.");
			}
		}
		Ok(())
	}

	//This function generates  a .DOT file called "peck_mypgm2_graph.dot".
	//The DOT file containts the properly formatted Graphviz definition of the graph represented
	//		by the DFA that was read in from the yaml file
	fn write_to_file(&mut self) {
		//Create the accepting states as Strings to push on the stack all the 
		//		accepting states in the DFA. Also set the state_state and previous_state for 
		//		purposes of knowing where the state is being drawn and transitioning to/from
		let mut accepting = String::new();
		let mut start_state = String::new();
		let mut previous_state = String::new();
		//Start with the start state as the "previous state" and push it onto the stack
		previous_state.push_str(&self.start.to_string());

		//For all accepting states, push them onto the stack along with the
		// 		double circle setting for the node to show it is accepting in the graph. 
		for accept_state in self.accept.iter() {
			accepting.push_str("\nnode [shape = doublecircle]; ");
			accepting.push_str(&accept_state.to_string());
		}

		//Push the start state along with the empty node pointing to it onto the stack
		start_state.push_str("\nqi -> ");
    	start_state.push_str(&self.start.to_string());

    	//Create file, add in Dot code
    	//The DOT code writes to the Grpahiviz definition style while popping off the top of the stacks
    	//		the accepting and start state information.
	    let mut file = std::fs::File::create("peck_mypgm2_graph.dot").expect("create failed");
	    file.write_all("digraph finite_state_machine {".as_bytes()).expect("write failed");
	    file.write_all("\nrankdir=LR;".as_bytes()).expect("write failed");
	    file.write_all(accepting.as_bytes()).expect("write failed");
	    file.write_all("\nnode [shape = point ]; qi".as_bytes()).expect("write failed");
	    file.write_all("\nnode [shape = circle]".as_bytes()).expect("write failed");
	    file.write_all(start_state.as_bytes()).expect("write failed");

	    //For loop goes through the transition vector while also storing the current index.
	    //		Inside each transition venctor, it uses the alphabet symbols to loop through
	    //		and push to the stack what the transition for previous_state to the next state will be.
	    //		It also adds the label for the transition from the alphabet. 
	    //		After all this is done, it writes to the file this line for the Graphviz definition.
	    //		Once the alphabet has been run through, then it sets the new previous_state
	    for (state_index, transition) in self.transitions.iter().enumerate() {
	    	for (i, symbol) in self.alphabet.iter().enumerate() {
				let mut state = String::new();
		    	state.push_str("\n\t");
		    	state.push_str(&previous_state);
		    	state.push_str(" -> ");
		    	state.push_str(&transition[i].to_string());
		    	state.push_str(" [ label = \"");
		    	state.push_str(&symbol.to_string());
		    	state.push_str("\"];");
		    	file.write_all(state.as_bytes()).expect("write failed");
	    	}	

	    	previous_state.clear();
	    	previous_state.push_str(&(state_index+2).to_string());
	    }

	    //Print the closing for the graph
	    file.write_all("\n}".as_bytes()).expect("write failed");
	}

}

#[cfg(test)]
mod test {

	use super::*;

	//This test is used to make sure that it fails when it can't find the filename given. 
	#[test]
	#[should_panic]
	fn test1() {
		let test_file_name = "testFile_that_isnt_real.dot";
		DFA::new_from_file(&test_file_name);
	}
}