use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[aoc_generator(day1)]
pub fn generator(input: &str) -> Vec<i32> {
    input.split('\n').map(|s| s.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[i32]) -> i32 {
    let mut nums = HashSet::new();
    for v in input {
        let other = 2020 - v;
        if nums.contains(&other) {
            return other * v;
        } else {
            nums.insert(v);
        }
    }
    0
}

#[aoc(day1, part2)]
pub fn part2(input: &[i32]) -> i32 {
    for c in input {
        let target = 2020 - c;
        let mut nums = HashSet::new();
        for v in input {
            let other = target - v;
            if nums.contains(&other) {
                return other * v * c;
            } else {
                nums.insert(v);
            }
        }
    }
    0
}
