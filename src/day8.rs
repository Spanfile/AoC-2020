use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
pub enum Opcode {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl Opcode {
    fn flip(self) -> Self {
        match self {
            Opcode::Jmp(val) => Opcode::Nop(val),
            Opcode::Nop(val) => Opcode::Jmp(val),
            _ => self,
        }
    }
}

#[aoc_generator(day8)]
pub fn generator(input: &str) -> Vec<Opcode> {
    input
        .split('\n')
        .map(|s| {
            let (opcode, value) = s.split_once(' ').unwrap();
            match opcode {
                "acc" => Opcode::Acc(value.parse().unwrap()),
                "jmp" => Opcode::Jmp(value.parse().unwrap()),
                "nop" => Opcode::Nop(value.parse().unwrap()),
                _ => panic!(),
            }
        })
        .collect()
}

#[aoc(day8, part1)]
pub fn part1(input: &[Opcode]) -> i32 {
    let mut idx = 0;
    let mut acc = 0;
    let mut visited = HashSet::new();

    loop {
        if visited.contains(&idx) {
            break;
        }

        visited.insert(idx);
        match input[idx as usize] {
            Opcode::Acc(val) => acc += val,
            Opcode::Jmp(val) => {
                idx += val;
                continue;
            }
            _ => (),
        }

        idx += 1;
    }

    acc
}

#[aoc(day8, part2)]
pub fn part2(original: &[Opcode]) -> i32 {
    let mut input = original.to_vec();
    let mut flip_history = HashSet::new();

    'outer: loop {
        let mut idx: i32 = 0;
        let mut visited = HashSet::new();
        let mut history = Vec::new();
        let mut acc = 0;

        loop {
            if visited.contains(&idx) {
                // found a loop, backtrack and flip the most recent not-already-tried-to-swap jmp or nop
                while let Some(recent) = history.pop() {
                    match input[recent as usize] {
                        Opcode::Jmp(_) | Opcode::Nop(_) => {
                            if !flip_history.insert(recent) {
                                // this one's been tried, try the next one
                                continue;
                            }

                            // reset the input, flip the instruction
                            input = original.to_vec();
                            input[recent as usize] = input[recent as usize].flip();
                            break;
                        }
                        _ => (),
                    }
                }

                continue 'outer;
            }

            if idx >= input.len() as i32 {
                break 'outer acc;
            }

            visited.insert(idx);
            history.push(idx);
            match input[idx as usize] {
                Opcode::Acc(val) => acc += val,
                Opcode::Jmp(val) => {
                    idx += val;
                    continue;
                }
                _ => (),
            }

            idx += 1;
        }
    }
}
