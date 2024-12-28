/*
    Comments.
*/

use std::collections::HashSet;

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

    fn from_usize(value: usize) -> Self {
        let a = (value / 26 + 'a' as usize) as u8 as char;
        let b = (value % 26 + 'a' as usize) as u8 as char;
        Self(a, b)
    }
}

fn build_graph(data: &[Connection]) -> UnGraph<usize, ()> {
    UnGraph::<usize, ()>::from_edges(data.iter().map(|c| (c.0.as_u32(), c.1.as_u32())))
}

pub fn day_23_part_1(data: &str) -> i64 {
    let (_, data) = parse_input_data(data).expect("Failed to parse input data");

    let graph = build_graph(&data);

    graph
        .node_identifiers()
        .par_bridge()
        .map(|node| {
            let node_identifier = Identifier::from_usize(node.index());
            let node_has_t = node_identifier.0 == 't';

            let mut triangles_count = 0;
            // get neighbors
            let node_neighbors = graph.neighbors(node);
            let node_neighbors_set: HashSet<NodeIndex> = node_neighbors.clone().collect();
            for neighbor in node_neighbors {
                if neighbor > node {
                    let neighbor_identifier = Identifier::from_usize(neighbor.index());
                    let neighbor_has_t = neighbor_identifier.0 == 't';
                    let neighbor_neighbors_set: HashSet<NodeIndex> =
                        graph.neighbors(neighbor).collect();
                    let common_neighbors = node_neighbors_set.intersection(&neighbor_neighbors_set);
                    for common_neighbor in common_neighbors {
                        if *common_neighbor > neighbor {
                            let common_neighbor_identifier =
                                Identifier::from_usize(common_neighbor.index());
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

pub fn day_23_part_2(data: &str) -> String {
    let (_, data) = parse_input_data(data).expect("Failed to parse input data");
    let graph = build_graph(&data);
    /*println!(
        "{:?}",
        petgraph::dot::Dot::with_config(&graph, &[petgraph::dot::Config::EdgeNoLabel])
    );*/

    "lol".to_string()
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
