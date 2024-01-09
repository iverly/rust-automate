use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::rules::{Rule, RuleSet};

/// The `Store` struct is a container for a shared mutable reference to a `RuleSet` object, wrapped in
/// an `Arc` and `Mutex`.
///
/// Properties:
///
/// * `c`: `c` is a public field of type `Arc<Mutex<RuleSet>>`.
#[derive(Debug, Clone, Default)]
pub struct Store {
    pub sets: HashMap<String, Arc<Mutex<RuleSet>>>,
}

impl Store {
    /// The `new` function creates a new instance of the `Store` struct with an empty `sets` HashMap.
    ///
    /// Returns:
    ///
    /// The `new` function is returning an instance of the `Store` struct.
    pub fn new() -> Self {
        Store::default()
    }

    /// The function `add_rule_set` adds a rule set to a collection.
    ///
    /// Arguments:
    ///
    /// * `name`: A `String` representing the name of the rule set.
    /// * `rule_set`: The `rule_set` parameter is of type `RuleSet`.
    pub fn add_rule_set(&mut self, name: String, rule_set: RuleSet) {
        self.sets.insert(name, Arc::new(Mutex::new(rule_set)));
    }

    /// The function `get_rule_set` returns an `Option` containing a cloned reference to a `RuleSet` if
    /// it exists in the `sets` HashMap.
    ///
    /// Arguments:
    ///
    /// * `name`: The `name` parameter is a reference to a string (`&str`).
    ///
    /// Returns:
    ///
    /// The function `get_rule_set` returns an `Option` containing an `Arc` wrapped in a `Mutex` of type
    /// `RuleSet`.
    pub fn get_rule_set(&self, name: &str) -> Option<Arc<Mutex<RuleSet>>> {
        self.sets.get(name).cloned()
    }

    /// The function `add_non_terminal` adds a non terminal to the store.
    // pub fn add_non_terminal(&mut self) {
    //     // add r to c1
    //     let ref_c1_to_r = self.r.clone();
    //     self.c.lock().unwrap().rules[0]
    //         .steps
    //         .clone()
    //         .lock()
    //         .unwrap()
    //         .append(
    //             vec![RuleStep {
    //                 token: None,
    //                 next: Some(ref_c1_to_r),
    //             }]
    //             .as_mut(),
    //         );

    //     // add r to r1
    //     let ref_r1_to_r = self.r.clone();
    //     self.r.lock().unwrap().rules[0]
    //         .steps
    //         .clone()
    //         .lock()
    //         .unwrap()
    //         .append(
    //             vec![RuleStep {
    //                 token: None,
    //                 next: Some(ref_r1_to_r),
    //             }]
    //             .as_mut(),
    //         );

    //     // add c to r2
    //     let ref_r2_to_c = self.c.clone();
    //     self.r.lock().unwrap().rules[1]
    //         .steps
    //         .clone()
    //         .lock()
    //         .unwrap()
    //         .append(
    //             vec![RuleStep {
    //                 token: None,
    //                 next: Some(ref_r2_to_c),
    //             }]
    //             .as_mut(),
    //         );
    // }

    /// The function `get_all_rules` returns a vector containing all the rules from the store.
    ///
    /// Returns:
    ///
    /// The `get_all_rules` function returns a `Vec<Rule>`.
    pub fn get_all_rules(&self) -> Vec<Rule> {
        let mut rules = Vec::new();

        for rule_set in self.sets.values() {
            let rule_set = rule_set.lock().unwrap();
            for rule in &rule_set.rules {
                rules.push(rule.clone());
            }
        }

        rules
    }
}
