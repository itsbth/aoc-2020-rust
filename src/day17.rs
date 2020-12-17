use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

type Input = HashSet<(i64, i64, i64)>;

#[aoc_generator(day17)]
pub fn code_generator(input: &str) -> Input {
    let mut out = Input::new();
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c == '#' {
                out.insert((x as i64, y as i64, 0));
            }
        }
    }
    out
}

const NEIGHBOURS: [(i64, i64, i64); 26] = [
    (-1, -1, -1),
    (-1, -1, 0),
    (-1, -1, 1),
    (-1, 0, -1),
    (-1, 0, 0),
    (-1, 0, 1),
    (-1, 1, -1),
    (-1, 1, 0),
    (-1, 1, 1),
    (0, -1, -1),
    (0, -1, 0),
    (0, -1, 1),
    (0, 0, -1),
    (0, 0, 1),
    (0, 1, -1),
    (0, 1, 0),
    (0, 1, 1),
    (1, -1, -1),
    (1, -1, 0),
    (1, -1, 1),
    (1, 0, -1),
    (1, 0, 0),
    (1, 0, 1),
    (1, 1, -1),
    (1, 1, 0),
    (1, 1, 1),
];

fn neighbours(cube: &Input, pos: &(i64, i64, i64)) -> usize {
    NEIGHBOURS
        .iter()
        .map(|(x, y, z)| (x + pos.0, y + pos.1, z + pos.2))
        .filter(|pos| cube.contains(pos))
        .count()
}

