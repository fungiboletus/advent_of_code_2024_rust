/*
    Comments.
*/

use nom::{
    character::complete::{line_ending, space1},
    multi::separated_list0,
    IResult,
};

fn parse_input_data(data: &str) -> IResult<&str, Vec<Vec<i64>>> {
    separated_list0(
        line_ending,
        separated_list0(space1, nom::character::complete::i64),
    )(data)
}

pub fn day_02_part_1(data: &str) -> i64 {
    let (_, data) = parse_input_data(data).expect("Failed to parse input data");

    data.iter()
        .map(|report| {
            if report.len() < 2 {
                panic!("Invalid report length: {}", report.len());
            }
            use std::cmp::Ordering;

            let increasing = match report[0].cmp(&report[1]) {
                Ordering::Less => true,
                Ordering::Greater => false,
                Ordering::Equal => return 0, // quick exit
            };

            report.windows(2).all(|window| {
                let diff = window[1] - window[0];
                // if the direction is the same, and the difference is not too big or null
                diff != 0 && (diff > 0) == increasing && diff.abs() <= 3
            }) as i64
        })
        .sum::<i64>()
}

pub fn day_02_part_2(data: &str) -> i64 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_day_02_part_1() {
        assert_eq!(day_02_part_1(EXAMPLE), 2);
    }

    #[test]
    fn test_day_02_part_2() {
        assert_eq!(day_02_part_2(EXAMPLE), 4);
    }
}
