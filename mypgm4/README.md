# Mypgm4 - Homework 5

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes. See deployment for notes on how to deploy the project on a live system.

### How to Run the Program

After downloading the mypgm4 directory, go into the directory and run the program with the following command: "cargo run sample.yaml". This will read in the sample.yaml file, use serve to grab the PDA information and put it into a struct. Then it will check for errors, and generate a .dot file to use in Graphviz. Edit the yaml file to get different results.  

Then enter a 'string' that should be any string that fits into the language based on the PDA made from the yaml file. So, '1001' would be an example for the given yaml file. It will read the string and accept or reject it.

```
Example

cargo run sample.yaml
1001
101
1
'''

## Running the tests

Run the following command in the mypgm4 directory: cargo test

These test checks if a fake file name is called, the program fails, and a test for if the PDA is validated.
