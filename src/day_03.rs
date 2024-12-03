/*
    The virtual machine is coming!

    The first part can be solved trivially with a regex, but we will do the parsing using nom.

    It's a bit hell because nom doesn't have a take_until working on parsers.
*/

use nom::{
    bytes::complete::{tag, take, take_while_m_n},
    combinator::map,
    error::ParseError,
    multi::many0,
    sequence::{preceded, tuple},
    IResult, InputIter, InputLength, InputTake, Parser, ToUsize,
};

fn mul_number(input: &str) -> IResult<&str, u16> {
    map(
        take_while_m_n(1, 3, |char: char| char.is_ascii_digit()),
        |digits: &str| {
            digits.chars().fold(0_u16, |acc, digit| {
                acc * 10 + (digit as u16).wrapping_sub('0' as u16)
            })
        },
    )(input)
}

fn parse_mul(input: &str) -> IResult<&str, (u16, u16)> {
    map(
        tuple((tag("mul("), mul_number, tag(","), mul_number, tag(")"))),
        |(_, a, _, b, _)| (a, b),
    )(input)
}

/**
 * Complete overkill parser, but it works.
 */
fn parse_with_skip_up_to_n<C, F, Input, Output, Error>(
    up_to: C,
    mut parser: F,
) -> impl FnMut(Input) -> IResult<Input, Output, Error>
where
    Input: Clone + InputIter + InputTake + InputLength,
    C: ToUsize,
    Error: ParseError<Input>,
    F: Clone + Parser<Input, Output, Error>,
{
    let up_to = up_to.to_usize();

    move |input: Input| {
        let up_to = up_to.min(input.input_len());
        let i = input.clone();
        if let Ok((left, result)) = parser.parse(input.clone()) {
            return Ok((left, result));
        }
        for n_skip in 1..=up_to {
            let ii = input.clone();
            if let Ok((left, result)) = preceded(take(n_skip), parser.clone()).parse(ii) {
                return Ok((left, result));
            };
        }
        Err(nom::Err::Error(Error::from_error_kind(
            i,
            nom::error::ErrorKind::Eof,
        )))
    }
}

fn parse_input_data(input: &str) -> IResult<&str, Vec<(u16, u16)>> {
    many0(parse_with_skip_up_to_n(1024_usize, parse_mul))(input)
}

pub fn day_03_part_1(data: &str) -> i64 {
    let (_, muls) = parse_input_data(data).expect("Failed to parse input data");

    muls.iter().map(|(a, b)| (*a as i64) * (*b as i64)).sum()
}

pub fn day_03_part_2(data: &str) -> i64 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn test_mul_number() {
        assert_eq!(mul_number("1"), Ok(("", 1)));
        assert_eq!(mul_number("123"), Ok(("", 123)));
        assert_eq!(mul_number("1234"), Ok(("4", 123)));
        assert_eq!(mul_number("123abc"), Ok(("abc", 123)));
        assert!(mul_number("abc123").is_err());
        assert_eq!(mul_number("123"), Ok(("", 123)));
    }

    #[test]
    fn test_parse_mul() {
        assert_eq!(parse_mul("mul(123,456)"), Ok(("", (123, 456))));
        assert_eq!(parse_mul("mul(123,456)"), Ok(("", (123, 456))));
        assert!(parse_mul("mul(123,456").is_err());
    }

    #[test]
    fn test_parse_input_data() {
        assert_eq!(
            parse_input_data("mul(123,456)"),
            Ok(("", (vec![(123, 456)])))
        );
        assert_eq!(
            parse_input_data("xmul(123,456)%&mul(123,456)("),
            Ok(("(", (vec![(123, 456), (123, 456)])))
        );
    }

    #[test]
    fn test_day_03_part_1() {
        assert_eq!(day_03_part_1("mul(2,4)"), 8);
        assert_eq!(day_03_part_1("xmul(2,4)"), 8);
        assert_eq!(day_03_part_1("xmul(2,4)%&mul(3,7)"), 29);
        assert_eq!(day_03_part_1(EXAMPLE), 161);
    }

    #[test]
    fn test_day_03_part_2() {
        assert_eq!(day_03_part_2(EXAMPLE), 42);
    }
}
