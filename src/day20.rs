use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct Tile {
    id: i32,
    edges: [u32; 4],
}

#[aoc_generator(day20)]
pub fn generator(input: &str) -> Vec<Tile> {
    input
        .split("\n\n")
        .map(|s| {
            let (id, tile) = s.split_once(":\n").unwrap();
            let id = id.strip_prefix("Tile ").and_then(|s| s.parse().ok()).unwrap();
            let mut edges = [0; 4];

            for (i, l) in tile.lines().enumerate() {
                let chars = l.chars().collect::<Vec<char>>();
                let first_c = chars.first().map(|c| if *c == '.' { 0 } else { 1 }).unwrap();
                let last_c = chars.last().map(|c| if *c == '.' { 0 } else { 1 }).unwrap();

                edges[1] |= last_c << i;
                edges[3] |= first_c << i;
                if i == 0 || i == 9 {
                    edges[if i == 0 { 0 } else { 2 }] = chars
                        .iter()
                        .map(|c| if *c == '.' { 0 } else { 1 })
                        .fold(0, |acc, x| (acc << 1) | x);
                }
            }

            Tile { id, edges }
        })
        .collect()
}

#[aoc(day20, part1)]
pub fn part1(input: &[Tile]) -> i32 {
    for tile in input {
        let mut edges = 0;
        for e1 in &tile.edges {
            for e2 in input
                .iter()
                .filter_map(|t| if t.id != tile.id { Some(t.edges.iter()) } else { None })
                .flatten()
            {
                if e1 == e2 || reversed(*e1) == *e2 {
                    edges += 1;
                }
            }
        }

        println!("{:?} has {} matching edges", tile, edges);
        if edges == 2 {
            println!("{:?} is a corner", tile);
        }
    }

    0
}

#[aoc(day20, part2)]
pub fn part2(input: &[Tile]) -> i32 {
    0
}

fn reversed(mut v: u32) -> u32 {
    let mut rev = 0;
    while v > 0 {
        rev <<= 1;
        if v & 1 == 1 {
            rev |= 1;
        }
        v >>= 1;
    }

    rev
}
