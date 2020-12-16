use std::{collections::HashMap, ops::RangeInclusive};

use aoc_runner_derive::{aoc, aoc_generator};

pub struct Field {
    name: String,
    range1: RangeInclusive<u64>,
    range2: RangeInclusive<u64>,
}

pub struct Ticket {
    fields: Vec<u64>,
}

#[aoc_generator(day16)]
pub fn generator(input: &str) -> (Vec<Field>, Ticket, Vec<Ticket>) {
    let (fields, tickets) = input.split_once("\n\n").unwrap();

    let fields = fields
        .lines()
        .map(|l| {
            let (name, ranges) = l.split_once(": ").unwrap();
            let (range1, range2) = ranges.split_once(" or ").unwrap();
            let (r1_min, r1_max) = range1.split_once("-").unwrap();
            let (r2_min, r2_max) = range2.split_once("-").unwrap();
            Field {
                name: name.to_owned(),
                range1: r1_min.parse().unwrap()..=r1_max.parse().unwrap(),
                range2: r2_min.parse().unwrap()..=r2_max.parse().unwrap(),
            }
        })
        .collect();

    let (your, others) = tickets.split_once("\n\n").unwrap();
    let (_, your) = your.split_once('\n').unwrap();

    let your = Ticket {
        fields: your.split(',').map(|f| f.parse().unwrap()).collect(),
    };

    let others = others
        .split('\n')
        .skip(1)
        .map(|t| Ticket {
            fields: t.split(',').map(|f| f.parse().unwrap()).collect(),
        })
        .collect();

    (fields, your, others)
}

#[aoc(day16, part1)]
pub fn part1(input: &(Vec<Field>, Ticket, Vec<Ticket>)) -> u64 {
    let (fields, _, others) = input;
    let mut invalids_sum = 0;

    'tickets: for (_i, ticket) in others.iter().enumerate() {
        'field: for ticket_field in &ticket.fields {
            for field in fields {
                if field.range1.contains(ticket_field) || field.range2.contains(ticket_field) {
                    continue 'field;
                }
            }
            invalids_sum += ticket_field;
            // print!("{}, ", i);
            continue 'tickets;
        }
    }

    invalids_sum
}

#[aoc(day16, part2)]
pub fn part2(input: &(Vec<Field>, Ticket, Vec<Ticket>)) -> u64 {
    let invalids = vec![
        1, 4, 14, 15, 20, 27, 31, 37, 39, 42, 44, 45, 46, 47, 53, 55, 58, 63, 65, 67, 71, 72, 86, 97, 99, 106, 115,
        116, 130, 138, 142, 145, 147, 148, 155, 157, 184, 192, 206, 210, 214, 221, 230, 231, 232, 233,
    ];
    let (fields, your, others) = input;
    let valid_others: Vec<&Ticket> = others
        .iter()
        .enumerate()
        .filter_map(|(i, t)| if invalids.contains(&i) { None } else { Some(t) })
        .collect();
    let mut field_map = HashMap::new();

    for field in fields {
        'column: for column_index in 0..fields.len() {
            for ticket_field in valid_others.iter().map(|t| t.fields[column_index]) {
                if !(field.range1.contains(&ticket_field) || field.range2.contains(&ticket_field)) {
                    continue 'column;
                }
            }
            (*field_map.entry(&field.name).or_insert_with(Vec::new)).push(column_index);
        }
    }

    let mut certain = HashMap::new();
    loop {
        let wanted_amount = certain.len() + 1;
        if let Some((field, mut columns)) = field_map
            .drain_filter(|_, columns| columns.len() == wanted_amount)
            .next()
        {
            columns.retain(|c| !certain.values().any(|v| v == c));
            certain.insert(field, *columns.first().unwrap());
        } else {
            break;
        };
    }

    println!("{:#?}", certain);

    certain
        .iter()
        .filter_map(|(name, index)| {
            if name.starts_with("departure") {
                Some(your.fields[*index])
            } else {
                None
            }
        })
        .product()
}
