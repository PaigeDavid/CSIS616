//! CSIS-616 - Program #3
//! 
//! Definition and methods associated with the state graph structure.
//! 
//! Ralph W. Crosby PhD.

use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;

use crate::dfa;

// *********************************************************************
/// Helper type alias to simplify references to a state
/// 
/// - The `RefCell` allows interior mutability for the state
///
///     This allows us to change the contents of the `_State` instance.
/// 
/// - The `Rc` adds reference counting aowing multiple references to the state.
///
///     This is required so that we can clone the references
type StateRef<'a> = Rc<RefCell<_State<'a>>>;

// *********************************************************************
/// The internal representation of a state.
/// 
/// Contains the actual fields associated with a state in the graph
struct _State<'a> {
    /// State name
    name: String,
    /// Is this an accepting state?
    accept: bool,
    /// Map of symbols to states defining the transitions
    adjacent: HashMap<char, StateRef<'a>>,
}

// *********************************************************************
/// Public representation of a state node
/// 
/// A tuple structure containing the reference to the actual _State
///
/// This allows us to tack implementation methods on the state since
/// we can't add implementations to a type alias.
pub struct State<'a>(StateRef<'a>);

// *********************************************************************
impl<'a> State<'a> {

    /// Creata a new state, initially with no transitions
    pub fn new(name: String, accept: bool) -> State<'a> {
        let state = _State { name: name, 
                             accept: accept, 
                             adjacent: HashMap::new() };
        State(Rc::new(RefCell::new(state)))
    }

    /// Create a transition on a state
    pub fn add_transition(&self, new_char: char, other: &State<'a>) {
        self.0.borrow_mut().adjacent.insert(new_char, other.0.clone());
    }

}

// *********************************************************************
/// Debug output for a state
impl<'a> fmt::Debug for State<'a> {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let state = self.0.borrow();

        write!(f, "state {{ name: {}, acccept: {}, transitions: [", 
                  state.name, 
                  state.accept).unwrap();

        for k in state.adjacent
                        .iter()
                        .map(|v| v.0)
                        .collect::<Vec<_>>() {
            write!(f, "( {}: {} )", 
                      k, 
                      state.adjacent[k].borrow().name ).unwrap();
        };

        write!(f, "]")

    }

}

// *********************************************************************
/// Definition of the state graph structure
pub struct Graph<'a> {
    alphabet: Vec<char>,
    start_state: StateRef<'a>,
    pub states: Vec<State<'a>>,
}

// *********************************************************************
impl<'a> Graph<'a> {

    /// Create a new graph from a dfa structure
    pub fn new_from_dfa(dfa: &dfa::DFA) -> Graph<'a> {

        // Create the states
        let states = (1..dfa.transitions.len() + 1)
                        .map(|i| State::new(format!("q{}", i), 
                                            dfa.accept.contains(&i))                            )
                        .collect::<Vec<State>>();

        // Add the transitions
        states
            .iter()
            .enumerate()
            .for_each(|s| for (j, t) in dfa.transitions[s.0].iter().enumerate() {
                                s.1.add_transition(dfa.alphabet[j], &states[*t-1]);
                           });

        Graph{alphabet: dfa.alphabet.clone(),
              start_state: states[dfa.start-1].0.clone(), 
              states: states}

    }

}

// *********************************************************************
/// Use display output to generate the Graphviz structure
impl<'a> fmt::Display for Graph<'a> {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        // Write the header
        writeln!(f, "digraph {{").unwrap();
        writeln!(f, "\trankdir=LR;").unwrap();

        // Write the shapes to use for the symbols
        writeln!(f, "\tnode [shape=point]; start;").unwrap();
        writeln!(f, "\tnode [shape=doublecircle]; {};",
                    self.states
                        .iter()
                        .filter(|s| s.0.borrow().accept)
                        .map(|s| s.0.borrow().name.clone())
                        .collect::<Vec<String>>()
                        .join(", ")).unwrap();
        writeln!(f, "\tnode [shape=circle];").unwrap();

        // Write the edges
        writeln!(f, "\tstart -> {};", self.start_state.borrow().name).unwrap();

        self.states.iter().for_each(|s|
            s.0.borrow().adjacent.iter().for_each(|a| 
                writeln!(f, "\t{} -> {} [label=\"{}\"];", 
                            s.0.borrow().name,
                            a.1.borrow().name,
                            a.0).unwrap()));

        write!(f, "}}\n")
    }
    
}

// *********************************************************************
/// Debug output for the graph
impl<'a> fmt::Debug for Graph<'a> {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        f.debug_struct("graph")
            .field("alphabet", &self.alphabet)
            .field("start_state", &self.start_state.borrow().name)
            .field("states", &self.states)
            .finish()

    }

}