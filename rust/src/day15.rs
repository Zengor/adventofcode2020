use std::collections::{hash_map::Entry, HashMap};

fn game_to_turn(mut starting: Vec<usize>, final_turn: usize) -> usize {
    let mut curr = starting.pop().unwrap();
    let mut history: HashMap<usize, usize> = starting
        .into_iter()
        .enumerate()
        .map(|(i, n)| (n, i + 1))
        .collect();
    for turn in history.len() + 1..final_turn {
        curr = match history.entry(curr) {
            Entry::Occupied(mut e) => turn - e.insert(turn),
            Entry::Vacant(e) => {
                e.insert(turn);
                0
            }
        }
    }
    curr
}
pub fn part1(input: &str) -> usize {
    let starting = input
        .trim()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();
    game_to_turn(starting, 2020)
}

pub fn part2(input: &str) -> usize {
    let starting = input
        .trim()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();
    game_to_turn(starting, 30000000)
}
