use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use crate::parser::Token;

pub mod rule_c;
pub mod rule_r;

/// The `RuleSet` struct represents a collection of rules.
///
/// Properties:
///
/// * `rules`: The `rules` property is a vector (dynamic array) of `Rule` structs.
#[derive(Debug, Clone)]
pub struct RuleSet {
    pub rules: Vec<Rule>,
}

impl RuleSet {
    /// The function `new` creates a new instance of the `RuleSet` struct with an empty vector of rules.
    ///
    /// Returns:
    ///
    /// A new instance of the `RuleSet` struct is being returned.
    pub fn new(rules: Vec<Rule>) -> RuleSet {
        RuleSet { rules }
    }
}

/// The `Rule` struct represents a set of steps that can be executed in a multi-threaded environment.
///
/// Properties:
///
/// * `steps`: The `steps` property is a vector of `RuleStep` structs, wrapped in an `Arc` and a
/// `Mutex`.
#[derive(Debug, Clone)]
pub struct Rule {
    // We use Arc and Mutex to allow passing to a next step a reference to the another rules
    // instead of a copy of the rules and avoid infinite recursion when we have a loop in the rules
    // definition (e.g. rule 1 -> rule 2 -> rule 3 -> rule 1)
    // This is specific to Rust and is (probably) not needed in other languages
    pub steps: Arc<Mutex<Vec<RuleStep>>>,
}

/// The `RuleStep` struct represents a step in a rule, with an optional token and an optional next rule set.
///
/// Properties:
///
/// * `token`: The `token` property is an optional field that represents a token. Tokens are typically
/// used in parsing and lexical analysis to represent the smallest units of a programming language, such
/// as keywords, identifiers, operators, and literals. In this case, the `token` field is of type
/// `Option<Token>
/// * `next`: The `next` property is an optional field that holds a reference to the next `RuleSet` in a
/// sequence of rules. It is wrapped in a `Box` to allow for dynamic allocation and ownership transfer.
#[derive(Clone)]
pub struct RuleStep {
    pub token: Option<Token>,
    pub next: Option<Arc<Mutex<RuleSet>>>,
}

impl Debug for RuleStep {
    /// The `fmt` function is used to format the `RuleStep` struct for debugging purposes in Rust.
    ///
    /// Arguments:
    ///
    /// * `f`: A mutable reference to a `std::fmt::Formatter` object. This object is used to format the
    /// output.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.next.is_some() {
            return f
                .debug_struct("RuleStep")
                .field("next", &self.next)
                .finish();
        }

        f.debug_struct("RuleStep")
            .field("token", &self.token)
            .finish()
    }
}
