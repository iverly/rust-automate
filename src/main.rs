use clap::Parser;
use logos::Logos;

use crate::parser::Token;

pub mod grammar;
pub mod parser;
pub mod rules;
pub mod store;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path of the grammar to use
    #[arg(long)]
    grammar: String,

    /// The path of the input file to validate
    #[arg(long)]
    input: String,
}

fn main() {
    // parse the arguments
    let args = Args::parse();

    // parse the grammar at the given path
    let grammar = grammar::Grammar::parse(args.grammar.as_str());
    println!("📚 Grammar to use:\n");
    println!("{}", grammar);

    // create a new store with the rules
    let store = grammar.to_store();

    // read the input file and create the lexer
    let input: String = std::fs::read_to_string(args.input.as_str()).unwrap();
    println!("📝 Input to be analyzed:\n");
    println!("{}", input);
    let lexer = Token::lexer(input.as_str());

    // create a new parser with the store
    let mut parser = crate::parser::Parser::new(store, lexer);

    // parse the input
    let correct = parser.parse();

    match correct {
        true => println!("✅ The input is correct"),
        false => println!("🚫 The input is incorrect"),
    }
}
