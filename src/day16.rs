use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::RangeInclusive;
use std::str::FromStr;

type Ticket = Vec<i64>;

#[derive(Debug, Default)]
pub struct Input {
    rules: HashMap<String, (RangeInclusive<i64>, RangeInclusive<i64>)>,
    my_ticket: Ticket,
    tickets: Vec<Ticket>,
}

#[aoc_generator(day16)]
pub fn code_generator(input: &str) -> Option<Input> {
    #[derive(Debug)]
    enum State {
        Rules,
        MyTicket,
        Tickets,
    };
    //                      ^([\w ]+): (\d+)-(\d+) or (\d+)-(\d+)$
    let rule = Regex::new(r"^([\w ]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
    let mut state = State::Rules;
    let mut rules = HashMap::<String, (RangeInclusive<i64>, RangeInclusive<i64>)>::new();
    let mut my_ticket = Default::default();
    let mut tickets: Vec<Ticket> = Vec::new();
    for line in input.lines() {
        // println!("s:{:?}, l:{}", state, line);
        if line.is_empty() {
            continue;
        }
        match state {
            State::Rules if line == "your ticket:" => {
                state = State::MyTicket;
            }
            State::Rules => {
                let rule = rule.captures(line)?;
                rules.insert(
                    rule.get(1)?.as_str().to_owned(),
                    (
                        RangeInclusive::new(
                            rule.get(2).and_then(|c| i64::from_str(c.as_str()).ok())?,
                            rule.get(3).and_then(|c| i64::from_str(c.as_str()).ok())?,
                        ),
                        RangeInclusive::new(
                            rule.get(4).and_then(|c| i64::from_str(c.as_str()).ok())?,
                            rule.get(5).and_then(|c| i64::from_str(c.as_str()).ok())?,
                        ),
                    ),
                );
            }
            State::MyTicket if line == "nearby tickets:" => {
                state = State::Tickets;
            }
            State::MyTicket => {
                my_ticket = line
                    .split(',')
                    .map(|n| i64::from_str(n).ok())
                    .collect::<Option<Vec<_>>>()?;
            }
            State::Tickets => {
                let ticket = line
                    .split(',')
                    .map(|n| i64::from_str(n).ok())
                    .collect::<Option<Vec<_>>>()?;
                tickets.push(ticket);
            }
        }
    }
    Some(Input {
        rules,
        my_ticket,
        tickets,
    })
}

fn apply_rule((r1, r2): &(RangeInclusive<i64>, RangeInclusive<i64>), value: &i64) -> bool {
    r1.contains(value) || r2.contains(value)
}

#[aoc(day16, part1)]
pub fn part1(input: &Input) -> i64 {
    input
        .tickets
        .iter()
        .flat_map(|ticket| {
            ticket
                .iter()
                .filter(|v| !input.rules.values().any(|rule| apply_rule(rule, v)))
        })
        .sum()
}

#[aoc(day16, part2)]
pub fn part2(input: &Input) -> i64 {
    let valid = input.tickets.iter().filter(|ticket| {
        ticket.iter().all(|v| {
            input
                .rules
                .values()
                .any(|(r1, r2)| r1.contains(v) || r2.contains(v))
        })
    });
    let mut valid_fields = input
        .my_ticket
        .iter()
        .map(|_| input.rules.keys().cloned().collect::<HashSet<_>>())
        .collect::<Vec<_>>();
    for ticket in valid {
        for (v, fields) in ticket.iter().zip(valid_fields.iter_mut()) {
            fields.retain(|rn| apply_rule(input.rules.get(rn).unwrap(), v));
        }
    }
    println!("{:?}", valid_fields);
    let mut locked = HashSet::<String>::new();
    'outer: loop {
        for rn in input.rules.keys() {
            if locked.contains(rn) {
                continue;
            }
            let mut iter = valid_fields
                .iter()
                .enumerate()
                .filter(|(_, f)| f.contains(rn));
            let first = iter.next();
            if let Some((oi, _)) = first {
                if iter.next().is_none() {
                    locked.insert(rn.to_owned());
                    valid_fields[oi].retain(|vv| vv == rn);
                    continue 'outer;
                }
            }
        }
        break;
    }
    println!("{:?}", valid_fields);
    valid_fields
        .iter()
        .map(|vf| vf.iter().next().unwrap())
        .zip(input.my_ticket.iter())
        .filter(|(f, _)| f.starts_with("departure"))
        .map(|(_, v)| v)
        .product()
}
