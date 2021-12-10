use aoc_2021::get_input;
use petgraph::algo::TarjanScc;
use petgraph::stable_graph::{NodeIndex, StableUnGraph};

type Graph = StableUnGraph<u32, ()>;
fn node(x: usize) -> NodeIndex {
    NodeIndex::new(x as usize)
}

fn part1(graph: &Graph) -> u32 {
    graph
        .node_indices()
        .filter(|n| graph[*n] < graph[graph.neighbors(*n).min_by_key(|n| graph[*n]).unwrap()])
        .map(|n| graph[n] + 1)
        .sum()
}

fn part2(mut graph: Graph) -> usize {
    let v = graph
        .node_indices()
        .filter(|n| graph[*n] == 9)
        .collect::<Vec<_>>();
    for n in v {
        graph.remove_node(n);
    }

    let mut sizes = vec![];
    TarjanScc::new().run(&graph, |component| sizes.push(component.len()));
    sizes.sort_unstable();
    sizes.reverse();
    sizes[0] * sizes[1] * sizes[2]
}

fn main() {
    let input = get_input!(|s| s
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>());
    let mut graph = Graph::default();
    for weight in input.concat() {
        graph.add_node(weight);
    }
    let width = input[0].len();
    let max = width * input.len() - 1;
    let nodes = graph.node_indices().collect::<Vec<_>>();
    for n in nodes {
        if n.index() >= width {
            graph.add_edge(n, node(n.index() - width), ());
        }
        if n.index() <= max - width {
            graph.add_edge(n, node(n.index() + width), ());
        }
        if n.index() % width != 0 {
            graph.add_edge(n, node(n.index() - 1), ());
        }
        if n.index() % width != width - 1 {
            graph.add_edge(n, node(n.index() + 1), ());
        }
    }

    println!("part1: {}", part1(&graph));
    println!("part2: {}", part2(graph));
}
