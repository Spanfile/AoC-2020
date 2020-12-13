use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day13)]
pub fn generator(input: &str) -> (i64, Vec<i64>) {
    let (timestamp, ids) = input.split_once('\n').unwrap();
    let timestamp = timestamp.parse().unwrap();
    let ids = ids.split(',').map(|i| i.parse().unwrap_or(0)).collect();

    (timestamp, ids)
}

#[aoc(day13, part1)]
pub fn part1(input: &(i64, Vec<i64>)) -> i64 {
    let (timestamp, ids) = input;
    let (id, leave) = ids
        .iter()
        .filter(|i| **i > 0)
        .map(|i| (i, ((timestamp / i) + 1) * i))
        .min_by_key(|(_, leave)| *leave)
        .unwrap();
    (leave - timestamp) * id
}

#[aoc(day13, part2)]
pub fn part2(input: &(i64, Vec<i64>)) -> i64 {
    let (_, ids) = input;
    let mut prod = 1;
    let mut timestamp = 100000000000000;

    for (idx, id) in ids.iter().enumerate().filter(|(_, id)| **id > 0) {
        while (timestamp + idx as i64) % id != 0 {
            timestamp += prod;
        }

        prod *= id;
    }

    timestamp
        % ids
            .iter()
            .filter(|id| **id > 0)
            .fold(1, |acc, x| num::integer::lcm(*x, acc))
}
