use aoc_runner_derive::{aoc, aoc_generator};
use std::{collections::HashMap, iter::Peekable};

#[derive(Debug, Clone)]
pub enum Rule {
    Character(char),
    All(Vec<usize>),
    Or(Box<Rule>, Box<Rule>),
}

impl Rule {
    fn matches(
        &self,
        message: &mut Peekable<impl Iterator<Item = char> + Clone>,
        rules: &HashMap<usize, Rule>,
        flipped: bool,
    ) -> bool {
        match self {
            Rule::Character(c) => match message.peek() {
                Some(n) if c == n => {
                    message.next();
                    true
                }
                _ => false,
            },
            Rule::All(rl) => {
                if !flipped {
                    rl.iter()
                        .all(|r| rules.get(r).unwrap().matches(message, rules, flipped))
                } else {
                    rl.iter()
                        .rev()
                        .all(|r| rules.get(r).unwrap().matches(message, rules, flipped))
                }
            }
            Rule::Or(left, right) => {
                let (left, right) = if flipped { (right, left) } else { (left, right) };
                let mut message_clone = message.clone();
                if left.matches(&mut message_clone, rules, flipped) {
                    *message = message_clone;
                    true
                } else {
                    right.matches(message, rules, flipped)
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
    let rule_42 = rules.get(&42).unwrap();
    let rule_31 = rules.get(&31).unwrap();

    let mut matched = 0;
    'msg: for msg in messages {
        let mut chars_rev = msg.chars().rev().peekable();

        let mut matched_31 = 0;
        while rule_31.matches(&mut chars_rev, rules, true) {
            matched_31 += 1;
        }

        for _ in 0..matched_31 {
            if !rule_42.matches(&mut chars_rev, rules, true) {
                continue 'msg;
            }
        }

        while rule_42.matches(&mut chars_rev, rules, true) {}

        if chars_rev.next().is_some() {
            continue 'msg;
        }

        matched += 1;
    }

    matched
}

fn count_matches(rules: &HashMap<usize, Rule>, messages: &[String]) -> usize {
    let rule_0 = rules.get(&0).unwrap();
    messages
        .iter()
        .filter(|m| {
            let mut chars = m.chars().peekable();
            let matches = rule_0.matches(&mut chars, rules, false);
            if chars.next().is_none() {
                matches
            } else {
                false
            }
        })
        .count()
}
