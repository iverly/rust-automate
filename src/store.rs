use std::sync::{Arc, Mutex};

use crate::rules::{Rule, RuleSet, RuleStep};

/// The `Store` struct is a container for a shared mutable reference to a `RuleSet` object, wrapped in
/// an `Arc` and `Mutex`.
///
/// Properties:
///
/// * `c`: `c` is a public field of type `Arc<Mutex<RuleSet>>`.
#[derive(Debug, Clone)]
pub struct Store {
    pub c: Arc<Mutex<RuleSet>>,
    pub r: Arc<Mutex<RuleSet>>,
}

impl Store {
    /// The function `new` creates a new instance of the `Store` struct with two `RuleSet` objects.
    ///
    /// Arguments:
    ///
    /// * `c`: The parameter `c` is of type `RuleSet`. It represents a set of rules that will be stored in
    /// the `Store` struct.
    /// * `r`: The parameter `r` is a `RuleSet` object.
    ///
    /// Returns:
    ///
    /// The `new` function is returning an instance of the `Store` struct.
    pub fn new(c: RuleSet, r: RuleSet) -> Self {
        Store {
            c: Arc::new(Mutex::new(c)),
            r: Arc::new(Mutex::new(r)),
        }
    }

    /// The function `add_non_terminal` adds a non terminal to the store.
    pub fn add_non_terminal(&mut self) {
        // add r to c1
        let ref_c1_to_r = self.r.clone();
        self.c.lock().unwrap().rules[0]
            .steps
            .clone()
            .lock()
            .unwrap()
            .append(
                vec![RuleStep {
                    token: None,
                    next: Some(ref_c1_to_r),
                }]
                .as_mut(),
            );

        // add r to r1
        let ref_r1_to_r = self.r.clone();
        self.r.lock().unwrap().rules[0]
            .steps
            .clone()
            .lock()
            .unwrap()
            .append(
                vec![RuleStep {
                    token: None,
                    next: Some(ref_r1_to_r),
                }]
                .as_mut(),
            );

        // add c to r2
        let ref_r2_to_c = self.c.clone();
        self.r.lock().unwrap().rules[1]
            .steps
            .clone()
            .lock()
            .unwrap()
            .append(
                vec![RuleStep {
                    token: None,
                    next: Some(ref_r2_to_c),
                }]
                .as_mut(),
            );
    }

    /// The function `get_all_rules` returns a vector containing all the rules from the store.
    ///
    /// Returns:
    ///
    /// The `get_all_rules` function returns a `Vec<Rule>`.
    pub fn get_all_rules(&self) -> Vec<Rule> {
        let mut rules = vec![];
        rules.extend(self.c.lock().unwrap().rules.clone());
        rules.extend(self.r.lock().unwrap().rules.clone());
        rules
    }
}
