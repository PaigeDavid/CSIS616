# Mypgm3 - Homework 3

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes. See deployment for notes on how to deploy the project on a live system.

### How to Run the Program

After downloading the mypgm3 directory, go into the directory and run the program with the following command: "cargo run sample.yaml 'string'". This will read in the sample.yaml file, use serve to grab the DFA information and put it into a struct. Then it will check for errors, and generate a .dot file to use in Graphviz. Edit the yaml file to get different results.  

The 'string' should be any string that fits into the language based on the DFA made from the yaml file. So, 'xyxy' would be an example for the given yaml file. It will read the string and accept or reject it.

```
Example
```
cargo run sample.yaml xyxy

## Running the tests

Run the following command in the mypgm3 directory: cargo test

These test checks if a fake file name is called, the program fails, if a string is accepted, and if a string is rejected.
