# Project 1

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes. See deployment for notes on how to deploy the project on a live system.

### How to Run the Program

After downloading the project1 directory, go into the directory and run the program with the following command: "cargo run 'regex'". This will read in the regex, use serve to grab the DFA information and put it into a struct. Then it will check for errors, and generate a .dot file to use in Graphviz.

Then enter a 'string' that should be any string that fits into the language based on the DFA made from the regex.  It will read the string and accept or reject it.

```
Example

cargo run a*b
b
ab
testb
'''

## Running the tests

Run the following command in the project1 directory: cargo test

This test checks if a graphviz file can be generated from the a regex.


### Caveats with the program

1) Single and Multiple * operators does work, but * and any other operator, and it does some funky output. Was close to resolving this. Suggestion on where the issue is in the comments of program

2) Single and multiple | operators work, but similar to the * operator, multiple symbols has varied results. 

4) Make sure with | symbol, the regex is in quotes:

```
Example

cargo run 'qq|aa|5'
qq
aa
5
'''

3) Single + working, multiple + working in some instances. Almost got working as well. Possible fix explained in comments

4) ( ), \w, and \d are not implemented at all. Ran out of time. 