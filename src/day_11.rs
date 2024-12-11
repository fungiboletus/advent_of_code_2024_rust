/*
    I did part 1 naively, fully expecting part 2 to be the same but worse.
    Part 2 was indeed the same but worse and it didn't scale.

    I checked the solution on r/adventofcode and slightly changed the algorithm
    to not compute the list of numbers but to compute the number of numbers
    recursively with memoization.
*/

use cached::proc_macro::cached;
use nom::{character::complete::space1, multi::separated_list1, IResult};

fn parse_input_data(data: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(space1, nom::character::complete::u64)(data)
}

#[inline]
fn nb_digits(number: u64) -> u32 {
    if number == 0 {
        return 1;
    }
    number.ilog10() + 1
}

fn split_in_two_per_digit(number: u64) -> (u64, u64) {
    let nb_digits = nb_digits(number);
    if nb_digits % 2 != 0 {
        panic!("Number of digits is not even");
    }
    let mask = 10u64.pow(nb_digits / 2);
    let left = number / mask;
    let right = number % mask;

    (left, right)
}

/*fn blink(input: &[u64]) -> Vec<u64> {
    input
        .iter()
        .flat_map(|&n| -> Box<dyn Iterator<Item = u64>> {
            if n == 0 {
                return Box::new(once(1));
            }
            if nb_digits(n) % 2 == 0 {
                let (a, b) = split_in_two_per_digit(n);
                return Box::new(once(a).chain(once(b)));
            }
            Box::new(once(n * 2024))
        })
        .collect()
}*/

#[cached]
fn blink_v2(number: u64, generation_left: usize) -> u64 {
    if generation_left == 0 {
        return 1;
    }
    let next_generation_left = generation_left - 1;
    if number == 0 {
        return blink_v2(1, next_generation_left);
    }
    if nb_digits(number) & 1 == 0 {
        let (a, b) = split_in_two_per_digit(number);
        return blink_v2(a, next_generation_left) + blink_v2(b, next_generation_left);
    }
    blink_v2(number * 2024, next_generation_left)
}

pub fn day_11_part_1(data: &str) -> i64 {
    let (_, data) = parse_input_data(data).expect("Failed to parse input data");

    data.iter().map(|&n| blink_v2(n, 25)).sum::<u64>() as i64
}

pub fn day_11_part_2(data: &str) -> i64 {
    let (_, data) = parse_input_data(data).expect("Failed to parse input data");
    data.iter().map(|&n| blink_v2(n, 75)).sum::<u64>() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "125 17";

    #[test]
    fn test_split_in_two_per_digit() {
        assert_eq!(split_in_two_per_digit(10), (1, 0));
        assert_eq!(split_in_two_per_digit(21), (2, 1));
        assert_eq!(split_in_two_per_digit(1234), (12, 34));
        assert_eq!(split_in_two_per_digit(1234567890), (12345, 67890));
        assert_eq!(split_in_two_per_digit(1000), (10, 0));
    }

    /*#[test]
    fn test_blink() {
        assert_eq!(
            blink(&[0, 1, 10, 99, 999]),
            vec![1, 2024, 1, 0, 9, 9, 2021976]
        );
    }*/

    #[test]
    fn test_day_11_part_1() {
        assert_eq!(day_11_part_1(EXAMPLE), 55312);
    }

    #[test]
    fn test_day_11_part_2() {
        assert_eq!(day_11_part_2(EXAMPLE), 65601038650482);
    }
}
