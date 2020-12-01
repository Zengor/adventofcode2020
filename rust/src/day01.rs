use std::collections::{HashMap, HashSet};

pub fn part1(input: &str) -> u32 {
    let nums: HashSet<u32> = input.lines().map(|l| l.parse().unwrap()).collect();
    for num in nums.iter() {
        let other = 2020 - num;
        if nums.contains(&other) {
            return num * other;
        }
    }
    unreachable!("It's assumed there is a solution in the list");
}

pub fn part1_combinations(input: &str) -> u32 {
    use itertools::Itertools;
    let nums: Vec<u32> = input.lines().map(|l| l.parse().unwrap()).collect();
    for (a, b) in nums.iter().tuple_combinations() {
        if a + b == 2020 {
            return a * b;
        }
    }
    unreachable!("It's assumed there is a solution in the list");
}

pub fn part1_regular_loop(input: &str) -> u32 {
    let nums: Vec<u32> = input.lines().map(|l| l.parse().unwrap()).collect();
    for (i, x) in nums.iter().enumerate() {
        for y in nums[i..].iter() {
            if x + y == 2020 {
                return x * y;
            }
        }
    }
    unreachable!("It's assumed there is a solution in the list");
}

pub fn part2_combinations(input: &str) -> u32 {
    use itertools::Itertools;
    let nums: Vec<u32> = input.lines().map(|l| l.parse().unwrap()).collect();
    for (a, b, c) in nums.iter().tuple_combinations() {
        if a + b + c == 2020 {
            return a * b * c;
        }
    }
    unreachable!("It's assumed there is a solution in the list");
}

pub fn part2(input: &str) -> u32 {
    let nums: HashMap<u32, u32> = input
        .lines()
        .map(|l| l.parse().unwrap())
        .map(|n| (n, 2020 - n))
        .collect();
    for (_num, _matching_sum) in nums.iter() {
        todo!()
    }
    unreachable!("It's assumed there is a solution in the list");
}
