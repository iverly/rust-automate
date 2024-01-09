use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use crate::{parser::Token, rules::RuleStep, store::Store};

/// The `Grammar` struct represents a grammar and contains a vector of `GrammarSet` objects.
///
/// Properties:
///
/// * `sets`: The `sets` property is a vector of `GrammarSet` structs.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Grammar {
    sets: Vec<GrammarSet>,
}

impl Grammar {
    /// The `parse` function reads a JSON file containing a grammar and deserializes it into a Rust data
    /// structure.
    ///
    /// Returns:
    ///
    /// The `parse` function is returning an instance of `Self`, which is the type that the function is
    /// defined in.
    pub fn parse() -> Self {
        let grammar = std::fs::read_to_string("./grammar.json").unwrap();
        serde_json::from_str(grammar.as_str()).unwrap()
    }

    /// The `to_store` function converts a given grammar into a store by adding all terminal rule sets
    /// and non-terminal rule sets to the store.
    ///
    /// Returns:
    ///
    /// a `Store` object.
    pub fn to_store(&self) -> Store {
        let mut store = Store::new();

        // add all terminal rule sets to store
        for set in &self.sets {
            let rule_set = set.to_rule_set_without_non_terminal();
            store.add_rule_set(set.name.clone(), rule_set);
        }

        // add all non-terminal rule sets to store
        for set in &self.sets {
            // get all rule sets
            let grammar_rule_sets = set.rules.clone();

            // get the rule set from the store
            let store_rule_set = store.get_rule_set(set.name.as_str()).unwrap();
            let mut index = 0;

            // iterate over all rule sets
            for rule_set in grammar_rule_sets {
                // if the rule does not have a non-terminal, skip it
                if rule_set.non_terminal.is_none() {
                    continue;
                }

                // get non-terminal name
                let non_terminal = rule_set.non_terminal.unwrap();
                // get rule set from store
                let rule_set = store.get_rule_set(non_terminal.as_str()).unwrap();

                // add non-terminal rule set to store
                let store_rule_set = store_rule_set.lock().unwrap();
                store_rule_set.rules[index]
                    .steps
                    .clone()
                    .lock()
                    .unwrap()
                    .append(
                        vec![RuleStep {
                            token: None,
                            next: Some(rule_set),
                        }]
                        .as_mut(),
                    );

                index += 1;
            }
        }

        store
    }
}

/// The `GrammarSet` struct represents a set of grammar rules, with a name and a vector of `GrammarRule`
/// objects.
///
/// Properties:
///
/// * `name`: The `name` property is a string that represents the name of the grammar set.
/// * `rules`: The `rules` property is a vector (dynamic array) of `GrammarRule` objects.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GrammarSet {
    pub name: String,
    pub rules: Vec<GrammarRule>,
}

impl GrammarSet {
    /// The function `to_rule_set_without_non_terminal` converts a rule set by removing non-terminal
    /// symbols.
    pub fn to_rule_set_without_non_terminal(&self) -> crate::rules::RuleSet {
        let mut rules = Vec::new();

        for rule in &self.rules {
            rules.push(rule.to_rule_without_non_terminal());
        }

        crate::rules::RuleSet { rules }
    }
}

/// The `GrammarRule` struct represents a grammar rule with a list of terminals and a non-terminal.
///
/// Properties:
///
/// * `terminals`: A vector of strings representing the terminal symbols in the grammar rule. Terminal
/// symbols are symbols that cannot be further expanded or derived in the grammar.
/// * `non_terminal`: The `non_terminal` property in the `GrammarRule` struct represents a non-terminal
/// symbol in a grammar rule. In formal language theory, a non-terminal symbol is a symbol that can be
/// replaced by a sequence of other symbols according to the rules of a grammar. Non-terminal symbols
/// are typically represented by
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GrammarRule {
    terminals: Vec<String>,
    non_terminal: Option<String>,
}

impl GrammarRule {
    /// The function `to_rule_without_non_terminal` converts a list of terminals into a rule without
    /// non-terminals in Rust.
    pub fn to_rule_without_non_terminal(&self) -> crate::rules::Rule {
        let mut steps = Vec::new();

        for terminal in &self.terminals {
            steps.push(crate::rules::RuleStep {
                token: Some(Token::from_string(terminal)),
                next: None,
            });
        }

        crate::rules::Rule {
            steps: Arc::new(Mutex::new(steps)),
        }
    }
}
