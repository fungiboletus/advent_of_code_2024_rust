/*
    Part 1 is relatively simple, I just made a map/dict to slightly
    speed up the rules retrieval.
*/

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

pub fn day_05_part_1(data: &str) -> i64 {
    let (_, data) = parse_input_data(data).expect("Failed to parse input data");
    println!("{:?}", data);

    // We create an array that for each page, contains the list of pages that should come before
    let mut faster_rules: [Vec<u8>; 256] = std::array::from_fn(|_| Vec::new());
    for (a, b) in data.rules {
        faster_rules[b as usize].push(a);
    }

    data.updates
        .iter()
        .filter(|update| {
            // It would make sense to use a set, but the size is so small
            // that a vector is faster.
            let mut pages_to_print = (*update).clone();

            for page in *update {
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
        })
        .map(|update| update[update.len() / 2] as i64)
        .sum()
}

pub fn day_05_part_2(data: &str) -> i64 {
    42
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
        assert_eq!(day_05_part_2(EXAMPLE), 42);
    }
}
