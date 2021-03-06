#![deny(clippy::all)]
// blame cargo-aoc
#![allow(clippy::ptr_arg)]
// #![deny(clippy::pedantic)]

use aoc_runner_derive::aoc_lib;

mod hgc;

mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;

aoc_lib! { year = 2020 }
