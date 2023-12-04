use std::sync::{Arc, Mutex};

use crate::parser::Token;

use super::{Rule, RuleSet, RuleStep};

// R -> rate <num> <num> <num> R | rate <num> <num> <num> D | rate <num> <num> <num> C | None
pub fn construct_r() -> RuleSet {
    let base: Vec<RuleStep> = vec![
        RuleStep {
            token: Some(Token::Options),
            next: None,
        },
        RuleStep {
            token: Some(Token::Number),
            next: None,
        },
        RuleStep {
            token: Some(Token::Number),
            next: None,
        },
        RuleStep {
            token: Some(Token::Number),
            next: None,
        },
    ];

    let c1: Rule = Rule {
        steps: Arc::new(Mutex::new(base.clone())),
    };

    let c2: Rule = Rule {
        steps: Arc::new(Mutex::new(base.clone())),
    };

    let c3: Rule = Rule {
        steps: Arc::new(Mutex::new(base)),
    };

    let c4: Rule = Rule {
        steps: Arc::new(Mutex::new(vec![])),
    };

    RuleSet {
        rules: vec![c1, c2, c3, c4],
    }
}
