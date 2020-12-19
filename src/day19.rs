use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[derive(Debug)]
pub enum Rule {
    Character(char),
    SingleOther(usize),
    TwoOthers(usize, usize),
    Or(Box<Rule>, Box<Rule>),
}

impl Rule {
    fn matches(&self, message: &mut (impl Iterator<Item = char> + Clone), rules: &HashMap<usize, Rule>) -> bool {
        match self {
            Rule::Character(c) => matches!(message.next(), Some(next) if *c == next),
            Rule::SingleOther(rule) => rules.get(rule).unwrap().matches(message, rules),
            Rule::TwoOthers(left, right) => {
                rules.get(left).unwrap().matches(message, rules) && rules.get(right).unwrap().matches(message, rules)
            }
            Rule::Or(left, right) => {
                let mut message_clone = message.clone();
                left.matches(message, rules) || right.matches(&mut message_clone, rules)
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
                } else if let Some((first, second)) = rule.split_once(' ') {
                    Rule::TwoOthers(first.parse().unwrap(), second.parse().unwrap())
                } else {
                    Rule::SingleOther(rule.parse().unwrap())
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
    let rule_0 = rules.get(&0).unwrap();
    messages
        .iter()
        .filter(|m| rule_0.matches(&mut m.chars(), rules))
        .count()
}

#[aoc(day19, part2)]
pub fn part2(input: &(HashMap<usize, Rule>, Vec<String>)) -> i32 {
    0
}
