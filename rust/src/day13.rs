pub fn part1(input: &str) -> u32 {
    let mut lines = input.lines();
    let my_time = lines.next().unwrap().parse::<u32>().unwrap();
    let bus_ids = lines
        .next()
        .unwrap()
        .split(",")
        .filter_map(|n| n.parse::<u32>().ok());
    let soonest = bus_ids.map(|id| id - (my_time % id)).min().unwrap();
    soonest * (soonest - (my_time % soonest))
}

pub fn part2(input: &str) -> usize {
    let mut bus_ids = input
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split(",")
        .enumerate()
        .filter_map(|(i, n)| n.parse::<usize>().ok().map(|n| (i, n)));
    let mut current = 0;
    let mut step = bus_ids.next().unwrap().1;
    for (i, id) in bus_ids {
        while (current + i) % id != 0 {
            current += step;
        }
        step *= id;
    }
    current
}
