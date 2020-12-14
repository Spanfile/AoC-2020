use aoc_runner_derive::{aoc, aoc_generator};

pub enum Action {
    North(i64),
    South(i64),
    East(i64),
    West(i64),
    TurnLeft(i64),
    TurnRight(i64),
    Forward(i64),
}

#[derive(Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn turn_left(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
}

#[aoc_generator(day12)]
pub fn generator(input: &str) -> Vec<Action> {
    input
        .lines()
        .map(|s| {
            let (action, value) = s.split_at(1);

            match action {
                "N" => Action::North(value.parse().unwrap()),
                "S" => Action::South(value.parse().unwrap()),
                "E" => Action::East(value.parse().unwrap()),
                "W" => Action::West(value.parse().unwrap()),
                "L" => Action::TurnLeft(value.parse().unwrap()),
                "R" => Action::TurnRight(value.parse().unwrap()),
                "F" => Action::Forward(value.parse().unwrap()),
                _ => panic!(),
            }
        })
        .collect()
}

#[aoc(day12, part1)]
pub fn part1(input: &[Action]) -> i64 {
    let mut x = 0;
    let mut y = 0;
    let mut direction = Direction::East;

    for action in input {
        match action {
            Action::North(dist) => y -= dist,
            Action::South(dist) => y += dist,
            Action::East(dist) => x += dist,
            Action::West(dist) => x -= dist,
            Action::TurnLeft(deg) => {
                for _ in 0..(deg / 90) {
                    direction = direction.turn_left()
                }
            }
            Action::TurnRight(deg) => {
                for _ in 0..(deg / 90) {
                    direction = direction.turn_right()
                }
            }
            Action::Forward(dist) => match direction {
                Direction::North => y -= dist,
                Direction::East => x += dist,
                Direction::South => y += dist,
                Direction::West => x -= dist,
            },
        }
    }

    x.abs() + y.abs()
}

#[aoc(day12, part2)]
pub fn part2(input: &[Action]) -> i64 {
    let mut x = 0;
    let mut y = 0;
    let mut w_x = 10;
    let mut w_y = -1;

    for action in input {
        match action {
            Action::North(dist) => w_y -= dist,
            Action::South(dist) => w_y += dist,
            Action::East(dist) => w_x += dist,
            Action::West(dist) => w_x -= dist,
            Action::TurnLeft(deg) => (w_x, w_y) = rotate_point(w_x, w_y, -deg),
            Action::TurnRight(deg) => (w_x, w_y) = rotate_point(w_x, w_y, *deg),
            Action::Forward(dist) => {
                x += dist * w_x;
                y += dist * w_y;
            }
        }
    }

    x.abs() + y.abs()
}

fn rotate_point(x: i64, y: i64, deg: i64) -> (i64, i64) {
    match deg {
        90 | -270 => (-y, x),
        -90 | 270 => (y, -x),
        180 | -180 => (-x, -y),
        _ => panic!(),
    }
}
