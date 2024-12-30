/*
    On Day 24, I'm doing a graph as it looks like a graph problem.
    Day 23 being a relatively easy graph problem is also a clue that
    it may be a graph problem on Day 24 too.

    I did implement a cache for the recursive function, but
    I didn't want to serialise the whole graph so I wanted to use
    its hash as a cache key, hoping for the best.
    I thought that the defalut hash map implementation would be a bit
    too straightforward, so I use Blake2 to hash into a 64-bit integer.

    I don't like blake3 by the way, I rather be safe than sorry about
    the number of rounds.
*/

use std::{
    collections::{BTreeMap, HashMap},
    hash::{Hash, Hasher},
};

use cached::proc_macro::cached;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, line_ending, satisfy},
    combinator::map,
    multi::separated_list0,
    sequence::tuple,
    IResult,
};
use petgraph::{visit::IntoNodeReferences, Graph};

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct WireName(u32);

#[inline]
fn wirename_char_to_u32(c: char) -> u32 {
    // start with 0-9 and then a-z
    if c.is_ascii_digit() {
        c as u32 - '0' as u32
    } else if c.is_ascii_lowercase() {
        c as u32 - 'a' as u32 + 10
    } else {
        panic!("Invalid character: {}", c);
    }
}

#[inline]
fn wirename_u32_to_char(value: u32) -> char {
    if value < 10 {
        (value + '0' as u32) as u8 as char
    } else if value < 36 {
        (value - 10 + 'a' as u32) as u8 as char
    } else {
        panic!("Invalid value: {}", value);
    }
}

impl WireName {
    fn from_chars(a: char, b: char, c: char) -> WireName {
        WireName(
            wirename_char_to_u32(a) * 1296 // 36*36
                + wirename_char_to_u32(b) * 36
                + wirename_char_to_u32(c),
        )
    }

    fn as_u32(&self) -> u32 {
        self.0
    }

    #[allow(dead_code)]
    fn from_usize(value: usize) -> WireName {
        static MAX: u32 = 46656; // 36 * 36 * 36;
        assert!(value < MAX as usize, "Value out of range: {}", value);
        WireName(value as u32)
    }

    // True if starts by z
    fn is_output(&self) -> bool {
        self.0 / 1296 % 36 == 35
    }
}

impl std::fmt::Display for WireName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let a = wirename_u32_to_char((self.0 / 1296) % 36);
        let b = wirename_u32_to_char((self.0 / 36) % 36);
        let c = wirename_u32_to_char(self.0 % 36);
        write!(f, "{}{}{}", a, b, c)
    }
}
impl std::fmt::Debug for WireName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} ({})", self, self.as_u32())
    }
}

#[derive(Debug, Clone, Hash)]
enum Operation {
    And,
    Or,
    Xor,
}

#[derive(Clone, Hash)]
struct Gate {
    input_a: WireName,
    input_b: WireName,
    output: WireName,
    operation: Operation,
}

impl std::fmt::Debug for Gate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:?} {:?} {:?} -> {:?}",
            self.input_a, self.operation, self.input_b, self.output
        )
    }
}

#[inline]
fn parse_one_letter(input: &str) -> IResult<&str, char> {
    satisfy(|c: char| c.is_ascii_lowercase() || c.is_ascii_digit())(input)
}

fn parse_wire_name(input: &str) -> IResult<&str, WireName> {
    map(
        tuple((parse_one_letter, parse_one_letter, parse_one_letter)),
        |(a, b, c)| WireName::from_chars(a, b, c),
    )(input)
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    alt((
        map(tag("AND"), |_| Operation::And),
        map(tag("OR"), |_| Operation::Or),
        map(tag("XOR"), |_| Operation::Xor),
    ))(input)
}

fn parse_gate(input: &str) -> IResult<&str, Gate> {
    map(
        tuple((
            parse_wire_name,
            tag(" "),
            parse_operation,
            tag(" "),
            parse_wire_name,
            tag(" -> "),
            parse_wire_name,
        )),
        |(input_a, _, operation, _, input_b, _, output)| Gate {
            input_a,
            input_b,
            output,
            operation,
        },
    )(input)
}

