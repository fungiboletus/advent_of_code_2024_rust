/*
    A relatively nice day on some graphs.

    Part 1 was about finding triangles in the graph. I looked for existing
    algortihms as I expected that it's mostly something that you can't come
    up on the spot.

    Part 2 was about the maximum clique problem. I didn't know about this
    problem so it was interesting to learn about it. I then implemented an
    existing algorithm without trying to do anything fancy. I didn't find
    a rust crate to do this, so I asked Gemini-Exp-1206 to implement
    the Bron–Kerbosch algorithm with pivot.
    I hope you don't mind. I still did some adjustments and refinments.

    I assume that some bespoke algorithms could be faster for the advent of
    code inputs, but the Wikipedia article said that the Bron–Kerbosch algorithm
    with pivot is a good choice in practice. Robson's algorithm is faster
    but it looks overly complex.
*/

use std::collections::HashSet;

use itertools::Itertools;
use nom::{
    character::{
        complete::{char, line_ending, satisfy},
        is_alphabetic,
    },
    combinator::map,
    multi::separated_list0,
    sequence::tuple,
    IResult,
};
use petgraph::{
    graph::{NodeIndex, UnGraph},
    visit::IntoNodeIdentifiers,
};
use rayon::iter::{ParallelBridge, ParallelIterator};

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Identifier(char, char);
struct Connection(Identifier, Identifier);

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}
impl std::fmt::Debug for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::fmt::Display for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}-{}", self.0, self.1)
    }
}
impl std::fmt::Debug for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

fn parse_identifier(input: &str) -> IResult<&str, Identifier> {
    map(
        tuple((
            satisfy(|c| is_alphabetic(c as u8)),
            satisfy(|c| is_alphabetic(c as u8)),
        )),
        |(a, b)| Identifier(a, b),
    )(input)
}

fn parse_connection(input: &str) -> IResult<&str, Connection> {
    map(
        tuple((parse_identifier, char('-'), parse_identifier)),
        |(a, _, b)| Connection(a, b),
    )(input)
}

fn parse_input_data(data: &str) -> IResult<&str, Vec<Connection>> {
    separated_list0(line_ending, parse_connection)(data)
}

impl Identifier {
    fn as_u32(&self) -> u32 {
        (self.0 as u32 - 'a' as u32) * 26 + (self.1 as u32 - 'a' as u32)
    }

    fn from_u32(value: usize) -> Self {
        let a = (value / 26 + 'a' as usize) as u8 as char;
        let b = (value % 26 + 'a' as usize) as u8 as char;
        Self(a, b)
    }
}

fn build_graph(data: &[Connection]) -> UnGraph<u32, ()> {
    UnGraph::<u32, ()>::from_edges(data.iter().map(|c| (c.0.as_u32(), c.1.as_u32())))
}

pub fn day_23_part_1(data: &str) -> i64 {
    let (_, data) = parse_input_data(data).expect("Failed to parse input data");

    let graph = build_graph(&data);

    graph
        .node_identifiers()
        .par_bridge()
        .map(|node| {
            let node_identifier = Identifier::from_u32(node.index());
            let node_has_t = node_identifier.0 == 't';

            let mut triangles_count = 0;
            // get neighbors
            let node_neighbors = graph.neighbors(node);
            let node_neighbors_set: HashSet<NodeIndex> = node_neighbors.clone().collect();
            for neighbor in node_neighbors {
                if neighbor > node {
                    let neighbor_identifier = Identifier::from_u32(neighbor.index());
                    let neighbor_has_t = neighbor_identifier.0 == 't';
                    let neighbor_neighbors_set: HashSet<NodeIndex> =
                        graph.neighbors(neighbor).collect();
                    let common_neighbors = node_neighbors_set.intersection(&neighbor_neighbors_set);
                    for common_neighbor in common_neighbors {
                        if *common_neighbor > neighbor {
                            let common_neighbor_identifier =
                                Identifier::from_u32(common_neighbor.index());
                            let common_neighbor_has_t = common_neighbor_identifier.0 == 't';
                            if node_has_t || neighbor_has_t || common_neighbor_has_t {
                                triangles_count += 1;
                            }
                        }
                    }
                }
            }

            triangles_count
        })
        .sum()
}

fn bron_kerbosch_with_pivot_recursive(
    graph: &UnGraph<u32, ()>,
    current_clique: &mut HashSet<NodeIndex>,
    candidate_nodes: HashSet<NodeIndex>,
    excluded_nodes: HashSet<NodeIndex>,
    maximal_cliques: &mut Vec<HashSet<NodeIndex>>,
) {
    if candidate_nodes.is_empty() && excluded_nodes.is_empty() {
        maximal_cliques.push(current_clique.clone());
        return;
    }

    let pivot_node = candidate_nodes
        .iter()
        .chain(&excluded_nodes)
        .max_by_key(|&node| {
            graph
                .neighbors(*node)
                .filter(|neighbor| candidate_nodes.contains(neighbor))
                .count()
        })
        .cloned()
        .unwrap_or_else(|| *candidate_nodes.iter().next().unwrap());

    let mut candidate_nodes = candidate_nodes;
    let mut excluded_nodes = excluded_nodes;

    for selected_node in candidate_nodes
        .clone()
        .difference(&graph.neighbors(pivot_node).collect())
    {
        let neighbors_of_selected: HashSet<NodeIndex> = graph.neighbors(*selected_node).collect();
        let mut new_clique = current_clique.clone();
        new_clique.insert(*selected_node);

        let new_candidate_nodes = candidate_nodes
            .intersection(&neighbors_of_selected)
            .cloned()
            .collect();
        let new_excluded_nodes = excluded_nodes
            .intersection(&neighbors_of_selected)
            .cloned()
            .collect();

        bron_kerbosch_with_pivot_recursive(
            graph,
            &mut new_clique,
            new_candidate_nodes,
            new_excluded_nodes,
            maximal_cliques,
        );

        candidate_nodes.remove(selected_node);
        excluded_nodes.insert(*selected_node);
    }
}

fn find_all_maximal_cliques(graph: &UnGraph<u32, ()>) -> Vec<HashSet<NodeIndex>> {
    let mut maximal_cliques: Vec<HashSet<NodeIndex>> = Vec::new();
    let mut current_clique = HashSet::new();
    let candidate_nodes: HashSet<NodeIndex> = graph.node_indices().collect();
    let excluded_nodes = HashSet::new();

    bron_kerbosch_with_pivot_recursive(
        graph,
        &mut current_clique,
        candidate_nodes,
        excluded_nodes,
        &mut maximal_cliques,
    );

    maximal_cliques
}

fn find_maximal_clique(graph: &UnGraph<u32, ()>) -> Option<HashSet<NodeIndex>> {
    let maximal_cliques = find_all_maximal_cliques(graph);
    maximal_cliques
        .iter()
        .max_by_key(|clique| clique.len())
        .cloned()
}

pub fn day_23_part_2(data: &str) -> String {
    let (_, data) = parse_input_data(data).expect("Failed to parse input data");
    let graph = build_graph(&data);

    let maximal_clique =
        find_maximal_clique(&graph).expect("No maximal clique found, is the graph empty?");

    maximal_clique
        .iter()
        .map(|node| Identifier::from_u32(node.index()))
        .sorted_unstable()
        .map(|identifier| identifier.to_string())
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn test_day_23_part_1() {
        assert_eq!(day_23_part_1(EXAMPLE), 7);
    }

    #[test]
    fn test_day_23_part_2() {
        assert_eq!(day_23_part_2(EXAMPLE), "co,de,ka,ta");
    }
}
