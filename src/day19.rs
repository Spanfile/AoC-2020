use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Rule {
    Character(char),
    All(Vec<usize>),
    Or(Box<Rule>, Box<Rule>),
}

impl Rule {
    fn matches(&self, message: &mut (impl Iterator<Item = char> + Clone), rules: &HashMap<usize, Rule>) -> bool {
        match self {
            Rule::Character(c) => matches!(message.next(), Some(next) if *c == next),
            Rule::All(rl) => rl.iter().all(|r| rules.get(r).unwrap().matches(message, rules)),
            Rule::Or(left, right) => {
                let mut message_clone = message.clone();
                if left.matches(&mut message_clone, rules) {
                    *message = message_clone;
                    true
                } else {
                    right.matches(message, rules)
                }
            }
        }
    }
}

#[aoc_generator(day19)]
pub fn generator(input: &str) -> (HashMap<usize, Rule>, Vec<String>) {
    let (rules, messages) = input.split_once("\n\n").unwrap();
    let messages = messages.lines().map(|l| l.to_owned()).collect();
    let rules = rules
        .lines()
        .map(|l| {
            let (id, rule) = l.split_once(": ").unwrap();

            fn parse_rule(rule: &str) -> Rule {
                if let Some((left, right)) = rule.split_once(" | ") {
                    Rule::Or(Box::new(parse_rule(left)), Box::new(parse_rule(right)))
                } else if let Some(c) = rule.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
                    Rule::Character(c.chars().next().unwrap())
                } else {
                    Rule::All(rule.split(' ').map(|r| r.parse().unwrap()).collect())
                }
            }

            (id.parse().unwrap(), parse_rule(rule))
        })
        .collect();

    (rules, messages)
}

#[aoc(day19, part1)]
pub fn part1(input: &(HashMap<usize, Rule>, Vec<String>)) -> usize {
    let (rules, messages) = input;
    count_matches(rules, messages)
}

#[aoc(day19, part2)]
pub fn part2(input: &(HashMap<usize, Rule>, Vec<String>)) -> usize {
    let (rules, messages) = input;
    let mut rules = rules.clone();
    // 8: 42 | 42 8
    // 11: 42 31 | 42 11 31
    rules.insert(
        8,
        Rule::Or(Box::new(Rule::All(vec![42])), Box::new(Rule::All(vec![42, 8]))),
    );
    rules.insert(
        11,
        Rule::Or(Box::new(Rule::All(vec![42, 31])), Box::new(Rule::All(vec![42, 11, 31]))),
    );

    count_matches(&rules, messages)
}

fn count_matches(rules: &HashMap<usize, Rule>, messages: &[String]) -> usize {
    let rule_0 = rules.get(&0).unwrap();
    messages
        .iter()
        .filter(|m| {
            let mut chars = m.chars();
            let matches = rule_0.matches(&mut chars, rules);
            if chars.next().is_none() {
                matches
            } else {
                false
            }
        })
        .count()
}
