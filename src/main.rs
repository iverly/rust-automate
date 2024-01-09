use crate::parser::Parser;

pub mod grammar;
pub mod parser;
pub mod rules;
pub mod store;

fn main() {
    let grammar = grammar::Grammar::parse();

    // create a new store with the rules
    let store = grammar.to_store();

    // create a new parser with the store
    let mut parser = Parser::new(store);

    // parse the input
    let result = parser.parse();

    // print the result
    println!("Result: {}", result);
}
