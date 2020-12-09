use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;
use std::str::FromStr;

#[aoc_generator(day9)]
pub fn generate(numbers: &str) -> Vec<i64> {
    numbers.lines().map(|n| i64::from_str(n).unwrap()).collect()
}

const PREAMBLE: usize = 25;

#[aoc(day9, part1)]
pub fn part1(numbers: &Vec<i64>) -> i64 {
    let mut valid: [HashSet<i64>; PREAMBLE] = Default::default();
    let mut cb = [0i64; PREAMBLE];
    for (idx, n) in numbers.iter().enumerate() {
        // println!("{:?}", valid);
        if idx >= PREAMBLE && valid.iter().all(|h| !h.contains(n)) {
            return *n;
        }
        let im = idx % PREAMBLE;
        cb[im] = *n;
        valid[im].clear();
        valid[im].extend(
            cb.iter()
                .enumerate()
                .filter(|(i, _)| *i != im)
                .map(|(_, v)| v + n),
        );
    }
    panic!("!");
}

const PART1_ANS: i64 = 1639024365;

#[aoc(day9, part2)]
pub fn part2(numbers: &Vec<i64>) -> i64 {
    let mut s = 0;
    let mut e = 0;
    let mut a = 0;
    loop {
        println!("{}..{} = {}", s, e - 1, a);
        if e >= numbers.len() || s > e {
            panic!("ran out of numbers");
        }
        if a == PART1_ANS {
            let (mn, mx) = numbers[s..(e - 1)]
                .iter()
                .fold((i64::MAX, i64::MIN), |(mn, mx), n| (mn.min(*n), mx.max(*n)));
            return mn + mx;
        }
        if a < PART1_ANS {
            a += numbers[e];
            e += 1;
        }
        if a > PART1_ANS {
            a -= numbers[s];
            s += 1;
        }
    }
}
