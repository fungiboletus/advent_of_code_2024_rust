/*
    Comments.
*/

use std::iter::once;

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

fn blink(input: &[u64]) -> Vec<u64> {
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
}

pub fn day_11_part_1(data: &str) -> i64 {
    let (_, data) = parse_input_data(data).expect("Failed to parse input data");

    let mut v = data;
    for _ in 0..25 {
        v = blink(&v);
    }
    v.len() as i64
}

pub fn day_11_part_2(data: &str) -> i64 {
    let (_, data) = parse_input_data(data).expect("Failed to parse input data");

    let mut lengths: Vec<usize> = Vec::with_capacity(25);
    let mut v = data;
    for i in 0..75 {
        v = blink(&v);
        lengths.push(v.len());
        println!("iteration {}: {}", i, v.len());
    }

    println!("{:?}", lengths);
    v.len() as i64
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

    #[test]
    fn test_blink() {
        assert_eq!(
            blink(&[0, 1, 10, 99, 999]),
            vec![1, 2024, 1, 0, 9, 9, 2021976]
        );
    }

    #[test]
    fn test_day_11_part_1() {
        assert_eq!(day_11_part_1(EXAMPLE), 55312);
    }

    #[test]
    fn test_day_11_part_2() {
        assert_eq!(day_11_part_2(EXAMPLE), 42);
    }
}
