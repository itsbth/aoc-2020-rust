use crate::hgc;
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[aoc_generator(day8)]
pub fn code_generator(input: &str) -> Vec<hgc::Op> {
    crate::hgc::parse_code(input)
        .collect::<Option<Vec<_>>>()
        .unwrap()
}

#[aoc(day8, part1)]
pub fn part1(code: &Vec<hgc::Op>) -> i64 {
    let mut program = hgc::Program::new(code.clone());
    let mut seen = HashSet::<usize>::new();
    loop {
        if seen.contains(&program.ip()) {
            return program.acc();
        }
        seen.insert(program.ip());
        program.step();
    }
}

#[aoc(day8, part2)]
pub fn part2(code: &Vec<hgc::Op>) -> i64 {
    for n in 0..code.len() {
        // println!("n: {}", n);
        let mut code = code.clone();
        {
            let no = match code[n] {
                (hgc::OpCode::Jmp, n) => (hgc::OpCode::Nop, n),
                (hgc::OpCode::Nop, n) => (hgc::OpCode::Jmp, n),
                _ => continue,
            };
            code[n] = no;
        }
        let mut program = crate::hgc::Program::new(code);
        let mut seen = HashSet::<usize>::new();
        loop {
            if seen.contains(&program.ip()) {
                // println!("loop after {}", seen.len());
                break;
            }
            seen.insert(program.ip());
            if program.step() {
                return program.acc();
            }
        }
    }
    panic!("end reached")
}