fn step(cube: &Input) -> Input {
    let mut xmx = i64::MIN;
    let mut xmn = i64::MAX;
    let mut ymx = i64::MIN;
    let mut ymn = i64::MAX;
    let mut zmx = i64::MIN;
    let mut zmn = i64::MAX;
    for (x, y, z) in cube.iter() {
        xmx = xmx.max(*x);
        xmn = xmn.min(*x);

        ymx = ymx.max(*y);
        ymn = ymn.min(*y);

        zmx = zmx.max(*z);
        zmn = zmn.min(*z);
    }

    let mut new = Input::new();
    for x in (xmn - 1)..=(xmx + 1) {
        for y in (ymn - 1)..=(ymx + 1) {
            for z in (zmn - 1)..=(zmx + 1) {
                let pos = (x, y, z);
                let next = match (cube.contains(&pos), neighbours(cube, &pos)) {
                    (true, 2) | (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                };
                if next {
                    new.insert(pos);
                }
            }
        }
    }
    new
}

fn print(cube: &Input) {
    let mut xmx = i64::MIN;
    let mut xmn = i64::MAX;
    let mut ymx = i64::MIN;
    let mut ymn = i64::MAX;
    let mut zmx = i64::MIN;
    let mut zmn = i64::MAX;
    for (x, y, z) in cube.iter() {
        xmx = xmx.max(*x);
        xmn = xmn.min(*x);

        ymx = ymx.max(*y);
        ymn = ymn.min(*y);

        zmx = zmx.max(*z);
        zmn = zmn.min(*z);
    }

    for z in (zmn - 1)..=(zmx + 1) {
        println!("z={}", z);
        for y in (xmn - 1)..=(xmx + 1) {
            for x in (ymn - 1)..=(ymx + 1) {
                let pos = (x, y, z);
                print!("{}", if cube.contains(&pos) { '#' } else { '.' });
            }
            println!("");
        }
    }
}

#[aoc(day17, part1)]
pub fn part1(cube: &Input) -> usize {
    (0..6)
        .fold(cube.to_owned(), |c, _| {
            // print(&c);
            step(&c)
        })
        .len()
}

type Input2 = HashSet<(i64, i64, i64, i64)>;

const NEIGHBOURS2: [(i64, i64, i64, i64); 80] = [
    (-1, -1, -1, -1),
    (-1, -1, -1, 0),
    (-1, -1, -1, 1),
    (-1, -1, 0, -1),
    (-1, -1, 0, 0),
    (-1, -1, 0, 1),
    (-1, -1, 1, -1),
    (-1, -1, 1, 0),
    (-1, -1, 1, 1),
    (-1, 0, -1, -1),
    (-1, 0, -1, 0),
    (-1, 0, -1, 1),
    (-1, 0, 0, -1),
    (-1, 0, 0, 0),
    (-1, 0, 0, 1),
    (-1, 0, 1, -1),
    (-1, 0, 1, 0),
    (-1, 0, 1, 1),
    (-1, 1, -1, -1),
    (-1, 1, -1, 0),
    (-1, 1, -1, 1),
    (-1, 1, 0, -1),
    (-1, 1, 0, 0),
    (-1, 1, 0, 1),
    (-1, 1, 1, -1),
    (-1, 1, 1, 0),
    (-1, 1, 1, 1),
    (0, -1, -1, -1),
    (0, -1, -1, 0),
    (0, -1, -1, 1),
    (0, -1, 0, -1),
    (0, -1, 0, 0),
    (0, -1, 0, 1),
    (0, -1, 1, -1),
    (0, -1, 1, 0),
    (0, -1, 1, 1),
    (0, 0, -1, -1),
    (0, 0, -1, 0),
    (0, 0, -1, 1),
    (0, 0, 0, -1),
    (0, 0, 0, 1),
    (0, 0, 1, -1),
    (0, 0, 1, 0),
    (0, 0, 1, 1),
    (0, 1, -1, -1),
    (0, 1, -1, 0),
    (0, 1, -1, 1),
    (0, 1, 0, -1),
    (0, 1, 0, 0),
    (0, 1, 0, 1),
    (0, 1, 1, -1),
    (0, 1, 1, 0),
    (0, 1, 1, 1),
    (1, -1, -1, -1),
    (1, -1, -1, 0),
    (1, -1, -1, 1),
    (1, -1, 0, -1),
    (1, -1, 0, 0),
    (1, -1, 0, 1),
    (1, -1, 1, -1),
    (1, -1, 1, 0),
    (1, -1, 1, 1),
    (1, 0, -1, -1),
    (1, 0, -1, 0),
    (1, 0, -1, 1),
    (1, 0, 0, -1),
    (1, 0, 0, 0),
    (1, 0, 0, 1),
    (1, 0, 1, -1),
    (1, 0, 1, 0),
    (1, 0, 1, 1),
    (1, 1, -1, -1),
    (1, 1, -1, 0),
    (1, 1, -1, 1),
    (1, 1, 0, -1),
    (1, 1, 0, 0),
    (1, 1, 0, 1),
    (1, 1, 1, -1),
    (1, 1, 1, 0),
    (1, 1, 1, 1),
];

fn neighbours2(cube: &Input2, pos: &(i64, i64, i64, i64)) -> usize {
    NEIGHBOURS2
        .iter()
        .map(|(x, y, z, w)| (x + pos.0, y + pos.1, z + pos.2, w + pos.3))
        .filter(|pos| cube.contains(pos))
        .count()
}

fn step2(cube: &Input2) -> Input2 {
    let mut xmx = i64::MIN;
    let mut xmn = i64::MAX;
    let mut ymx = i64::MIN;
    let mut ymn = i64::MAX;
    let mut zmx = i64::MIN;
    let mut zmn = i64::MAX;
    let mut wmx = i64::MIN;
    let mut wmn = i64::MAX;
    for (x, y, z, w) in cube.iter() {
        xmx = xmx.max(*x);
        xmn = xmn.min(*x);

        ymx = ymx.max(*y);
        ymn = ymn.min(*y);

        zmx = zmx.max(*z);
        zmn = zmn.min(*z);

        wmx = wmx.max(*w);
        wmn = wmn.min(*w);
    }

    let mut new = Input2::new();
    for x in (xmn - 1)..=(xmx + 1) {
        for y in (ymn - 1)..=(ymx + 1) {
            for z in (zmn - 1)..=(zmx + 1) {
                for w in (wmn - 1)..=(wmx + 1) {
                    let pos = (x, y, z, w);
                    let next = match (cube.contains(&pos), neighbours2(cube, &pos)) {
                        (true, 2) | (true, 3) => true,
                        (false, 3) => true,
                        _ => false,
                    };
                    if next {
                        new.insert(pos);
                    }
                }
            }
        }
    }
    new
}

#[aoc(day17, part2)]
pub fn part2(cube: &Input) -> usize {
    (0..6)
        .fold(
            cube.iter()
                .map(|(x, y, z)| (*x, *y, *z, 0))
                .collect::<HashSet<_>>(),
            |c, _| {
                // print(&c);
                step2(&c)
            },
        )
        .len()
}
