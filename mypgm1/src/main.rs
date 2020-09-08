use std::io::Write;
use std::fs;

fn main() {

    let s = std::env::args().nth(1).expect("error parsing argument");
    let nodes = s.split(",").collect::<Vec<&str>>();

    //Call write_to_file function using the Vec(&str) that was parsed from the command line arguments
    write_to_file(nodes, "peck_graph.dot");
}

fn write_to_file(node_names: Vec<&str>, file_name: &str) {
	let mut accept_state = String::new();
	let mut start_state = String::new();
	let mut previous_state = String::new();
	previous_state.push_str(node_names[0]);
    
    //Add a double circle around the final state to show accepting state
    accept_state.push_str("\nnode [shape = doublecircle]; ");
    accept_state.push_str(node_names[node_names.len()-1]);

    //Starting state
    start_state.push_str("\nqi -> ");
    start_state.push_str(node_names[0]);

    //Create file, add in Dot code
    let mut file = std::fs::File::create(file_name).expect("create failed");
    file.write_all("digraph finite_state_machine {".as_bytes()).expect("write failed");
    file.write_all("\nrankdir=LR;".as_bytes()).expect("write failed");
    file.write_all(accept_state.as_bytes()).expect("write failed");
    file.write_all("\nnode [shape = point ]; qi".as_bytes()).expect("write failed");
    file.write_all("\nnode [shape = circle]".as_bytes()).expect("write failed");
    file.write_all(start_state.as_bytes()).expect("write failed");

    //Iterate through node names
    for node in node_names.iter().skip(1) {
    	let mut state = String::new();
    	state.push_str("\n");
    	state.push_str(&previous_state);
    	state.push_str(" -> ");
    	state.push_str(node);
    	file.write_all(state.as_bytes()).expect("write failed");

    	previous_state.clear();
    	previous_state.push_str(node);
    }

	file.write_all("\n}".as_bytes()).expect("write failed");

	println!("data written to file" );
}

#[test]

fn test_write_to_file() {
	let test_nodes = "a,b,c,d,e,f".split(",").collect::<Vec<&str>>();

	write_to_file(test_nodes, "test_file.dot");

	let file_contents = fs::read_to_string("peck_graph.dot").expect("Something went wrong reading the file");

	let mut test_case = String::new();
	test_case.push_str("digraph finite_state_machine {\nrankdir=LR;\nnode [shape = doublecircle]; f\nnode [shape = point ]; qi\nnode [shape = circle]\nqi -> a\na -> b\nb -> c\nc -> d\nd -> e\ne -> f\n}");

	fs::remove_file("test_file.dot").expect("Something went wrong removing the file");
	assert_eq!(file_contents, test_case);
}



