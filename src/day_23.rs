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
    algo::connected_components,
    prelude::UnGraphMap,
    visit::{IntoNodeIdentifiers, IntoNodeReferences},
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

fn build_graph(data: &[Connection]) -> UnGraphMap<Identifier, ()> {
    UnGraphMap::<Identifier, ()>::from_edges(data.iter().map(|c| (c.0, c.1)))
}

pub fn day_23_part_1(data: &str) -> i64 {
    let (_, data) = parse_input_data(data).expect("Failed to parse input data");
    //println!("{:?}", data);

    //let graph = UnGraph::<Identifier, ()>::from_edges(data.iter().map(|c| (c.0, c.1)));

    /*let mut graph = UnGraph::<&str, ()>::default();
    let a = graph.add_node("Node A");
    let b = graph.add_node("Node B");

    let mut graph = UnGraph::<Identifier, ()>::default();
    let a = graph.add_node(Identifier('a', 'b'));
    let b = graph.add_node(Identifier('b', 'c'));*/

    /*let graph =
    UnGraph::<Identifier, ()>::from_edges(
        vec![(Identifier('a', 'b'), Identifier('b', 'c'))]
    );*/

    /*let graph = UnGraphMap::<Identifier, ()>::from_edges(vec![(
        Identifier('a', 'b'),
        Identifier('b', 'c'),
    )]);*/

    let graph = build_graph(&data);

    //println!("{:?}", graph);

    //let lol = connected_components(&graph);
    //println!("{:?}", lol);

    /*println!(
        "{:?}",
        petgraph::dot::Dot::with_config(&graph, &[petgraph::dot::Config::EdgeNoLabel])
    );*/

    //let mut triangles_count = 0;
    //let mut triangles: Vec<(Identifier, Identifier, Identifier)> = Vec::new();

    //for node in graph.nodes() {
    graph
        .nodes()
        .par_bridge()
        .map(|node| {
            //println!("{:?}", node);

            let mut triangles_count = 0;
            // get neighbors
            let node_neighbors = graph.neighbors(node);
            let node_neighbors_set: HashSet<Identifier> = node_neighbors.clone().collect();
            for neighbor in node_neighbors {
                //println!("  {:?}", neighbor);
                if neighbor > node {
                    let neighbor_neighbors_set: HashSet<Identifier> =
                        graph.neighbors(neighbor).collect();
                    let common_neighbors = node_neighbors_set.intersection(&neighbor_neighbors_set);
                    for common_neighbor in common_neighbors {
                        if *common_neighbor > neighbor
                            && (node.0 == 't' || neighbor.0 == 't' || common_neighbor.0 == 't')
                        {
                            //println!("    {:?}", common_neighbor);
                            triangles_count += 1;
                            //triangles.push((node, neighbor, *common_neighbor));
                        }
                    }
                }
            }

            triangles_count
        })
        .sum()

    //println!("{:?}", triangles);

    //triangles_count
}

pub fn day_23_part_2(data: &str) -> i64 {
    42
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
        assert_eq!(day_23_part_2(EXAMPLE), 42);
    }
}
