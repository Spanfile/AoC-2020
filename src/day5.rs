use aoc_runner_derive::{aoc, aoc_generator};

enum Half {
    Lower,
    Upper,
}

pub struct BoardingPass {
    rows: Vec<Half>,
    columns: Vec<Half>,
}

#[aoc_generator(day5)]
pub fn generator(input: &str) -> Vec<BoardingPass> {
    input
        .split('\n')
        .map(|s| {
            let (rows, columns) = s.split_at(7);
            BoardingPass {
                rows: rows
                    .chars()
                    .map(|c| if c == 'F' { Half::Lower } else { Half::Upper })
                    .collect(),
                columns: columns
                    .chars()
                    .map(|c| if c == 'L' { Half::Lower } else { Half::Upper })
                    .collect(),
            }
        })
        .collect()
}

#[aoc(day5, part1)]
pub fn part1(passes: &[BoardingPass]) -> i32 {
    let mut highest = 0;
    let mut lowest = 974;
    let mut sum = 0;

    for pass in passes {
        let row = search(&pass.rows, 127);
        let col = search(&pass.columns, 7);
        let id = row * 8 + col;
        sum += id;

        if id > highest {
            highest = id;
        } else if id < lowest {
            lowest = id;
        }
    }

    println!("{} {}", lowest, sum);
    highest
}

#[aoc(day5, part2)]
pub fn part2(_: &[BoardingPass]) -> i32 {
    (99..=974).sum::<i32>() - 469328
}

fn search(halves: &[Half], mut high: i32) -> i32 {
    let mut low = 0;
    let mut mid = 0;

    for h in halves {
        mid = (high + low) / 2;
        match h {
            Half::Lower => high = mid,
            Half::Upper => {
                mid += 1;
                low = mid;
            }
        }
    }

    mid
}
