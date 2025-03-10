/*
    After a stupid brute force attempt,
    I came up with the idea of checking if it can be devided
    by the last number to trim the number of solutions.

    But I got a bit stuck because my code worked on the example
    and not on the actual problem. So I found this neat solution:
     - https://github.com/mkeeter/advent-of-code/blob/main/2024/07/src/lib.rs
     - I did forget to consider quite a few things.
     - *thanks*.

*/

use nom::{
    bytes::complete::tag, character::complete::line_ending, combinator::map,
    multi::separated_list1, sequence::tuple, IResult,
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn rec_look_for_solutions(current: u64, target: u64, numbers: &[u64], concat: bool) -> bool {
    if let Some((last, rest)) = numbers.split_last() {
        // If we can substract
        if current > *last && rec_look_for_solutions(current - last, target, rest, concat) {
            return true;
        }

        // If we can devide
        if current % *last == 0 && rec_look_for_solutions(current / last, target, rest, concat) {
            return true;
        }

        if concat {
            // fancy way to get a base 10 number with the same place value
            let place_value = 10_u64.pow(last.ilog10() + 1);
            // If it finishes with the last number, it's potentially a concatenation
            if current % place_value == *last {
                return rec_look_for_solutions(current / place_value, target, rest, concat);
            }
        }

        return false;
    }
    current == target
}

fn is_valid_case(target: u64, numbers: &[u64], concat: bool) -> bool {
    if numbers.is_empty() {
        panic!("We need at least one number");
    }
    rec_look_for_solutions(target, numbers[0], &numbers[1..], concat)
}

fn parse_input_data(data: &str) -> IResult<&str, Vec<(u64, Vec<u64>)>> {
    separated_list1(
        line_ending,
        map(
            tuple((
                nom::character::complete::u64,
                tag(": "),
                separated_list1(tag(" "), nom::character::complete::u64),
            )),
            |(a, _, b)| (a, b),
        ),
    )(data)
}

pub fn day_07_part_1(data: &str) -> i64 {
    let (_, data) = parse_input_data(data).expect("Failed to parse input data");

    data.par_iter()
        //.skip(849)
        .filter(|(target, numbers)| is_valid_case(*target, numbers, false))
        .map(|(target, _)| *target)
        .sum::<u64>() as i64
}

pub fn day_07_part_2(data: &str) -> i64 {
    let (_, data) = parse_input_data(data).expect("Failed to parse input data");

    data.par_iter()
        .filter(|(target, numbers)| is_valid_case(*target, numbers, true))
        .map(|(target, _)| *target)
        .sum::<u64>() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_day_07_part_1() {
        assert_eq!(day_07_part_1(EXAMPLE), 3749);
    }

    #[test]
    fn test_day_07_part_2() {
        assert_eq!(day_07_part_2(EXAMPLE), 11387);
    }
}
