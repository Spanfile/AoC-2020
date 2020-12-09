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
    let mut end = 0;
    let mut sum = 0;

    loop {
        match sum.cmp(&target) {
            Ordering::Equal => break,
            Ordering::Less => {
                sum += input[end];
                end += 1;
            }
            Ordering::Greater => {
                sum -= input[start];
                start += 1;
            }
        }
    }

    input[start..end]
        .iter()
        .max()
        .and_then(|max| input[start..end].iter().min().map(|min| min + max))
        .unwrap()
}
