/*
    Part 1 is relatively simple, I just made a map/dict to slightly
    speed up the rules retrieval.

    Part 2 was a bit more complex to do well. First-off, I didn't
    know about the Kahn's algorithm until today, which is something.

    Second, the test case was nice but the actual data was a bit nasty
    with cycles in the rule set. I thought I was going crazy, but I
    checked the r/adventofcode subreddit and someone mentionned that
    the rules wern't a DAG until you only consider the page numbers of
    a given update. The post finished with "That's just evil.", and I
    think I agree because it's only day 5 :'(.

    Anyway, a hard part 2 if you don't know about Kahn's algorithm or
    tried to reuse the Kahn's outputs between updates.
*/

use std::collections::VecDeque;

use nom::{
    bytes::complete::tag, character::complete::line_ending, combinator::map,
    multi::separated_list1, sequence::tuple, IResult,
};

#[derive(Debug)]
struct Data {
    rules: Vec<(u8, u8)>,
    updates: Vec<Vec<u8>>,
}

fn parse_rule(input: &str) -> IResult<&str, (u8, u8)> {
    map(
        tuple((
            nom::character::complete::u8,
            tag("|"),
            nom::character::complete::u8,
        )),
        |(a, _, b)| (a, b),
    )(input)
}

fn parse_update(input: &str) -> IResult<&str, Vec<u8>> {
    separated_list1(tag(","), nom::character::complete::u8)(input)
}

fn parse_input_data(input: &str) -> IResult<&str, Data> {
    map(
        tuple((
            separated_list1(line_ending, parse_rule),
            line_ending,
            line_ending,
            separated_list1(line_ending, parse_update),
        )),
        |(rules, _, _, updates)| Data { rules, updates },
    )(input)
}

fn is_valid_update(update: &Vec<u8>, faster_rules: &[Vec<u8>]) -> bool {
    // It would make sense to use a set, but the size is so small
    // that a vector is faster.
    let mut pages_to_print = (*update).clone();

    for page in update {
        // find the rules applying to this page
        let rules = &faster_rules[*page as usize];
        for rule in rules {
            // If we have yet to print a page that is in the rule
            // it means we have an invalid update.
            if pages_to_print.contains(rule) {
                return false;
            }
        }
        pages_to_print.retain(|p| p != page);
    }

    true
}

fn sort_with_lookup_table(data: &[u8], lookup_table: &[u8]) -> Vec<u8> {
    // Std fancy way
    // data.sort_unstable_by_key(|&page| lookup_table[page as usize]);

    // Variant of a count sort algorithm, simplified as data and lookup table
    // contain only distinct values.
    let mut output = Vec::with_capacity(data.len());

    let mut temp: [Option<u8>; 256] = [None; 256];
    for page in data {
        let rank = lookup_table[*page as usize];
        temp[rank as usize] = Some(*page);
    }

    for page in temp.iter().flatten() {
        output.push(*page);
    }

    output
}

fn build_faster_rules(rules: &[(u8, u8)]) -> [Vec<u8>; 256] {
    let mut faster_rules: [Vec<u8>; 256] = std::array::from_fn(|_| Vec::new());
    for (a, b) in rules {
        faster_rules[*b as usize].push(*a);
    }
    faster_rules
}

pub fn day_05_part_1(data: &str) -> i64 {
    let (_, data) = parse_input_data(data).expect("Failed to parse input data");

    // We create an array that for each page, contains the list of pages that should come before
    let faster_rules = build_faster_rules(&data.rules);

    data.updates
        .iter()
        .filter(|update| is_valid_update(update, &faster_rules))
        .map(|update| update[update.len() / 2] as i64)
        .sum()
}

// We are going for a Kahn's algorithm approach. I started to come up with
// something looking vaguely like it, but it sounded hard, so I checked the solution.
// Sorry.
fn kahn_algorithm(numbers: &[u8], rules: &[(u8, u8)]) -> (Vec<u8>, [u8; 256]) {
    let mut graph: [Vec<u8>; 256] = std::array::from_fn(|_| Vec::new());
    let mut in_degree: [u8; 256] = [0; 256];

    let mut number_presence = [false; 256];
    for number in numbers {
        number_presence[*number as usize] = true;
    }

    for (a, b) in rules {
        if number_presence[*a as usize] && number_presence[*b as usize] {
            graph[*a as usize].push(*b);
            in_degree[*b as usize] += 1;
        }
    }

    let mut queue: VecDeque<u8> = numbers
        .iter()
        .filter(|n| in_degree[**n as usize] == 0)
        .copied()
        .collect();

    let mut sorted_order = Vec::with_capacity(numbers.len());

    while !queue.is_empty() {
        let current = queue.pop_front().expect("Queue is empty !");
        sorted_order.push(current);

        for next_page in &graph[current as usize] {
            in_degree[*next_page as usize] -= 1;
            if in_degree[*next_page as usize] == 0 {
                queue.push_back(*next_page);
            }
        }
    }

    if sorted_order.len() != numbers.len() {
        panic!("We didn't find all the pages ! A cycle is present !");
    }

    let mut sort_lookup_table: [u8; 256] = [0; 256];

    for (i, page) in sorted_order.iter().enumerate() {
        sort_lookup_table[*page as usize] = i as u8;
    }

    (sorted_order, sort_lookup_table)
}

pub fn day_05_part_2(data: &str) -> i64 {
    let (_, data) = parse_input_data(data).expect("Failed to parse input data");

    let faster_rules = build_faster_rules(&data.rules);

    data.updates
        .iter()
        .filter(|update| !is_valid_update(update, &faster_rules))
        .map(|update| {
            let (_, sort_lookup_table) = kahn_algorithm(update, &data.rules);
            let sorted_update = sort_with_lookup_table(update, &sort_lookup_table);
            sorted_update[update.len() / 2] as i64
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_day_05_part_1() {
        assert_eq!(day_05_part_1(EXAMPLE), 143);
    }

    #[test]
    fn test_day_05_part_2() {
        assert_eq!(day_05_part_2(EXAMPLE), 123);
    }
}
