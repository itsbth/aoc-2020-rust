use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

#[derive(Debug)]
pub enum Command {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

impl FromStr for Command {
    type Err = ();
    fn from_str(c: &str) -> Result<Self, Self::Err> {
        match c {
            "N" => Ok(Command::North),
            "S" => Ok(Command::South),
            "E" => Ok(Command::East),
            "W" => Ok(Command::West),
            "L" => Ok(Command::Left),
            "R" => Ok(Command::Right),
            "F" => Ok(Command::Forward),
            _ => Err(()),
        }
    }
}

type Input = Vec<(Command, i64)>;

#[aoc_generator(day12)]
pub fn generate(commands: &str) -> Input {
    commands
        .lines()
        .map(|n| {
            let (c, n) = n.split_at(1);
            (Command::from_str(c).unwrap(), i64::from_str(n).unwrap())
        })
        .collect()
}

// E S W N
const FD: [(i64, i64); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

#[aoc(day12, part1)]
pub fn part1(commands: &Input) -> i64 {
    let (x, y, _) = commands.iter().fold((0, 0, 0), |(x, y, f), cmd| match cmd {
        (Command::North, n) => (x, y - n, f),
        (Command::South, n) => (x, y + n, f),
        (Command::East, n) => (x - n, y, f),
        (Command::West, n) => (x + n, y, f),
        (Command::Right, n) => (x, y, f + n / 90),
        (Command::Left, n) => (x, y, f - n / 90),
        (Command::Forward, n) => {
            let (dx, dy) = FD[(((f % 4) + 4) % 4) as usize];
            (x + dx * n, y + dy * n, f)
        }
    });
    println!("{}, {}", x, y);
    x.abs() + y.abs()
}

#[derive(Debug)]
struct Ship {
    pos: (i64, i64),
    wp: (i64, i64),
}

impl Ship {
    fn new() -> Self {
        Self {
            pos: (0, 0),
            wp: (-10, -1),
        }
    }
    fn forward(&self, n: i64) -> Self {
        Self {
            pos: (self.pos.0 + self.wp.0 * n, self.pos.1 + self.wp.1 * n),
            wp: self.wp,
        }
    }
    fn left(&self, n: i64) -> Self {
        let (mut wx, mut wy) = self.wp;
        for _ in 0..n {
            let ox = wx;
            wx = -wy;
            wy = ox;
        }
        Self {
            pos: self.pos,
            wp: (wx, wy),
        }
    }
    fn right(&self, n: i64) -> Self {
        let (mut wx, mut wy) = self.wp;
        for _ in 0..n {
            let ox = wx;
            wx = wy;
            wy = -ox;
        }
        Self {
            pos: self.pos,
            wp: (wx, wy),
        }
    }
    fn north(&self, n: i64) -> Self {
        Self {
            wp: (self.wp.0, self.wp.1 - n),
            pos: self.pos,
        }
    }
    fn south(&self, n: i64) -> Self {
        Self {
            wp: (self.wp.0, self.wp.1 + n),
            pos: self.pos,
        }
    }
    fn east(&self, n: i64) -> Self {
        Self {
            wp: (self.wp.0 - n, self.wp.1),
            pos: self.pos,
        }
    }
    fn west(&self, n: i64) -> Self {
        Self {
            wp: (self.wp.0 + n, self.wp.1),
            pos: self.pos,
        }
    }
    fn distance(&self) -> i64 {
        self.pos.0.abs() + self.pos.1.abs()
    }
}

#[aoc(day12, part2)]
pub fn part2(commands: &Input) -> i64 {
    commands
        .iter()
        .fold(Ship::new(), |ship, cmd| match cmd {
            (Command::North, n) => ship.north(*n),
            (Command::South, n) => ship.south(*n),
            (Command::East, n) => ship.east(*n),
            (Command::West, n) => ship.west(*n),
            (Command::Right, n) => ship.right(*n / 90),
            (Command::Left, n) => ship.left(*n / 90),
            (Command::Forward, n) => ship.forward(*n),
        })
        .distance()
}
