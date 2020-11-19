//! CSIS-616 - Program #3
//! 
//! Some parts were originally made by: Ralph W. Crosby PhD.
//! Edited and added to by: Paige Peck
//! 
//! 
//! Process a yaml format deterministic finite automaton producing
//! - A textual representation of the internal state graph
//! - A Graphviz `.dot` file representing the graph
//! 
//! # Usage
//! 
//! ```
//! cargo run regex
//! ```
//! where: `regex` is a series of symbols that will generate a DFA and decide if input
//! is accepted or rejected by the regex
//! 
//! # Output
//! 
//! To `stderr`: Debug display of the internal graph structure
//! 
//! To `stdout`: Graphviz definitions of the graph structure



use std::io;
use std::io::prelude::*;
use std::io::Write;


// *********************************************************************
/// # Deterministic Finite Automata Structure
struct DFA {

    /// The set of characters comprising the alphabet
    alphabet: Vec<char>,

    /// State number (1 relative) for the start state
    start: usize,

    /// Set of accept states (1 relative)
    accept: Vec<usize>, //will need to be Vec<usize> when multiple accept states are implemented

    /// Matrix of transitions, rows are states, columns characters in the alphabet
    transitions: Vec<Vec<usize>>,
    
}

//State based representation of the DFA version of the RegEx
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
	transitions: Vec<usize>
}

struct Transitions {
	chars: char,

	state: usize
}

fn main() {

	//Get and validate the RegEx on the command line
	let regex = get_regex(std::env::args());

	let dfa = DFA::new_from_regex(&regex);

	//Create the dfa structure based on in RegEx entered from the command line
    let state_graph = StateGraph::new_from_dfa(&dfa);

	//eprintln!("{:?}", state_graph);

    state_graph.write_graphviz();



	// Process through the input until end of file (cntl-z) is encountered
    state_graph.process();
}

// *********************************************************************
/// Return the RegEx passed as the first parameter
fn get_regex(args: std::env::Args) -> String {

    // Get the arguments as a vector
    let args: Vec<String> = args.collect();

    // Make sure only one argument was passed
    if args.len() != 2 {
        writeln!(std::io::stderr(), "Usage: cargo run 'regex'")
            .unwrap();
        std::process::exit(1);
    }
    
    args[1].to_string()
    
}

// *********************************************************************
/// Implement the methods of the DFA structure
impl DFA {

