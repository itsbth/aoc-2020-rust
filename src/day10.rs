use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;
use std::collections::HashMap;

#[aoc_generator(day10)]
pub fn generate(adapters: &str) -> Vec<u64> {
    adapters
        .lines()
        .map(|n| u64::from_str(n).unwrap())
        .collect()
}

#[aoc(day10, part1)]
pub fn part1(adapters: &Vec<u64>) -> u64 {
    let mut adapters = adapters.clone();
    adapters.push(0);
    adapters.sort_unstable();
    let mut dist = [0u64; 3];
    for w in adapters.windows(2) {
        let (t, n) = (w[0], w[1]);
        dist[(n - t - 1) as usize] += 1;
    }
    dist[0] * (dist[2] + 1)
}

fn neighbours<'t>(aa: &'t [bool; 256], a: usize) -> impl Iterator<Item = usize> + 't {
    (1..=3).map(move |n| n + a).filter(move |i| aa[*i])
}

fn dfs(mut cache: &mut HashMap<usize, usize>, aa: &[bool; 256], start: usize, end: usize) -> usize {
    match cache.get(&start) {
        Some(res) => return *res,
        None => {}
    }
    if start + 3 == end {
        cache.insert(start, 1);
        return 1;
    }
    let count = neighbours(aa, start).map(|n| dfs(&mut cache, aa, n, end)).sum();
    cache.insert(start, count);
    count
}

#[aoc(day10, part2)]
pub fn part2(adapters: &Vec<u64>) -> usize {
    let aa = {
        let mut aa = [false; 256]; // random number > max(adapters)
        for a in adapters.iter() {
            aa[*a as usize] = true;
        }
        aa
    };  
    let mut cache = HashMap::<usize, usize>::new();
    dfs(&mut cache, &aa, 0, *adapters.iter().max().unwrap() as usize + 3)
}
