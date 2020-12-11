use aoc_runner_derive::{aoc, aoc_generator};
// use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}

impl FromStr for Seat {
    type Err = ();
    fn from_str(s: &str) -> Result<Seat, Self::Err> {
        match s {
            "." => Ok(Self::Floor),
            "L" => Ok(Self::Empty),
            "#" => Ok(Self::Occupied),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Ferry {
    sx: usize,
    sy: usize,
    data: Vec<Seat>,
}

const NEIGHBOURS: [(i64, i64); 8] = [
    (-1, -1),
    (-1, -0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

impl Ferry {
    fn new(sx: usize, sy: usize, data: Vec<Seat>) -> Self {
        Self { sx, sy, data }
    }
    fn get(&self, x: i64, y: i64) -> Option<Seat> {
        if x < 0 || x >= self.sx as i64 {
            return None;
        }
        if y < 0 || y >= self.sy as i64 {
            return None;
        }
        Some(self.data[(y * self.sx as i64 + x) as usize])
    }
    fn enumerate<'t>(&'t self) -> impl Iterator<Item = (usize, usize, Seat)> + 't {
        (0..self.data.len()).map(move |idx| (idx % self.sx, idx / self.sx, self.data[idx]))
    }
    fn occupied(&self, x: usize, y: usize) -> usize {
        NEIGHBOURS
            .iter()
            .filter(|(xo, yo)| self.get(x as i64 + xo, y as i64 + yo) == Some(Seat::Occupied))
            .count()
    }
    fn step(&self) -> Self {
        Ferry {
            sx: self.sx,
            sy: self.sy,
            data: self
                .enumerate()
                .map(|(x, y, s)| match (s, self.occupied(x, y)) {
                    (Seat::Empty, 0) => Seat::Occupied,
                    (Seat::Occupied, n) if n >= 4 => Seat::Empty,
                    (s, _) => s,
                })
                .collect(),
        }
    }
    fn get_first(&self, x: i64, y: i64, xv: i64, yv: i64) -> Option<Seat> {
        let mut t = 0;
        loop {
            t += 1;
            match self.get(x + xv * t, y + yv * t) {
                Some(Seat::Floor) => continue,
                s => return s,
            }
        }
    }
    fn occupied2(&self, x: usize, y: usize) -> usize {
        NEIGHBOURS
            .iter()
            .filter(|(xo, yo)| self.get_first(x as i64, y as i64, *xo, *yo) == Some(Seat::Occupied))
            .count()
    }
    fn step2(&self) -> Self {
        Ferry {
            sx: self.sx,
            sy: self.sy,
            data: self
                .enumerate()
                .map(|(x, y, s)| match (s, self.occupied2(x, y)) {
                    (Seat::Empty, 0) => Seat::Occupied,
                    (Seat::Occupied, n) if n >= 5 => Seat::Empty,
                    (s, _) => s,
                })
                .collect(),
        }
    }
}

#[aoc_generator(day11)]
pub fn generate(ferry: &str) -> Ferry {
    let grid = ferry
        .lines()
        .flat_map(|l| {
            l.chars()
                .map(|c| Seat::from_str(c.to_string().as_str()).unwrap())
        })
        .collect::<Vec<Seat>>();
    let sx = ferry.lines().next().map(|l| l.len()).unwrap();
    let sy = ferry.lines().count();
    Ferry::new(sx, sy, grid)
}

#[aoc(day11, part1)]
pub fn part1(ferry: &Ferry) -> usize {
    let mut ferry: Ferry = ferry.clone();
    loop {
        let nf = ferry.step();
        if ferry == nf {
            return nf.enumerate().filter(|(_, _, s)| s == &Seat::Occupied).count();
        }
        ferry = nf;
    }
}

#[aoc(day11, part2)]
pub fn part2(ferry: &Ferry) -> usize {
    let mut ferry: Ferry = ferry.clone();
    loop {
        let nf = ferry.step2();
        if ferry == nf {
            return nf.enumerate().filter(|(_, _, s)| s == &Seat::Occupied).count();
        }
        ferry = nf;
    }
}
