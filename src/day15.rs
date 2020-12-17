use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::str::FromStr;

type Input = Vec<i64>;

#[aoc_generator(day15)]
pub fn generator(nums: &str) -> Input {
    nums.split(',').map(|n| i64::from_str(n).unwrap()).collect()
}

#[aoc(day15, part1)]
pub fn part1(code: &Input) -> i64 {
    let mut seen = HashMap::<i64, usize>::new();
    let mut last = *code.iter().last().unwrap();
    for (idx, n) in code[..code.len() - 1].iter().enumerate() {
        seen.insert(*n, idx);
    }
    for i in code.len()..2020 {
        let cur = if let Some(r) = seen.get(&last) {
            i - r - 1
        } else {
            0
        };
        seen.insert(last as i64, i - 1);
        last = cur as i64;
    }
    last
}

#[aoc(day15, part2)]
pub fn part2(code: &Input) -> i64 {
    let mut seen = HashMap::<i64, usize>::new();
    let mut last = *code.iter().last().unwrap();
    for (idx, n) in code[..code.len() - 1].iter().enumerate() {
        seen.insert(*n, idx);
    }
    for i in code.len()..30000000 {
        let cur = if let Some(r) = seen.get(&last) {
            i - r - 1
        } else {
            0
        };
        seen.insert(last as i64, i - 1);
        last = cur as i64;
    }
    last
}

const SENTINEL: usize = usize::MAX;

#[aoc(day15, part2, Vec)]
pub fn part2_vec(code: &Input) -> i64 {
    let mut seen = vec![SENTINEL; 30000000];
    let mut last = *code.iter().last().unwrap();
    for (idx, n) in code[..code.len() - 1].iter().enumerate() {
        // seen.insert(*n, idx);
        seen[*n as usize] = idx;
    }
    for i in code.len()..30000000 {
        let cur = if seen[last as usize] != SENTINEL {
            i - seen[last as usize] - 1
        } else {
            0
        };
        // seen.insert(last as i64, i - 1);
        seen[last as usize] = i - 1;
        last = cur as i64;
    }
    last
}
