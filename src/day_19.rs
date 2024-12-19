/*
    Comments.
*/

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, one_of},
    combinator::map,
    error::ParseError,
    multi::{many1, separated_list1},
    sequence::tuple,
    Err, IResult, InputIter, InputLength, InputTake, Parser,
};

type Pattern = Vec<char>;

fn parse_pattern(data: &str) -> IResult<&str, Pattern> {
    many1(one_of("wubrg"))(data)
}

fn parse_patterns(data: &str) -> IResult<&str, Vec<Pattern>> {
    separated_list1(tag(", "), parse_pattern)(data)
}

fn parse_input_data(data: &str) -> IResult<&str, (Vec<Pattern>, Vec<Pattern>)> {
    map(
        tuple((
            parse_patterns,
            line_ending,
            line_ending,
            separated_list1(line_ending, parse_pattern),
        )),
        |(rules, _, _, patterns)| (rules, patterns),
    )(data)
}

pub fn day_19_part_1(data: &str) -> usize {
    let (_, (patterns, designs)) = parse_input_data(data).expect("Failed to parse input data");

    let mut regex_string = String::new();
    regex_string.push_str("^(");

    for pattern in patterns {
        for c in pattern {
            regex_string.push(c);
        }
        regex_string.push('|');
    }
    // Remove the last '|'
    regex_string.pop();
    regex_string.push_str(")*$");

    let regex = regex::Regex::new(&regex_string).expect("Failed to create regex");

    designs
        .iter()
        .filter(|d| {
            let string = d.iter().collect::<String>();
            regex.is_match(&string)
        })
        .count()
}

pub fn day_19_part_2(data: &str) -> i64 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test_day_19_part_1() {
        assert_eq!(day_19_part_1(EXAMPLE), 6);
    }

    #[test]
    fn test_day_19_part_2() {
        assert_eq!(day_19_part_2(EXAMPLE), 42);
    }
}