#[derive(Debug, Hash)]
struct Problem {
    gates: Vec<Gate>,
    initial_values: BTreeMap<WireName, bool>,
}

fn parse_initial_values(input: &str) -> IResult<&str, BTreeMap<WireName, bool>> {
    map(
        separated_list0(
            line_ending,
            map(
                tuple((
                    parse_wire_name,
                    tag(": "),
                    alt((map(char('0'), |_| false), map(char('1'), |_| true))),
                )),
                |(name, _, value)| (name, value),
            ),
        ),
        |values| values.into_iter().collect(),
    )(input)
}

fn parse_input_data(data: &str) -> IResult<&str, Problem> {
    map(
        tuple((
            parse_initial_values,
            line_ending,
            line_ending,
            separated_list0(line_ending, parse_gate),
        )),
        |(initial_values, _, _, gates)| Problem {
            gates,
            initial_values,
        },
    )(data)
}

/*
struct GraphNode {
    kind: NodeType,
    value: Option<bool>,
}

impl std::fmt::Debug for GraphNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?} - ", self.kind)?;
        if let Some(value) = self.value {
            write!(f, "{:?}", value)?;
        } else {
            write!(f, "?")?;
        }
        Ok(())
    }
}
*/

struct InputNode {
    wire: WireName,
    value: bool,
}

impl std::fmt::Debug for InputNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?} - {:?}", self.wire, self.value)
    }
}

enum GraphNode {
    Gate(Gate),
    //Wire(WireName),
    Input(InputNode),
    //Output(WireName),
}

impl std::fmt::Debug for GraphNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GraphNode::Gate(gate) => write!(f, "Gate {:?}", gate),
            GraphNode::Input(wire) => write!(f, "Input {:?}", wire),
            //NodeType::Wire(wire) => write!(f, "Wire {:?}", wire),
            //NodeType::Output(wire) => write!(f, "Output {:?}", wire),
        }
    }
}

fn build_graph(data: &Problem) -> Graph<GraphNode, ()> {
    let mut graph = Graph::<GraphNode, ()>::new();

    let mut map: HashMap<WireName, petgraph::graph::NodeIndex> = HashMap::new();

    // Add the initial values as inputs
    for (gate_name, value) in &data.initial_values {
        let node_index = graph.add_node(GraphNode::Input(InputNode {
            wire: *gate_name,
            value: *value,
        }));
        map.insert(*gate_name, node_index);
    }

    // Add the gates first, so we are sure everything exists
    // before we add the edges.
    for gate in &data.gates {
        let node_index = graph.add_node(GraphNode::Gate(gate.clone()));
        map.insert(gate.output, node_index);
    }

    // Add the edges
    for gate in &data.gates {
        let node_index_gate = map[&gate.output];
        let node_index_input_a = map[&gate.input_a];
        let node_index_input_b = map[&gate.input_b];

        //graph.add_edge(node_index_input_a, node_index_gate, ());
        //graph.add_edge(node_index_input_b, node_index_gate, ());
        graph.add_edge(node_index_gate, node_index_input_a, ());
        graph.add_edge(node_index_gate, node_index_input_b, ());
    }

    // Add all the nodes
    /*for gate in &data.gates {
        graph.add_node(gate.input_a);
        graph.add_node(gate.input_b);
        graph.add_node(gate.output);
    }*/

    // Add all the edges
    //for gate in &data.gates {
    //graph.add_edge(gate.input_a, gate.output, gate.operation);
    //graph.add_edge(gate.input_b, gate.output, gate.operation);
    //}

    graph
}

