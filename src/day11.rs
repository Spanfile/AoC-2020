use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy, PartialEq)]
pub enum Seat {
    Floor,
    Empty,
    Occupied,
}

#[aoc_generator(day11)]
pub fn generator(input: &str) -> (Vec<Seat>, i32, i32) {
    let mut w = 0;
    let mut h = 0;
    let mut seats = Vec::new();

    for l in input.lines() {
        h += 1;
        if w == 0 {
            w = l.len() as i32;
        }

        for c in l.chars() {
            seats.push(match c {
                '.' => Seat::Floor,
                'L' => Seat::Empty,
                _ => panic!(),
            });
        }
    }

    (seats, w, h)
}

#[aoc(day11, part1)]
pub fn part1(input: &(Vec<Seat>, i32, i32)) -> usize {
    let (mut seats, w, h) = input.clone();
    let mut count = 0;

    loop {
        let mut changes = false;
        let mut new_seats = seats.clone();

        for y in 0..h {
            for x in 0..w {
                let mut occupied = 0;

                for a in -1..=1 {
                    for b in -1..=1 {
                        if (0..w).contains(&(x + a))
                            && (0..h).contains(&(y + b))
                            && !(a == 0 && b == 0)
                            && get_seat(&seats, w, x + a, y + b) == Seat::Occupied
                        {
                            occupied += 1;
                        }
                    }
                }

                match get_seat(&seats, w, x, y) {
                    Seat::Empty => {
                        if occupied == 0 {
                            set_seat(&mut new_seats, w, x, y, Seat::Occupied);
                            changes = true;
                        }
                    }
                    Seat::Occupied => {
                        if occupied >= 4 {
                            set_seat(&mut new_seats, w, x, y, Seat::Empty);
                            changes = true;
                        }
                    }
                    _ => (),
                }
            }
        }

        if changes {
            seats = new_seats;
        } else {
            break seats.iter().filter(|s| **s == Seat::Occupied).count();
        }
    }
}

#[aoc(day11, part2)]
pub fn part2(input: &(Vec<Seat>, i32, i32)) -> usize {
    let (mut seats, w, h) = input.clone();

    loop {
        let mut changes = false;
        let mut new_seats = seats.clone();

        for y in 0..h {
            for x in 0..w {
                let mut occupied = 0;

                // left
                for a in 1.. {
                    if !seat_exists(w, h, x - a, y) {
                        break;
                    }

                    match get_seat(&seats, w, x - a, y) {
                        Seat::Empty => break,
                        Seat::Occupied => {
                            occupied += 1;
                            break;
                        }
                        _ => (),
                    }
                }

                // right
                for a in 1.. {
                    if !seat_exists(w, h, x + a, y) {
                        break;
                    }

                    match get_seat(&seats, w, x + a, y) {
                        Seat::Empty => break,
                        Seat::Occupied => {
                            occupied += 1;
                            break;
                        }
                        _ => (),
                    }
                }

                // up
                for a in 1.. {
                    if !seat_exists(w, h, x, y - a) {
                        break;
                    }

                    match get_seat(&seats, w, x, y - a) {
                        Seat::Empty => break,
                        Seat::Occupied => {
                            occupied += 1;
                            break;
                        }
                        _ => (),
                    }
                }

                // down
                for a in 1.. {
                    if !seat_exists(w, h, x, y + a) {
                        break;
                    }

                    match get_seat(&seats, w, x, y + a) {
                        Seat::Empty => break,
                        Seat::Occupied => {
                            occupied += 1;
                            break;
                        }
                        _ => (),
                    }
                }

                // top-left
                for a in 1.. {
                    if !seat_exists(w, h, x - a, y - a) {
                        break;
                    }

                    match get_seat(&seats, w, x - a, y - a) {
                        Seat::Empty => break,
                        Seat::Occupied => {
                            occupied += 1;
                            break;
                        }
                        _ => (),
                    }
                }

                // top-right
                for a in 1.. {
                    if !seat_exists(w, h, x + a, y - a) {
                        break;
                    }

                    match get_seat(&seats, w, x + a, y - a) {
                        Seat::Empty => break,
                        Seat::Occupied => {
                            occupied += 1;
                            break;
                        }
                        _ => (),
                    }
                }

                // bottom-left
                for a in 1.. {
                    if !seat_exists(w, h, x - a, y + a) {
                        break;
                    }

                    match get_seat(&seats, w, x - a, y + a) {
                        Seat::Empty => break,
                        Seat::Occupied => {
                            occupied += 1;
                            break;
                        }
                        _ => (),
                    }
                }

                // bottom-right
                for a in 1.. {
                    if !seat_exists(w, h, x + a, y + a) {
                        break;
                    }

                    match get_seat(&seats, w, x + a, y + a) {
                        Seat::Empty => break,
                        Seat::Occupied => {
                            occupied += 1;
                            break;
                        }
                        _ => (),
                    }
                }

                match get_seat(&seats, w, x, y) {
                    Seat::Empty => {
                        if occupied == 0 {
                            set_seat(&mut new_seats, w, x, y, Seat::Occupied);
                            changes = true;
                        }
                    }
                    Seat::Occupied => {
                        if occupied >= 5 {
                            set_seat(&mut new_seats, w, x, y, Seat::Empty);
                            changes = true;
                        }
                    }
                    _ => (),
                }
            }
        }

        if changes {
            seats = new_seats;
        } else {
            break seats.iter().filter(|s| **s == Seat::Occupied).count();
        }
    }
}

fn seat_exists(w: i32, h: i32, x: i32, y: i32) -> bool {
    (0..w).contains(&x) && (0..h).contains(&y)
}

fn get_seat(seats: &[Seat], w: i32, x: i32, y: i32) -> Seat {
    seats[(y * w + x) as usize]
}

fn set_seat(seats: &mut [Seat], w: i32, x: i32, y: i32, seat: Seat) {
    seats[(y * w + x) as usize] = seat;
}
