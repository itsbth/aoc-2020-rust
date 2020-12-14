use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

pub enum Op {
    SetMask(u64, u64),
    SetMem(usize, u64),
}

type Input = Vec<Op>;

#[aoc_generator(day14)]
pub fn generate(input: &str) -> Input {
    let mask_re = Regex::new("^mask = ([01X]{36})$").unwrap();
    let mem_re = Regex::new("^mem\\[(\\d+)\\] = (\\d+)$").unwrap();
    input
        .lines()
        .map(|l| {
            if let Some(cap) = mask_re.captures(l) {
                let mask = cap.get(1).unwrap().as_str();
                let mm = u64::from_str_radix(
                    mask.chars()
                        .map(|c| match c {
                            'X' => '0',
                            _ => '1',
                        })
                        .collect::<String>()
                        .as_str(),
                    2,
                )
                .unwrap();
                let mv = u64::from_str_radix(mask.replace("X", "0").as_str(), 2).unwrap();
                return Op::SetMask(mm, mv);
            }
            if let Some(cap) = mem_re.captures(l) {
                return Op::SetMem(
                    cap.get(1)
                        .and_then(|c| usize::from_str(c.as_str()).ok())
                        .unwrap(),
                    cap.get(2)
                        .and_then(|c| u64::from_str(c.as_str()).ok())
                        .unwrap(),
                );
            }
            panic!("bad line");
        })
        .collect()
}

#[aoc(day14, part1)]
pub fn part1(inp: &Input) -> u64 {
    let mut mem = HashMap::<usize, u64>::new();
    let mut mm = 0;
    let mut mv = 0;
    for cmd in inp {
        match cmd {
            Op::SetMask(m, v) => {
                mm = *m;
                mv = *v;
            }
            Op::SetMem(p, v) => {
                mem.insert(*p, v & !mm | mv);
            }
        }
    }
    mem.values().sum()
}

pub fn set_val(map: &mut HashMap<usize, u64>, addr: usize, mask: usize, value: u64) {
    // println!("{:b} {:b}", addr, mask);
    if mask == 0 {
        // println!("inserting {} into {}", value, addr);
        map.insert(addr, value);
        return;
    }
    let lb = mask & (0 - mask);
    set_val(map, addr | lb, mask & !lb, value);
    set_val(map, addr & !lb, mask & !lb, value);
}

#[aoc(day14, part2, Naive)]
pub fn part2(inp: &Input) -> u64 {
    let mut mem = HashMap::<usize, u64>::new();
    let mut mm = 0;
    let mut mv = 0;
    for cmd in inp {
        match cmd {
            Op::SetMask(m, v) => {
                mm = *m;
                mv = *v;
            }
            Op::SetMem(p, v) => {
                let mask = !mm as usize & 0xF_FF_FF_FF_FF;
                set_val(&mut mem, *p | mv as usize, mask, *v);
            }
        }
    }
    // println!("{:?}", mem.iter().collect::<Vec<_>>());
    mem.values().sum()
}
