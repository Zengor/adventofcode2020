use petgraph::prelude::*;
use regex::Regex;
use std::collections::HashMap;

fn parse_input<'a>(input: &'a str) -> (DiGraph<u32, u32>, NodeIndex) {
    let re = Regex::new(r"(\d)* ?([a-z]* [a-z]*) bag").unwrap();
    let mut bag_graph: DiGraph<u32, u32> = DiGraph::new();
    let mut bag_index: HashMap<&str, NodeIndex> = HashMap::new();
    let mut shiny_gold_node = None;
    for line in input.lines() {
        let mut matches = re.captures_iter(line);
        // the bag this line describes has no number before it
        // but the capture group with its name is still 2
        let this_bag = &matches.next().unwrap().get(2).unwrap().as_str();
        let from_node = *bag_index
            .entry(this_bag)
            .or_insert_with(|| bag_graph.add_node(1));
        if this_bag == &"shiny gold" {
            shiny_gold_node = Some(from_node.clone());
        }
        for other in matches {
            if &other[2] == "no other" {
                break;
            }
            println!("{}", &other[0]);
            let (count, name) = (
                other.get(1).unwrap().as_str(),
                other.get(2).unwrap().as_str(),
            );
            let count: u32 = count.parse().unwrap();
            let to_node = *bag_index
                .entry(&name)
                .or_insert_with(|| bag_graph.add_node(1));
            bag_graph.add_edge(from_node, to_node, count);
        }
    }
    (bag_graph, shiny_gold_node.unwrap())
}

pub fn part1(input: &str) -> usize {
    let (graph, sg) = parse_input(input);
    graph
        .node_indices()
        .filter(|n| petgraph::algo::has_path_connecting(&graph, *n, sg, None))
        .count()
        - 1 // `has_pah_connecting` will return true for (sg, sg),
            // which doesn't count
}

pub fn part2(input: &str) -> u32 {
    let (mut graph, sg) = parse_input(input);
    let mut search = DfsPostOrder::new(&graph, sg);
    while let Some(node) = search.next(&graph) {
        let mut neighbors = graph.neighbors(node).detach();
        let mut sum = 0;
        while let Some((edge, neighbor)) = neighbors.next(&graph) {
            sum += graph[edge] * graph[neighbor];
        }
        graph[node] = 1 + sum;
    }
    graph[sg] - 1 // weight counts the node itself, but problem wants _other_
}
