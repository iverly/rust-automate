use std::sync::{Arc, Mutex};

use logos::{Lexer, Logos};

use crate::{rules::RuleStep, store::Store};

// const INPUT: &str = r#"contact A B 20 32
// contact A B 20 32"#;

const INPUT: &str = r#"contact A B 20 32
delay 10 20 32"#;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
pub enum Token {
    #[token("contact")]
    Contact,

    #[token("rate")]
    #[token("delay")]
    Options,

    #[regex("[a-zA-Z]+")]
    Identifier,

    #[regex("[0-9]+")]
    Number,
}

/// The `Parser` struct is used for parsing code and contains a lexer and a store.
///
/// Properties:
///
/// * `lexer`: The `lexer` property is an instance of the `Lexer` struct. It is a lexer that takes a
/// static lifetime reference to a `Token` type. A lexer is responsible for breaking down a stream of
/// characters into a sequence of tokens, which can then be processed by the parser.
/// * `store`: The `store` property is an instance of the `Store` struct. It is used to store and manage
/// data during the parsing process.
pub struct Parser {
    lexer: Lexer<'static, Token>,
    store: Store,
}

impl Parser {
    /// The function `new` creates a new instance of the `Parser` struct with an empty lexer.
    ///
    /// Arguments:
    ///
    /// * `input`: The `input` parameter is a reference to a static string (`&'static str`). It represents
    /// the input that the parser will be working with.
    ///
    /// Returns:
    ///
    /// A new instance of the `Parser` struct is being returned.
    pub fn new(store: Store) -> Parser {
        let lexer = Token::lexer(INPUT);
        Parser { lexer, store }
    }

    /// The `parse` function takes a string input and initializes the lexer with it.
    ///
    /// Arguments:
    ///
    /// * `input`: A reference to a string that represents the input to be parsed.
    pub fn parse(&mut self) -> bool {
        let rules = self.store.get_all_rules();

        // try all rules one by one
        // if one of them matches, return true
        // if none of them matches, return false
        for rule in rules {
            // clone the lexer because it is consumed after each call to next()
            let mut lexer = self.lexer.clone();

            // process the rule (recursively)
            let result = Self::process(&mut lexer, rule.steps, 0, None);

            // if the rule matches, return true
            if result {
                return true;
            }
        }

        // if none of the rules matches, return false
        false
    }

    /// The function `process` takes a lexer, a list of rule steps, an index, and a next token, and
    /// recursively processes the tokens according to the rule steps.
    ///
    /// Arguments:
    ///
    /// * `lexer`: A mutable reference to a `Lexer` object, which is used to tokenize input.
    /// * `steps`: `steps` is an `Arc<Mutex<Vec<RuleStep>>>` which represents a shared mutable reference to
    /// a vector of `RuleStep` structs. The `Arc` type is used for reference counting and allows multiple
    /// threads to have ownership of the same data. The `Mutex` type is used
    /// * `index`: The `index` parameter represents the current index of the step being processed in the
    /// list of steps. It is used to keep track of the progress in the rule matching process.
    /// * `next_token`: The `next_token` parameter is an optional `Token` that represents the next token to
    /// be processed. It is used to pass the token from the previous step to the current step when the
    /// current step is a reference to another rule. If `next_token` is `Some(token)`, it means
    ///
    /// Returns:
    ///
    /// The function `process` returns a `bool` which indicates whether the rule matching process
    #[warn(clippy::only_used_in_recursion)]
    pub fn process(
        _lexer: &mut Lexer<'static, Token>,
        steps: Arc<Mutex<Vec<RuleStep>>>,
        index: usize,
        next_token: Option<Token>,
    ) -> bool {
        // get the next token from the lexer or use the one passed as argument
        // this is mandatory because the lexer is consumed after each call to next()
        // and if we find on the previous call that the next step is a reference to another rule
        // so we didn't consume the token and lexer didn't not allow us to call previous()
        let token = match next_token {
            Some(t) => Some(Ok(t)),
            None => _lexer.next(),
        };

        // get the number of steps
        let steps_size = steps.lock().unwrap().len();

        // no more tokens and no more steps => nothing to do
        if token.is_none() && index == steps_size {
            return true;
        }

        // we still have tokens or steps => something is wrong
        if token.is_none() || index == steps_size {
            return false;
        }

        // get the current step
        let step: RuleStep = steps.lock().unwrap()[index].clone();

        match token {
            Some(t) => {
                if step.token.is_none() && step.next.is_some() {
                    // if the current step is a reference to another rule
                    Self::process(_lexer, step.next.unwrap().steps, 0, Some(t.unwrap()))
                } else if t.unwrap() == step.token.unwrap() {
                    // if the current step is a token
                    Self::process(_lexer, steps, index + 1, None)
                } else {
                    // if the current step is a token and it doesn't match the current token
                    false
                }
            }
            None => false,
        }
    }
}
