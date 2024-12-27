/*
    Comments.
*/

use nom::{character::complete::line_ending, multi::separated_list0, IResult};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[inline]
fn mix(a: u32, b: u32) -> u32 {
    a ^ b
}

#[inline]
fn prune(number: u32) -> u32 {
    number & 0b111111111111111111111111
}

#[inline]
fn multiply_by_64(number: u32) -> u32 {
    number << 6
}

#[inline]
fn divide_by_32(number: u32) -> u32 {
    number >> 5
}

#[inline]
fn multiply_by_2048(number: u32) -> u32 {
    number << 11
}

fn compute_next_secret(secret: u32) -> u32 {
    let mut next = prune(mix(multiply_by_64(secret), secret));
    next = prune(mix(divide_by_32(next), next));
    next = prune(mix(multiply_by_2048(next), next));
    next
}

fn parse_input_data(data: &str) -> IResult<&str, Vec<u32>> {
    separated_list0(line_ending, nom::character::complete::u32)(data)
}

pub fn day_22_part_1(data: &str) -> i64 {
    let (_, data) = parse_input_data(data).expect("Failed to parse input data");

    data.par_iter()
        .map(|seed| {
            let mut secret = *seed;
            for _ in 0..2000 {
                secret = compute_next_secret(secret);
            }
            secret as i64
        })
        .sum()
}

pub fn day_22_part_2(data: &str) -> i64 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "1
10
100
2024
";

    #[test]
    fn test_day_22_mix() {
        assert_eq!(mix(42, 15), 37);
    }

    #[test]
    fn test_day_22_prune() {
        assert_eq!(prune(100000000), 16113920);
    }

    #[test]
    fn test_day_22_compute_next_secret() {
        assert_eq!(compute_next_secret(123), 15887950);
        assert_eq!(compute_next_secret(15887950), 16495136);
        assert_eq!(compute_next_secret(16495136), 527345);
        assert_eq!(compute_next_secret(527345), 704524);
        assert_eq!(compute_next_secret(704524), 1553684);
        assert_eq!(compute_next_secret(1553684), 12683156);
        assert_eq!(compute_next_secret(12683156), 11100544);
        assert_eq!(compute_next_secret(11100544), 12249484);
        assert_eq!(compute_next_secret(12249484), 7753432);
        assert_eq!(compute_next_secret(7753432), 5908254);
    }

    #[test]
    fn test_day_22_part_1() {
        assert_eq!(day_22_part_1(EXAMPLE), 37327623);
    }

    #[test]
    fn test_day_22_part_2() {
        assert_eq!(day_22_part_2(EXAMPLE), 42);
    }
}
