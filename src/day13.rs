use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;
use num::integer::lcm;

type Input = (i64, Vec<Option<i64>>);

#[aoc_generator(day13)]
pub fn generate(input: &str) -> Input {
    let l: Vec<&str> = input.lines().collect();
    let dt = i64::from_str(l[0]).unwrap();
    let buses = l[1].split(',').map(|n| {
        if n == "x" {
            None
        } else {
            i64::from_str(n).ok()
        }
    });
    (dt, buses.collect())
}

#[aoc(day13, part1)]
pub fn part1(input: &Input) -> i64 {
    let (dt, buses) = input;
    let (b, t) = buses
        .iter()
        .flat_map(|b| b.map(|b| (b, b - (dt % b))))
        .fold(
            (-1, i64::MAX),
            |(b, t), (nb, nt)| if nt < t { (nb, nt) } else { (b, t) },
        );
    println!("b:{}, t:{}", b, t);
    b * t
}

// heavily inspired by / nabbed from https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod: i64 = modulii.iter().product();
    let mut sum = 0;
    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod /modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }
    Some(sum % prod)
}

#[aoc(day13, part2)]
pub fn part2(input: &Input) -> i64 {
    let mut ns = Vec::<i64>::new();
    let mut gs = Vec::<i64>::new();
    for (c, n) in input.1.iter().enumerate() {
        if let Some(n) = n {
            ns.push(*n);
            gs.push((n - c as i64) % n);
        }
    }
    chinese_remainder(&gs[..], &ns[..]).unwrap()
}

#[aoc(day13, part2, Sharp)]
pub fn part2_sharp(input: &Input) -> i64 {
    let mut ns = Vec::<i64>::new();
    let mut gs = Vec::<i64>::new();
    for (c, n) in input.1.iter().enumerate() {
        if let Some(n) = n {
            ns.push(*n);
            gs.push(c as i64);
        }
    }
    let mut cur = 0;
    loop {
        let valid = ns.iter().zip(gs.iter()).filter(|(&n, &o)| (cur + o) % n == 0).collect::<Vec<_>>();
        if valid.len() == ns.len() { return cur; }
        cur += valid.iter().map(|(n, _)| n).fold(1, |a, b| lcm(a, **b));
    }
}
