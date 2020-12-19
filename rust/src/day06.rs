use itertools::Itertools;
use std::collections::HashMap;

pub fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .flat_map(|g| g.chars().filter(|c| !c.is_whitespace()).unique())
        .count()
}

pub fn part2(input: &str) -> usize {
    let groups = input.split("\n\n");
    let mut counts: HashMap<char, usize> = HashMap::new();
    let mut final_sum: usize = 0;
    for group in groups {
        let mut group_size = 0;
        counts.clear();
        for person in group.lines() {
            group_size += 1;
            for c in person.chars().unique() {
                counts.entry(c).and_modify(|n| *n += 1).or_insert(1);
            }
            println!("{:?}", counts);
        }
        final_sum += counts.values().filter(|&&n| n == group_size).count();
        println!("{}", final_sum);
    }
    final_sum
}

// fn count_chosen_by_all(group: &str) -> usize {}
// pub fn part2_alt(input: &str) -> usize {
//     input.split("\n\n").map(|group| group.lines().map)
// }
