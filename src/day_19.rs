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

// Nom 8 should support this and has its first beta a few days ago
// but we stick to nom 7 and implement it ourselves
fn alt_on_vec<F, Input, Output, Error>(
    mut parsers: Vec<F>,
) -> impl FnMut(Input) -> IResult<Input, Output, Error>
where
    Input: Clone, // + InputIter + InputTake + InputLength,
    Error: ParseError<Input>,
    F: Parser<Input, Output, Error>,
{
    move |input: Input| {
        for parser in parsers.iter_mut() {
            let i = input.clone();
            match parser.parse(i) {
                Ok(result) => return Ok(result),
                Err(_) => continue,
            }
        }
        Err(nom::Err::Error(Error::from_error_kind(
            input,
            nom::error::ErrorKind::Alt,
        )))
    }
}

pub fn day_19_part_1(data: &str) -> i64 {
    let (_, data) = parse_input_data(data).expect("Failed to parse input data");
    println!("{:?}", data);

    let a = tag::<&str, &str, nom::error::Error<&str>>("a");
    let b = tag::<&str, &str, nom::error::Error<&str>>("b");
    let c = tag::<&str, &str, nom::error::Error<&str>>("c");

    //alt((a, b, c))("c");
    let parsers = vec![a, b, c];
    let mut alt_parser = alt_on_vec(parsers);

    let result = alt_parser("c");
    println!("{:?}", result);
    let result = alt_parser("d");
    println!("{:?}", result);

    42
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
