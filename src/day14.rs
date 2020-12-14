use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Operation {
    Mask { ones: u64, zeros: u64, floating: u64 },
    Mem { addr: usize, val: u64 },
}

#[aoc_generator(day14)]
pub fn generator(input: &str) -> Vec<Operation> {
    input
        .lines()
        .map(|l| {
            let (operation, value) = l.split_once(" = ").unwrap();

            if let Some(addr) = operation.strip_prefix("mem[").and_then(|s| s.strip_suffix(']')) {
                Operation::Mem {
                    addr: addr.parse().unwrap(),
                    val: value.parse().unwrap(),
                }
            } else {
                let ones = value.replace('X', "0");
                let zeros = value.replace('X', "1");
                let floating = value.replace('1', "0").replace('X', "1");
                Operation::Mask {
                    ones: u64::from_str_radix(&ones, 2).unwrap(),
                    zeros: u64::from_str_radix(&zeros, 2).unwrap(),
                    floating: u64::from_str_radix(&floating, 2).unwrap(),
                }
            }
        })
        .collect()
}

#[aoc(day14, part1)]
pub fn part1(input: &[Operation]) -> u64 {
    let mut memory = HashMap::new();
    let mut mask_ones = 0;
    let mut mask_zeros = 0;

    for op in input {
        match op {
            Operation::Mask {
                ones,
                zeros,
                floating: _,
            } => {
                mask_ones = *ones;
                mask_zeros = *zeros;
            }
            Operation::Mem { addr, val } => {
                let val = (val | mask_ones) & mask_zeros;
                memory.insert(addr, val);
            }
        }
    }

    memory.values().sum()
}

#[aoc(day14, part2)]
pub fn part2(input: &[Operation]) -> u64 {
    let mut memory = HashMap::new();
    let mut mask_ones = 0;
    let mut masks = Vec::new();

    for op in input {
        match op {
            Operation::Mask {
                ones,
                zeros: _,
                floating,
            } => {
                mask_ones = *ones;

                let bit_indices = (0..36).filter(|b| (floating >> b & 1) == 1).collect::<Vec<u64>>();
                masks.clear();
                for i in 1..=bit_indices.len() {
                    for c in bit_indices.iter().combinations(i) {
                        let mut mask = 0;
                        for b in c {
                            mask |= 1 << *b;
                        }
                        masks.push(mask);
                    }
                }
            }
            Operation::Mem { addr, val } => {
                let addr = *addr as u64 | mask_ones;
                memory.insert(addr, *val);
                for mask in &masks {
                    memory.insert(addr | mask, *val);
                }
            }
        }
    }

    memory.values().sum()
}