#[cached(
    key = "(u64, petgraph::graph::NodeIndex)",
    convert = "{ (graph_hash, node_index.clone()) }"
)]
fn resolve_node_recursive(
    graph: &Graph<GraphNode, ()>,
    graph_hash: u64,
    node_index: petgraph::graph::NodeIndex,
) -> bool {
    let node_weight = graph.node_weight(node_index).expect("Node not found");

    match node_weight {
        GraphNode::Input(input) => input.value,
        GraphNode::Gate(gate) => {
            let neighbors = graph.neighbors(node_index).collect_tuple();
            if let Some((input_a, input_b)) = neighbors {
                let input_a = resolve_node_recursive(graph, graph_hash, input_a);
                let input_b = resolve_node_recursive(graph, graph_hash, input_b);
                match gate.operation {
                    Operation::And => input_a && input_b,
                    Operation::Or => input_a || input_b,
                    Operation::Xor => input_a ^ input_b,
                }
            } else {
                panic!("Invalid gate");
            }
            /*let input_a = resolve_node_recursive(graph, node_index);
            let input_b = resolve_node_recursive(graph, node_index);
            match gate.operation {
                Operation::And => input_a && input_b,
                Operation::Or => input_a || input_b,
                Operation::Xor => input_a ^ input_b,
            }*/
        }
    }
}

/* somewhat over-engineered */
fn get_problem_hash(data: &Problem) -> u64 {
    use blake2::Blake2s256;
    use blake2::Digest;

    struct Blake2Hasher(blake2::Blake2s256);

    impl Blake2Hasher {
        fn new() -> Self {
            Self(Blake2s256::new())
        }
    }

    impl Hasher for Blake2Hasher {
        fn write(&mut self, bytes: &[u8]) {
            self.0.update(bytes);
        }

        fn finish(&self) -> u64 {
            let result = self.0.clone().finalize();
            let mut output = [0u8; 8];
            output.copy_from_slice(&result[..8]); // Take the first 8 bytes
            u64::from_be_bytes(output)
        }
    }

    use std::hash::Hash;
    let mut hasher = Blake2Hasher::new();
    data.hash(&mut hasher);
    hasher.finish()
}

pub fn day_24_part_1(data: &str) -> i64 {
    let (_, data) = parse_input_data(data).expect("Failed to parse input data");

    // We need a good hash to use as a cache key
    let hash = get_problem_hash(&data);

    let graph = build_graph(&data);

    // get all the output nodes first before sorting them
    let mut outputs = graph
        .node_references()
        .filter(|(_, weight)| match &weight {
            GraphNode::Gate(gate) => gate.output.is_output(),
            _ => false,
        })
        .collect::<Vec<_>>();

    outputs.sort_unstable_by_key(|(_, weight)| match weight {
        GraphNode::Gate(gate) => std::cmp::Reverse(gate.output),
        _ => panic!("Invalid node"),
    });

    outputs
        .iter()
        .map(|(node, _)| resolve_node_recursive(&graph, hash, *node))
        // Build back the binary number
        .fold(0, |acc, value| acc << 1 | value as i64)
}

pub fn day_24_part_2(data: &str) -> i64 {
    42
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    const EXAMPLE_SHORT: &str = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";

    const EXAMPLE_LONG: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    // Just a quick test as I don't trust myself enough :D
    #[test]
    fn test_day_24_all_unique_wire_names() {
        let mut wire_names: HashSet<String> = HashSet::new();

        for i in 0..46656 {
            let wire_name = WireName::from_usize(i);
            let wire_name_str = wire_name.to_string();
            assert!(
                !wire_names.contains(&wire_name_str),
                "Duplicate wire name: {}",
                wire_name_str
            );
            wire_names.insert(wire_name_str);
        }

        let mut visited_digits = [false; 46656];

        for wire_name_str in wire_names {
            let mut chars = wire_name_str.chars();
            let wire_name = WireName::from_chars(
                chars.next().unwrap(),
                chars.next().unwrap(),
                chars.next().unwrap(),
            );
            let wire_name_value = wire_name.as_u32();
            assert!(
                wire_name_value < 46656,
                "Invalid wire name value: {} ({})",
                wire_name,
                wire_name_value
            );
            assert!(
                !visited_digits[wire_name_value as usize],
                "Duplicate wire name value: {} ({})",
                wire_name, wire_name_value
            );
            visited_digits[wire_name_value as usize] = true;
        }
    }

    #[test]
    fn test_day_24_part_1() {
        assert_eq!(day_24_part_1(EXAMPLE_SHORT), 4);
        assert_eq!(day_24_part_1(EXAMPLE_LONG), 2024);
    }

    #[test]
    fn test_day_24_part_2() {
        assert_eq!(day_24_part_2(EXAMPLE_SHORT), 42);
    }
}
