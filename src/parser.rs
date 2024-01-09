use std::sync::{Arc, Mutex};

use logos::{Lexer, Logos};

use crate::{
    rules::{Rule, RuleStep},
    store::Store,
};

// Valid input
// const INPUT: &str = r#"contact A B 20 32
// rate 10 20 30
// delay 10 20 30
// rate 10 20 30
// contact A B 20 32
// rate 10 20 30"#;

// Invalid input
const INPUT: &str = r#"contact A B 20 32"#;

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

    End,
}

impl Token {
    /// The function `from_string` takes a string as input and returns a corresponding token based on
    /// the string value.
    ///
    /// Arguments:
    ///
    /// * `s`: The parameter `s` is of type `&str`, which means it is a reference to a string slice.
    ///
    /// Returns:
    ///
    /// a value of type `Token`.
    pub fn from_string(s: &str) -> Token {
        match s {
            "Contact" => Token::Contact,
            "Rate" => Token::Options,
            "Delay" => Token::Options,
            "Identifier" => Token::Identifier,
            "Number" => Token::Number,
            "End" => Token::End,
            _ => panic!("Invalid token"),
        }
    }
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

    /// The `parse` function retrieves all rules from the store, creates a rule set, and processes it using
    /// the lexer.
    ///
    /// Returns:
    ///
    /// The `parse` function is returning a boolean value.
    pub fn parse(&mut self) -> bool {
        let rules = self.store.get_all_rules();
        Self::process_rule_set(&mut self.lexer, rules, 0, None, false)
    }

    /// The function `process_rule_set` takes a lexer, a set of rules, an index, and a next token, and
    /// tries each rule one by one until it finds a match, returning true if a match is found and false
    /// otherwise.
    ///
    /// Arguments:
    ///
    /// * `_lexer`: A mutable reference to a `Lexer` object.
    /// * `rules`: A vector of Rule structs. Each Rule struct contains a set of steps to be processed.
    /// * `index`: The `index` parameter represents the current index in the input stream that the lexer
    /// is processing. It is used to keep track of the progress of the lexer as it matches tokens
    /// against the input.
    /// * `next_token`: An optional parameter that represents the next token in the input stream. It is
    /// used to determine if a rule matches based on the current token and the next token.
    /// * `end`: A boolean value that indicates whether the lexer has reached the end of the input stream.
    ///
    /// Returns:
    ///
    /// The function `process_rule_set` returns a boolean value. It returns `true` if one of the rules
    /// in the `rules` vector matches, and `false` if none of the rules matches.
    pub fn process_rule_set(
        _lexer: &mut Lexer<'static, Token>,
        rules: Vec<Rule>,
        index: usize,
        next_token: Option<Token>,
        end: bool,
    ) -> bool {
        // try all rules one by one
        // if one of them matches, return true
        // if none of them matches, return false
        for rule in rules {
            // clone the lexer because it is consumed after each call to next()
            let mut lexer = _lexer.clone();

            // process the rule (recursively)
            let result = Self::process(
                &mut lexer,
                rule.steps.clone(),
                index,
                next_token.clone(),
                end,
            );

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
    /// that the previous step was a reference to another rule, and the current step is a token.
    /// * `end`: The `end` parameter is a boolean value that indicates whether the lexer has reached the
    /// end of the input stream.
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
        mut end: bool,
    ) -> bool {
        // clone the steps because we need to use it after we drop the lock
        let steps_cloned = steps.lock().unwrap().clone();
        drop(steps);

        // get the next token from the lexer or use the one passed as argument
        // this is mandatory because the lexer is consumed after each call to next()
        // and if we find on the previous call that the next step is a reference to another rule
        // so we didn't consume the token and lexer didn't not allow us to call previous()
        let token = match next_token {
            Some(t) => Some(Ok(t)),
            None => match _lexer.next() {
                Some(t) => Some(t),
                None => match end {
                    true => None,
                    false => {
                        end = true;
                        Some(Ok(Token::End))
                    }
                },
            },
        };

        // get the number of steps
        let steps_size = steps_cloned.len();

        // no more tokens and no more steps => nothing to do
        if (token.is_none() || token == Some(Ok(Token::End))) && index == steps_size {
            return true;
        }

        // we still have tokens or steps => something is wrong
        if token.is_none() || index == steps_size {
            return false;
        }

        // get the current step
        let step: RuleStep = steps_cloned[index].clone();
        match token {
            Some(t) => {
                if step.token.is_none() && step.next.is_some() {
                    // trick to avoid consuming the lock
                    let temp = step.next.unwrap();
                    let temp2 = temp.lock().unwrap();
                    let rules = temp2.clone().rules;
                    drop(temp2);
                    drop(temp);

                    // if the current step is a reference to another rules set
                    Self::process_rule_set(_lexer, rules, 0, Some(t.unwrap()), end)
                } else if t.unwrap() == step.token.unwrap() {
                    // if the current step is a token
                    Self::process(
                        _lexer,
                        Arc::new(Mutex::new(steps_cloned)),
                        index + 1,
                        None,
                        end,
                    )
                } else {
                    // if the current step is a token and it doesn't match the current token
                    false
                }
            }
            None => false,
        }
    }
}
