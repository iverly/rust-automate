use crate::{parser::Parser, rules::rule_c::construct_c, store::Store};
use rules::rule_r::construct_r;

pub mod parser;
pub mod rules;
pub mod store;

fn main() {
    // create a new store with the rules
    let mut store = Store::new(construct_c(), construct_r());
    // add non terminal to the store
    store.add_non_terminal();

    // create a new parser with the store
    let mut parser = Parser::new(store);

    // parse the input
    let result = parser.parse();

    // print the result
    println!("Result: {}", result);
}
