use std::sync::{Arc, Mutex};

use crate::parser::Token;

use super::{Rule, RuleSet, RuleStep};

// R -> rate <num> <num> <num> R | rate <num> <num> <num> C
// D -> delay <num> <num> <num> R | delay <num> <num> <num> C

// R is basically the same as D, but with a rate token instead of a delay token
// so in the token definition, we can just have a single token for both
// and then in the rule definition, we can have a single rule for both
// this is what we're doing here with the token `options`
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

    let r1: Rule = Rule {
        steps: Arc::new(Mutex::new(base.clone())),
    };

    let r2: Rule = Rule {
        steps: Arc::new(Mutex::new(base)),
    };

    RuleSet {
        rules: vec![r1, r2],
    }
}