	//Create and return a DFA on the heap
	//Generate the DFA from the given regex
	fn new_from_regex(regex: &str) -> Box<DFA> {

		//Setup the regex as the language / alphabet of the dfa
		//Remove any duplicate word characters
		let mut l = regex.replace("|", "");
		l = l.replace("+", "");
		l = l.replace("*", "");

		//Creates a language Vec<char> without the operators in it and pushing the sigma symbol for alphabet purposes
		let mut language: Vec<char> = l.chars().collect();
		language.sort();
		language.dedup();
		language.push('Σ');

		let final_state = l.len()+1;

		//Create a near blank dfa object, with 1 being start state, accept state being the final state
		// which is calculated based on the length of the regex length + 1
		let mut dfa = Box::new(DFA{alphabet: language,
									start: 1,
									accept: [final_state].to_vec(),
									transitions: vec![] });


		//Set current and next state to traverse through the graph as we create the transition matrix. 
		let mut current_state = 1;
		let mut next_state = 2;

		//Create the Transitions Struct to save any transitions characters. These are characters that would
		// need to be cycled back to. First character and second state will always start this off. 
		let mut transitions: Vec<Transitions> = Vec::new();
		let t = Transitions{chars: regex.chars().next().unwrap(),
											state: 2};
		transitions.push(t);

		//Create a previous_char character for | and * operators
		let mut previous_char = regex.chars().next().unwrap();

		//Traverse through the regex string, reading characters and deciding what to do depending on the character.
		for c in regex.chars() {
			let mut states: Vec<usize> = Vec::new();


			//Checks if previous char was a | operator. 
			//If so, save the current character as a transition or cycle character
			//Also fixes any previous transition state
			if previous_char == '|' {
				for (n, a) in dfa.alphabet.iter().enumerate() {
					if *a == c {
						dfa.transitions[0][n] = next_state;
					}
				}
				let j = Transitions{chars: c, state: next_state};
				transitions.push(j);
			}

			//Same as above, just with the * operator.
			if previous_char == '*' {
				let j = Transitions{chars: c, state: next_state};
				transitions.push(j);
			}

			//Operator '|': Implemented - single and multiple | operators are working
			//Multiple types of symbols are untested and could produce varying results
			//Checks if character is | operator. If so, save the final state as an accept state, reset
			//current state back to 1, and set previous_char as |
			if c == '|' {

				let final_bar_state = dfa.transitions.len()+1;

				let mut final_bar_state_count: Vec<usize> = Vec::new();

				dfa.accept.push(final_bar_state);
				for _a in dfa.alphabet.iter() {
					final_bar_state_count.push(final_bar_state);
				}
				dfa.transitions.push(final_bar_state_count);
				current_state = 1;

				previous_char = '|';
			}
			//Operator '+': Implemented - single works, multiple is funky, almost working
			//Removes the previous transition matrix to remake it with updated states
			//Fix to the multiple + operators I believe is using a for loop to go through the entire transitions vec
			// but I have ran out of time to get that working. 
			else if c == '+' {

				dfa.transitions.remove(dfa.transitions.len()-1);

				next_state -= 1;
				current_state -= 1;

				for a in dfa.alphabet.iter() {
					if a == &previous_char {
						states.push(next_state);
					} else {
						if *a == transitions[0].chars {
							states.push(transitions[0].state);
						} else {
							states.push(1);
						}
					}
				}

				dfa.transitions.push(states);
				next_state += 1;
				current_state += 1;
			}
			//Operator '*': Implemented - Single and multiple * operators are working. Something funky happens with the more characters
			// added into the regex, especially after a *. Not time to check it. Very close to getting this part fixed, most of it works
			//Similar to + operator, remove previous transition to replace it with new one. 
			// Step back 2 states for next and current to allow for proper transition. Push necessary states.
			// Potential fix is similar to + operator with iterating over transitions instead of just checking index 0.
			//At the end, add 2 to current state to get back, and set previous_char as *
			else if c == '*' {

				dfa.transitions.remove(dfa.transitions.len()-1);
				let mut pushed_forward = false;
				next_state -= 2;
				current_state -= 2;

				for a in dfa.alphabet.iter() {
					if a == &previous_char {
						next_state += 1;
						states.push(next_state);
					} else if *a == 'Σ' {
						states.push(1);
					} else {
						if *a == transitions[0].chars {
							states.push(transitions[0].state);
						} else if !pushed_forward {
							next_state += 1;
							states.push(next_state);
							pushed_forward = true;
						} else {
							states.push(1);
						}
					}
				}

				dfa.transitions.push(states);
				current_state += 2;
				previous_char = '*';
			}
			//All word character symbols: Implemented
			//Allows for any character that is in the language to be added in, checks if there is a transition/cycle
			//to be made, set the state as that before pushing. If it is not a transition, push to state 1
			//if sigma symbol, push to state 1
			else if c != 'Σ'
			{
				
				for a in dfa.alphabet.iter() {
					let mut was_transition = false;
					if c == *a {
						states.push(next_state);
					}
					else { 
						for i in 0..transitions.len() {
							if *a == transitions[i].chars {
								states.push(transitions[i].state);
								was_transition = true;
							}
						}

						if was_transition == false {
							if previous_char == '*' && *a != 'Σ' {
								states.push(1);
								previous_char = c;
							} else {
								states.push(1);
							}
						}
					}
				}

				if previous_char != '|' {
					dfa.transitions.push(states);
				}

				next_state += 1;
				current_state += 1;

				previous_char = c;
			}
		}

		//Go back through and fix any transitions that weren't marked properly 
		//    (i.e. | transitions to state 2 from state 4 if applicable)
		for i in 0..dfa.transitions.len() {
			for n in 0..dfa.transitions[i].len() {
				if n < dfa.transitions[i].len() - 1 && dfa.transitions[i][n] == 1 {
					for c in 0..transitions.len() {
						if dfa.alphabet[n] == transitions[c].chars {
							dfa.transitions[i][n] = transitions[c].state;
						}
					}
				}
			}
		}

		//Set final state as a cycle for transition matrix. If 3 states, push [3,3,3]
		let mut final_state_count: Vec<usize> = Vec::new();
		for _alphabet in dfa.alphabet.iter() {
			final_state_count.push(final_state);
		}
		dfa.transitions.push(final_state_count);


		dfa
	}


}

