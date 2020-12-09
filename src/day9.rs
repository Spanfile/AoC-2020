use aoc_runner_derive::{aoc, aoc_generator};
use std::{cmp::Ordering, collections::HashSet};

#[aoc_generator(day9)]
pub fn generator(input: &str) -> Vec<usize> {
    input.split('\n').map(|s| s.parse().unwrap()).collect()
}

#[aoc(day9, part1)]
pub fn part1(input: &[usize]) -> usize {
    let mut seen = HashSet::new();
    let mut start = 0;
    let mut end = 25;

    'outer: loop {
        let next = input[end];

        for v in input.iter().skip(start).take(25) {
            if *v >= next {
                continue;
            }

            let target = next - v;
            if seen.contains(&target) {
                seen.remove(&input[start]);
                start += 1;
                end += 1;

                continue 'outer;
            } else {
                seen.insert(v);
            }
        }

        break next;
    }
}

#[aoc(day9, part2)]
pub fn part2(input: &[usize]) -> usize {
    let target = 15690279;
    let mut start = 0;

    let (smallest, largest) = 'outer: loop {
        let mut sum = 0;
        let mut smallest = usize::MAX;
        let mut largest = 0;

        for v in input.iter().skip(start) {
            if *v > largest {
                largest = *v;
            } else if *v < smallest {
                smallest = *v;
            }

            sum += v;
            match sum.cmp(&target) {
                Ordering::Equal => break 'outer (smallest, largest),
                Ordering::Greater => break,
                _ => (),
            }
        }

        start += 1;
    };

    smallest + largest
}
