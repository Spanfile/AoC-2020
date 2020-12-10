use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[aoc_generator(day10)]
pub fn generator(input: &str) -> Vec<usize> {
    input.split('\n').map(|s| s.parse().unwrap()).collect()
}

#[aoc(day10, part1)]
pub fn part1(input: &[usize]) -> usize {
    let mut input: HashSet<usize> = input.to_owned().into_iter().collect();
    let mut current = 0;
    let mut ones = 0;
    let mut threes = 1;

    while !input.is_empty() {
        if input.remove(&(current + 1)) {
            current += 1;
            ones += 1;
        } else {
            input.remove(&(current + 3));
            current += 3;
            threes += 1;
        }
    }

    ones * threes
}

#[aoc(day10, part2)]
pub fn part2(input: &[usize]) -> usize {
    let mut input = input.to_vec();
    input.sort_unstable();

    let mut values = vec![0];
    values.extend_from_slice(&input);
    values.push(values.last().unwrap() + 3);

    let mut total = 1usize;
    let mut ones = 0;
    for diff in values.iter().enumerate().skip(1).map(|(i, v)| v - values[i - 1]) {
        match (diff, ones) {
            (1, _) => ones += 1,
            (3, 1) => ones = 0,
            (3, 2) => {
                total *= 2;
                ones = 0;
            }
            (3, 3) => {
                total *= 4;
                ones = 0;
            }
            (3, 4) => {
                total *= 7;
                ones = 0;
            }
            _ => (),
        }
    }

    total
}
