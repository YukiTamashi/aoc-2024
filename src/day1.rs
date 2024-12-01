use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use winnow::{
    ascii::{digit1, space0},
    combinator::separated_pair,
    PResult, Parser,
};

#[derive(Clone)]
pub struct Input {
    left: Vec<u32>,
    right: Vec<u32>,
}

impl Input {
    fn sorted(&self) -> Input {
        let left = self.left.iter().copied().sorted().collect();
        let right = self.right.iter().copied().sorted().collect();
        Input { left, right }
    }

    fn diff(self) -> u32 {
        self.left
            .iter()
            .zip(self.right)
            .map(|(v1, v2)| v1.abs_diff(v2))
            .sum()
    }

    fn similarity(&self) -> u32 {
        let frequency =
            self.right
                .iter()
                .fold(HashMap::with_capacity(self.right.len()), |mut map, x| {
                    map.entry(x).and_modify(|x| *x += 1).or_insert(1);
                    map
                });
        self.left
            .iter()
            .map(|i| frequency.get(i).unwrap_or(&0) * i)
            .sum()
    }
}

fn parse_num(input: &mut &str) -> PResult<u32> {
    digit1.parse_to().parse_next(input)
}

#[aoc_generator(day1)]
fn input_generator(input: &str) -> Input {
    let (left, right): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| {
            separated_pair(parse_num, space0, parse_num)
                .parse_next(&mut line.trim())
                .unwrap()
            //This is non winnow solution, somehow works faster for single solution
            //But its slower for benchmarking
            /*line.split_whitespace()
            .flat_map(str::parse::<u32>)
            .collect_tuple()//from itertools
            .unwrap()*/
        })
        .unzip();
    Input { left, right }
}
#[aoc(day1, part1)]
pub fn solve_day1(input: &Input) -> u32 {
    //Tried paralellization, apparently for this input size it is slower than sequential
    input.sorted().diff()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &Input) -> u32 {
    input.similarity()
}