// *********************************************************************
// Implement the methods of the DFA structure
impl StateGraph<> {

	/// Create a state graph from a DFA structure
    fn new_from_dfa(dfa: &DFA) -> Box<StateGraph> {

        // Create an empty graph object
        let mut graph = Box::new(StateGraph{alphabet: dfa.alphabet.clone(), 
                                            start_state: dfa.start - 1,
                                            states: vec!() });

        // Look through the transition table building state objects
        for row in dfa.transitions.iter() {
            let mut v = Box::new(State{accept_state: false, transitions: vec!()});
            for col in row {
                v.transitions.push(col-1);
            } 
            graph.states.push(v);
        }    

        // Set the accept states
        for astate in dfa.accept.iter() {
            graph.states[*astate - 1].accept_state = true;
        }

        graph

    }

    /// Execute the graph on a sentence
    /// Return Err if a character not in the alphabet is encountered
    /// Return Ok and a bool indicating accept (true) or reject (false)
    fn test_sentence(&self, sentence: &str) -> Result<bool, String> {

        let mut state = self.start_state;

        //Full alphabet to test against for sigma character
        let full_alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz0123456789 ".chars().collect();

        for ch in sentence.chars() {

        	//Check if character is a word character. Accept it if it is and change it to the 'Σ' symbol for matching purposes
    		let mut c = ch;
    		if !self.alphabet.contains(&c) && full_alphabet.contains(&c) {
    			c = 'Σ';
    		}
            let state_no = match self.alphabet.iter().position(|v| *v == ch || *v == c) {
                Some(t) => t,
                None => return Err(format!("Character <{}> does not have a transition", ch))
            };

            print!("δ(q{}, {}) → ", state+1, ch);
            state = self.states[state].transitions[state_no];
            println!("(q{})", state+1);

        }

        Ok(self.states[state].accept_state)
    }


	fn write_graphviz(&self) {

        println!("digraph {{");
        println!("\trankdir=LR;");
        println!("\tnode [shape=point]; start;");
        
        for (n, state) in self.states.iter().enumerate() {
            if state.accept_state {
                println!("\tnode [shape=doublecircle]; q{};", n+1);
            }
        }
        
        println!("\tnode [shape=circle];");
        println!("\tstart -> q{}", self.start_state+1);
        for (n, state) in self.states.iter().enumerate() {

            for (i, ch) in self.alphabet.iter().enumerate() {
                println!("\tq{} -> q{} [label=\"{}\"]", n+1, state.transitions[i] + 1, ch);
            }

        }
        println!("}}");

    }

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


}

#[cfg(test)]
mod test {

    use super::*;
    
    //This test is used to make sure that it creates a graphviz file
    #[test]
    fn test1() {
        let dfa = DFA::new_from_regex("a*b");

		//Create the dfa structure based on in RegEx entered from the command line
	    let state_graph = StateGraph::new_from_dfa(&dfa);

	    state_graph.write_graphviz();
    }
}