# Mypgm2 - Homework 2 

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes. See deployment for notes on how to deploy the project on a live system.

### How to Run the Program

After downloading the mypgm2 directory, go into the directory and run the program with the following command: "cargo run sample.yaml". This will read in the sample.yaml file, use serve to grab the DFA information and put it into a struct. Then it will check for errors, and generate a .dot file to use in Graphviz. Edit the yams file to get different results.  

```
Example

cargo run sample.yaml
```

## Running the tests

Run the following command in the mypgm2 directory: 
```
cargo test
```

This test checks if a fake file name is called, the program fails.
