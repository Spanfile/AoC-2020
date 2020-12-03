use aoc_runner_derive::{aoc, aoc_generator};

pub struct Map {
    w: usize,
    h: usize,
    trees: Vec<bool>,
}

impl Map {
    fn oof_ow_the_trees(&self, step_x: usize, step_y: usize) -> usize {
        (step_y..self.h)
            .step_by(step_y)
            .zip((step_x..).step_by(step_x))
            .filter(|(y, x)| self.trees[y * self.w + x % self.w])
            .count()
    }
}

#[aoc_generator(day3)]
pub fn generator(input: &str) -> Map {
    let mut w = 0;
    let mut h = 0;
    let mut trees = Vec::new();
    for l in input.lines() {
        h += 1;
        if w == 0 {
            w = l.len();
        }

        for c in l.chars() {
            trees.push(c == '#');
        }
    }

    Map { w, h, trees }
}

#[aoc(day3, part1)]
pub fn part1(map: &Map) -> usize {
    map.oof_ow_the_trees(3, 1)
}

#[aoc(day3, part2)]
pub fn part2(map: &Map) -> usize {
    292 * map.oof_ow_the_trees(1, 1)
        * map.oof_ow_the_trees(5, 1)
        * map.oof_ow_the_trees(7, 1)
        * map.oof_ow_the_trees(1, 2)
}
