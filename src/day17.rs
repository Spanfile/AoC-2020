use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cube {
    Active,
    Inactive,
}

#[aoc_generator(day17)]
pub fn generator(input: &str) -> HashMap<(i32, i32, i32, i32), Cube> {
    input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                if c == '#' {
                    ((x as i32, y as i32, 0, 0), Cube::Active)
                } else {
                    ((x as i32, y as i32, 0, 0), Cube::Inactive)
                }
            })
        })
        .flatten()
        .collect()
}

#[aoc(day17, part1)]
pub fn part1(input: &HashMap<(i32, i32, i32, i32), Cube>) -> usize {
    let mut cubes = input.clone();

    for _ in 0..6 {
        let mut new_cubes = cubes.clone();
        let (mut x_min, mut x_max) = (i32::MAX, i32::MIN);
        let (mut y_min, mut y_max) = (i32::MAX, i32::MIN);
        let (mut z_min, mut z_max) = (i32::MAX, i32::MIN);

        for ((x, y, z, _), cube) in &new_cubes {
            if *cube == Cube::Active {
                if *x > x_max {
                    x_max = *x;
                } else if *x < x_min {
                    x_min = *x;
                }

                if *y > y_max {
                    y_max = *y;
                } else if *y < y_min {
                    y_min = *y;
                }

                if *z > z_max {
                    z_max = *z;
                } else if *z < z_min {
                    z_min = *z;
                }
            }
        }

        for c_x in x_min - 1..=x_max + 1 {
            for c_y in y_min - 1..=y_max + 1 {
                for c_z in z_min - 1..=z_max + 1 {
                    let c_coords = (c_x, c_y, c_z, 0);
                    let mut active_neighbours = 0;
                    for n_x in -1..=1 {
                        for n_y in -1..=1 {
                            for n_z in -1..=1 {
                                if n_x == 0 && n_y == 0 && n_z == 0 {
                                    continue;
                                }

                                let n_coords = (n_x + c_x, n_y + c_y, n_z + c_z, 0);

                                if cubes.get(&n_coords).copied() == Some(Cube::Active) {
                                    active_neighbours += 1;
                                }
                            }
                        }
                    }

                    *new_cubes.entry(c_coords).or_insert(Cube::Inactive) =
                        match cubes.get(&c_coords).copied().unwrap_or(Cube::Inactive) {
                            Cube::Active if (2..=3).contains(&active_neighbours) => Cube::Active,
                            Cube::Inactive if active_neighbours == 3 => Cube::Active,
                            _ => Cube::Inactive,
                        }
                }
            }
        }

        cubes = new_cubes;
    }

    cubes.values().filter(|c| **c == Cube::Active).count()
}

#[aoc(day17, part2)]
pub fn part2(input: &HashMap<(i32, i32, i32, i32), Cube>) -> usize {
    let mut cubes = input.clone();

    for _ in 0..6 {
        let mut new_cubes = cubes.clone();
        let (mut x_min, mut x_max) = (i32::MAX, i32::MIN);
        let (mut y_min, mut y_max) = (i32::MAX, i32::MIN);
        let (mut z_min, mut z_max) = (i32::MAX, i32::MIN);
        let (mut w_min, mut w_max) = (i32::MAX, i32::MIN);

        for ((x, y, z, w), cube) in &new_cubes {
            if *cube == Cube::Active {
                if *x > x_max {
                    x_max = *x;
                } else if *x < x_min {
                    x_min = *x;
                }

                if *y > y_max {
                    y_max = *y;
                } else if *y < y_min {
                    y_min = *y;
                }

                if *z > z_max {
                    z_max = *z;
                } else if *z < z_min {
                    z_min = *z;
                }

                if *w > w_max {
                    w_max = *w;
                } else if *w < w_min {
                    w_min = *w;
                }
            }
        }

        for c_x in x_min - 1..=x_max + 1 {
            for c_y in y_min - 1..=y_max + 1 {
                for c_z in z_min - 1..=z_max + 1 {
                    for c_w in w_min - 1..=w_max + 1 {
                        let c_coords = (c_x, c_y, c_z, c_w);
                        let mut active_neighbours = 0;
                        for n_x in -1..=1 {
                            for n_y in -1..=1 {
                                for n_z in -1..=1 {
                                    for n_w in -1..=1 {
                                        if n_x == 0 && n_y == 0 && n_z == 0 && n_w == 0 {
                                            continue;
                                        }

                                        let n_coords = (n_x + c_x, n_y + c_y, n_z + c_z, n_w + c_w);

                                        if cubes.get(&n_coords).copied() == Some(Cube::Active) {
                                            active_neighbours += 1;
                                        }
                                    }
                                }
                            }
                        }

                        *new_cubes.entry(c_coords).or_insert(Cube::Inactive) =
                            match cubes.get(&c_coords).copied().unwrap_or(Cube::Inactive) {
                                Cube::Active if (2..=3).contains(&active_neighbours) => Cube::Active,
                                Cube::Inactive if active_neighbours == 3 => Cube::Active,
                                _ => Cube::Inactive,
                            }
                    }
                }
            }
        }

        cubes = new_cubes;
    }

    cubes.values().filter(|c| **c == Cube::Active).count()
}
