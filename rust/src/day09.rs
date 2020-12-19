#![allow(dead_code)]

use crate::util::Matrix;
use itertools::Itertools;
use std::collections::HashSet;

fn find_sums(n: u32, others: &[u32]) -> impl Iterator<Item = u32> + '_ {
    others.iter().map(move |o| o + n)
}

struct Cache {
    inner: Matrix<u32>,
    next_replacement: usize,
}

impl Cache {
    fn pop(&mut self) {}
}

pub fn part1(input: &str) -> u64 {
    let numbers: Vec<u64> = input.lines().map(|l| l.parse().unwrap()).collect();
    for i in 25..numbers.len() {
        let found = numbers[i - 25..i]
            .iter()
            .tuple_combinations()
            .any(|(a, b)| a + b == numbers[i]);
        if !found {
            return numbers[i];
        }
    }
    unreachable!();
}

#[derive(Clone, Debug)]
struct NumberSlot {
    starting_index: usize,
    sum: u64,
    min: u64,
    max: u64,
    active: bool,
}

impl NumberSlot {
    fn new(idx: usize, value: u64) -> NumberSlot {
        NumberSlot {
            starting_index: idx,
            sum: value,
            min: value,
            max: value,
            active: true,
        }
    }
}

pub fn part2(input: &str) -> u64 {
    use std::cmp::{max, min};
    let target = part1(input);
    let numbers: Vec<u64> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut sums: Vec<NumberSlot> = numbers
        .iter()
        .enumerate()
        .map(|(i, n)| NumberSlot::new(i, *n))
        .collect();
    for stage in 1..numbers.len() {
        for i in 0..(numbers.len() - stage) {
            let curr = &mut sums[i];
            if !curr.active {
                continue;
            }
            let new = numbers[i + stage];
            curr.min = min(curr.min, new);
            curr.max = max(curr.max, new);
            curr.sum = curr.sum + numbers[i + stage];
            if curr.sum == target {
                return sums[i].min + sums[i].max;
            } else if curr.sum > target {
                curr.active = false;
            }
        }
    }
    unreachable!()
}

pub fn part2_dp(input: &str) -> u64 {
    todo!()
}
