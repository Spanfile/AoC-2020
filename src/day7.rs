use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};

#[aoc_generator(day7)]
pub fn generator(input: &str) -> HashMap<String, Vec<(usize, String)>> {
    input
        .split('\n')
        .map(|s| {
            let (bag, contains) = s.split_once(" contain ").unwrap();
            let bag = bag.strip_suffix(" bags").unwrap();

            if contains != "no other bags." {
                (
                    bag.to_string(),
                    contains
                        .split(", ")
                        .map(|b| {
                            let (amount, bag) = b.split_once(' ').unwrap();
                            let (bag, _) = bag.rsplit_once(" ").unwrap();
                            (amount.parse::<usize>().unwrap(), bag.to_string())
                        })
                        .collect(),
                )
            } else {
                (bag.to_string(), vec![])
            }
        })
        .collect()
}

#[aoc(day7, part1)]
pub fn part1(input: &HashMap<String, Vec<(usize, String)>>) -> usize {
    let mut found = HashSet::new();
    let mut find = vec!["shiny gold"];

    'finder: loop {
        if let Some(f) = find.pop() {
            for (bag, bags) in input {
                for (_, b) in bags {
                    if b == f {
                        found.insert(bag);
                        find.push(bag);
                    }
                }
            }
            continue 'finder;
        }
        break;
    }
    found.len()
}

#[aoc(day7, part2)]
pub fn part2(input: &HashMap<String, Vec<(usize, String)>>) -> usize {
    all_these_bags(input, "shiny gold")
}

fn all_these_bags(bags: &HashMap<String, Vec<(usize, String)>>, bag: &str) -> usize {
    let mut total = 0;
    for (amount, b) in bags.get(bag).unwrap() {
        println!("{} contains {} {} bags", bag, amount, b);
        total += amount + amount * all_these_bags(bags, b);
    }
    total
}
