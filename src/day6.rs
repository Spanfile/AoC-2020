use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[aoc_generator(day6)]
pub fn generator(input: &str) -> Vec<(usize, HashMap<char, usize>)> {
    input
        .split("\n\n")
        .map(|s| {
            let mut chars = HashMap::new();
            let mut people = 1;
            for c in s.chars() {
                if c == '\n' {
                    people += 1;
                    continue;
                }
                *chars.entry(c).or_insert(0) += 1;
            }
            (people, chars)
        })
        .collect()
}

#[aoc(day6, part1)]
pub fn part1(groups: &[(usize, HashMap<char, usize>)]) -> usize {
    groups.iter().map(|(_, answers)| answers.len()).sum()
}

#[aoc(day6, part2)]
pub fn part2(groups: &[(usize, HashMap<char, usize>)]) -> usize {
    groups
        .iter()
        .map(|(people, answers)| answers.values().filter(|count| **count == *people).count())
        .sum()
}
