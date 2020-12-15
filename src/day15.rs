use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[aoc_generator(day15)]
pub fn generator(input: &str) -> Vec<i32> {
    input.split(',').map(|s| s.parse().unwrap()).collect()
}

#[aoc(day15, part1)]
pub fn part1(input: &[i32]) -> i32 {
    let mut numbers = input
        .iter()
        .enumerate()
        .map(|(i, v)| (*v, (1, i + 1)))
        .collect::<HashMap<_, _>>();
    let mut last_number = *input.last().unwrap();

    for turn in input.len() + 1..=2020 {
        let (times_spoken, last_spoken) = *numbers.get(&last_number).unwrap();
        if times_spoken == 1 {
            // println!("turn {}: {} has been spoken only once, saying 0", turn, last_number);
            last_number = 0;
        } else {
            // print!(
            //     "turn {}: {} has been spoken more than once, last in turn {}, saying ",
            //     turn, last_number, last_spoken,
            // );
            numbers.entry(last_number).and_modify(|(_, t)| {
                *t = turn - 1;
            });
            last_number = (turn - 1 - last_spoken) as i32;
            // println!("{}", last_number);
        }

        numbers
            .entry(last_number)
            .and_modify(|(s, _)| {
                *s += 1;
            })
            .or_insert((1, turn));
    }

    last_number
}

#[aoc(day15, part2)]
pub fn part2(_: &[i32]) -> u64 {
    // change the turn bound above to 30000000 instead of 2020
    0
}
