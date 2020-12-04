use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Default)]
pub struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
}

#[aoc_generator(day4)]
pub fn generator(input: &str) -> Vec<Passport> {
    let mut passports = Vec::new();
    for passport_args in input.split("\n\n") {
        let mut passport = Passport::default();
        for arg in passport_args.split_whitespace() {
            let (key, value) = arg.split_once(':').unwrap();
            match key {
                "byr" => passport.byr = Some(value.to_string()),
                "iyr" => passport.iyr = Some(value.to_string()),
                "eyr" => passport.eyr = Some(value.to_string()),
                "hgt" => passport.hgt = Some(value.to_string()),
                "hcl" => passport.hcl = Some(value.to_string()),
                "ecl" => passport.ecl = Some(value.to_string()),
                "pid" => passport.pid = Some(value.to_string()),
                _ => panic!(),
            }
        }
        passports.push(passport);
    }
    passports
}

#[aoc(day4, part1)]
pub fn part1(input: &[Passport]) -> usize {
    input
        .iter()
        .filter(|p| {
            p.byr
                .as_ref()
                .and(p.iyr.as_ref())
                .and(p.eyr.as_ref())
                .and(p.hgt.as_ref())
                .and(p.hcl.as_ref())
                .and(p.ecl.as_ref())
                .and(p.pid.as_ref())
                .is_some()
        })
        .count()
}

#[aoc(day4, part2)]
pub fn part2(input: &[Passport]) -> usize {
    input
        .iter()
        .filter(|p| {
            in_range(p.byr.as_deref(), 1920..=2002)
                .and(in_range(p.iyr.as_deref(), 2010..=2020))
                .and(in_range(p.eyr.as_deref(), 2020..=2030))
                .and(hgt(p.hgt.as_deref()))
                .and(hcl(p.hcl.as_deref()))
                .and(ecl(p.ecl.as_deref()))
                .and(pid(p.pid.as_deref()))
                .is_some()
        })
        .count()
}

fn in_range(val: Option<&str>, range: std::ops::RangeInclusive<i32>) -> Option<&str> {
    let val = val?;
    if let Ok(num) = val.parse::<i32>() {
        if range.contains(&num) {
            return Some(val);
        }
    }
    None
}

fn hgt(val: Option<&str>) -> Option<&str> {
    let val = val?;
    if let Some(num) = val.strip_suffix("cm") {
        if let Ok(num) = num.parse::<i32>() {
            if (150..=193).contains(&num) {
                return Some(val);
            }
        }
    } else if let Some(num) = val.strip_suffix("in") {
        if let Ok(num) = num.parse::<i32>() {
            if (59..=76).contains(&num) {
                return Some(val);
            }
        }
    }
    None
}

fn hcl(val: Option<&str>) -> Option<&str> {
    let val = val?;
    if let Some(hex) = val.strip_prefix('#') {
        if u32::from_str_radix(hex, 16).is_ok() {
            return Some(val);
        }
    }
    None
}

fn ecl(val: Option<&str>) -> Option<&str> {
    let val = val?;
    match val {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => Some(val),
        _ => None,
    }
}

fn pid(val: Option<&str>) -> Option<&str> {
    let val = val?;
    if val.len() == 9 && val.parse::<i32>().is_ok() {
        return Some(val);
    }
    None
}
