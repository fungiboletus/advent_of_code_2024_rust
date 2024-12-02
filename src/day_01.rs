use nom::{
    character::complete::{line_ending, space1},
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};

fn parse_input_data(data: &str) -> IResult<&str, Vec<(i64, i64)>> {
    separated_list0(
        line_ending,
        separated_pair(
            nom::character::complete::i64,
            space1,
            nom::character::complete::i64,
        ),
    )(data)
}

pub fn day_01_part_1(data: &str) -> i64 {
    let (_, data) = parse_input_data(data).expect("Failed to parse input data");

    let mut left_list = data.iter().map(|(a, _)| *a).collect::<Vec<i64>>();
    let mut right_list = data.iter().map(|(_, b)| *b).collect::<Vec<i64>>();

    // Sort ascending
    left_list.sort_unstable();
    right_list.sort_unstable();

    let difference = left_list
        .iter()
        .zip(right_list)
        .fold(0, |acc, (a, b)| (a - b).abs() + acc);

    difference
}

pub fn day_01_part_2(data: &str) -> i64 {
    let (_, data) = parse_input_data(data).expect("Failed to parse input data");

    // Create a 100 000 sized array, that's about 781KB
    // We could have used a Map/Dict too, but I like the array tonight.
    let mut nb_of_appearances = vec![0_usize; 100_000];

    // Fill the array with the number of appearances
    for (_, location) in data.iter() {
        #[allow(clippy::manual_range_contains)]
        if *location < 0 || *location > 100_000 {
            panic!("Invalid location: {}", location);
        }
        nb_of_appearances[*location as usize] += 1;
    }

    let sum = data.iter().fold(0, |acc, (location, _)| {
        if *location < 0 || *location > 100_000 {
            panic!("Invalid location: {}", location);
        }
        acc + *location * nb_of_appearances[*location as usize] as i64
    });

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_day_01_part_1() {
        assert_eq!(day_01_part_1(EXAMPLE), 11);
    }

    #[test]
    fn test_day_01_part_2() {
        assert_eq!(day_01_part_2(EXAMPLE), 31);
    }
}
