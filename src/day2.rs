use aoc_runner_derive::{aoc, aoc_generator};

pub struct PasswordEntry {
    a: usize,
    b: usize,
    req: char,
    pass: String,
}

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<PasswordEntry> {
    input
        .split('\n')
        .map(|s| {
            let args = s.split_whitespace().collect::<Vec<&str>>();
            let (a, b) = args[0]
                .split_once('-')
                .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
                .unwrap();
            let req = args[1].chars().next().unwrap();
            let pass = args[2].to_string();
            PasswordEntry { a, b, req, pass }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[PasswordEntry]) -> usize {
    input
        .iter()
        .filter(|p| (p.a..p.b + 1).contains(&p.pass.matches(p.req).count()))
        .count()
}

#[aoc(day2, part2)]
pub fn part2(input: &[PasswordEntry]) -> usize {
    input
        .iter()
        .filter(|p| {
            let chars: Vec<char> = p.pass.chars().collect();
            (chars[p.a - 1] == p.req) ^ (chars[p.b - 1] == p.req)
        })
        .count()
}
