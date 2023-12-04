use std::sync::{Arc, Mutex};

use crate::parser::Token;

use super::{Rule, RuleSet, RuleStep};

// C -> contact <id> <id> <num> <num> D | contact <id> <id> <num> <num> R | None
pub fn construct_c() -> RuleSet {
    let base: Vec<RuleStep> = vec![
        RuleStep {
            token: Some(Token::Contact),
            next: None,
        },
        RuleStep {
            token: Some(Token::Identifier),
            next: None,
        },
        RuleStep {
            token: Some(Token::Identifier),
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
        steps: Arc::new(Mutex::new(base)),
    };

    let c3: Rule = Rule {
        steps: Arc::new(Mutex::new(vec![])),
    };

    RuleSet {
        rules: vec![c1, c2, c3],
    }
}
