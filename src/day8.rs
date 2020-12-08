use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
pub enum Opcode {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
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
pub fn part2(input: &[Opcode]) -> i32 {
    let mut to_search = vec![630..=634];
    let mut candidates = HashSet::new();

    while let Some(search) = to_search.pop() {
        for (idx, opcode) in input.iter().enumerate() {
            match opcode {
                Opcode::Jmp(val) | Opcode::Nop(val) => {
                    if search.contains(&(val + idx as i32)) {
                        if !candidates.insert(idx) {
                            continue;
                        }

                        println!("candidate: {:?} at idx {}", opcode, idx);

                        let end = idx as i32;
                        for i in 1i32.. {
                            if let Opcode::Jmp(_) = input[(end - i) as usize] {
                                let start = end - i + 1;
                                to_search.push(start..=end);
                                break;
                            }
                        }
                    }
                }
                _ => (),
            }
        }
    }

    'outer: for c in candidates {
        let mut opcodes = input.to_vec();
        match opcodes[c] {
            Opcode::Jmp(val) => opcodes[c] = Opcode::Nop(val),
            Opcode::Nop(val) => opcodes[c] = Opcode::Jmp(val),
            _ => (),
        }

        let mut visited = HashSet::new();
        let mut idx = 0i32;
        let mut acc = 0;

        loop {
            if visited.contains(&idx) {
                continue 'outer;
            }

            if idx >= opcodes.len() as i32 {
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

        return acc;
    }

    0
}
