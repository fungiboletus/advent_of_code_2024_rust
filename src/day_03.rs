/*
    The virtual machine is coming!

    The parsing can be solved trivially with a regex, but we will do the parsing using nom.

    It's a bit hell because nom doesn't have a take_until working on parsers, only on a tag.

    So I made my own utility function, name parse_and_skip_up_to_n.
    Such a beautiful function name and what a wonderful signature.

    I also didn't notice the use of a different example in part 2.
*/

use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while_m_n},
    combinator::map,
    error::ParseError,
    multi::many0,
    sequence::{preceded, tuple},
    IResult, InputIter, InputLength, InputTake, Parser, ToUsize,
};

#[derive(Debug, PartialEq)]
struct MulInstruction(u16, u16);

#[derive(Debug, PartialEq)]
enum Instruction {
    Mul(MulInstruction),
    Do,
    Dont,
}

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

fn parse_mul(input: &str) -> IResult<&str, Instruction> {
    map(
        tuple((tag("mul("), mul_number, tag(","), mul_number, tag(")"))),
        |(_, a, _, b, _)| Instruction::Mul(MulInstruction(a, b)),
    )(input)
}

fn parse_do(input: &str) -> IResult<&str, Instruction> {
    map(tag("do()"), |_| Instruction::Do)(input)
}

fn parse_dont(input: &str) -> IResult<&str, Instruction> {
    map(tag("don't()"), |_| Instruction::Dont)(input)
}

/**
 * Complete overkill parser, but it works.
 */
fn parse_and_skip_up_to_n<C, F, Input, Output, Error>(
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

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((parse_mul, parse_do, parse_dont))(input)
}

fn parse_input_data(input: &str) -> IResult<&str, Vec<Instruction>> {
    many0(parse_and_skip_up_to_n(1024_usize, parse_instruction))(input)
}

pub fn day_03_part_1(data: &str) -> i64 {
    let (_, instructions) = parse_input_data(data).expect("Failed to parse input data");

    instructions
        .iter()
        .map(|instruction| match instruction {
            Instruction::Mul(MulInstruction(a, b)) => (*a as i64) * (*b as i64),
            _ => 0,
        })
        .sum()
}

pub fn day_03_part_2(data: &str) -> i64 {
    let (_, instructions) = parse_input_data(data).expect("Failed to parse input data");

    let mut mul_enabled = true;
    let mut sum = 0;

    instructions
        .iter()
        .for_each(|instruction| match instruction {
            Instruction::Mul(MulInstruction(a, b)) => {
                if mul_enabled {
                    sum += (*a as i64) * (*b as i64);
                }
            }
            Instruction::Do => {
                mul_enabled = true;
            }
            Instruction::Dont => {
                mul_enabled = false;
            }
        });

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PART_1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const EXAMPLE_PART_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

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
        assert_eq!(
            parse_mul("mul(123,456)"),
            Ok(("", Instruction::Mul(MulInstruction(123, 456))))
        );
        assert_eq!(
            parse_mul("mul(123,456)"),
            Ok(("", Instruction::Mul(MulInstruction(123, 456))))
        );
        assert!(parse_mul("mul(123,456").is_err());
    }

    #[test]
    fn test_parse_do() {
        assert_eq!(parse_do("do()"), Ok(("", Instruction::Do)));
        assert!(parse_do("don't()").is_err());
    }

    #[test]
    fn test_parse_dont() {
        assert_eq!(parse_dont("don't()"), Ok(("", Instruction::Dont)));
        assert!(parse_dont("do()").is_err());
    }

    #[test]
    fn test_parse_instruction() {
        assert_eq!(
            parse_instruction("mul(123,456)"),
            Ok(("", Instruction::Mul(MulInstruction(123, 456))))
        );
        assert_eq!(parse_instruction("do()"), Ok(("", Instruction::Do)));
        assert_eq!(parse_instruction("don't()"), Ok(("", Instruction::Dont)));
    }

    #[test]
    fn test_parse_input_data() {
        assert_eq!(
            parse_input_data("mul(123,456)"),
            Ok(("", (vec![Instruction::Mul(MulInstruction(123, 456))])))
        );
        assert_eq!(
            parse_input_data("xmul(123,456)%&mul(123,456)("),
            Ok((
                "(",
                (vec![
                    Instruction::Mul(MulInstruction(123, 456)),
                    Instruction::Mul(MulInstruction(123, 456))
                ])
            ))
        );
        assert_eq!(
            parse_input_data("adon't()b"),
            Ok(("b", (vec![Instruction::Dont])))
        );
        assert_eq!(
            parse_input_data("adon't()bdo()b"),
            Ok(("b", (vec![Instruction::Dont, Instruction::Do])))
        );
        assert_eq!(
            parse_input_data("mul(123,456)adon't()bdo()b"),
            Ok((
                "b",
                (vec![
                    Instruction::Mul(MulInstruction(123, 456)),
                    Instruction::Dont,
                    Instruction::Do
                ])
            ))
        );
    }

    #[test]
    fn test_day_03_part_1() {
        assert_eq!(day_03_part_1("mul(2,4)"), 8);
        assert_eq!(day_03_part_1("xmul(2,4)"), 8);
        assert_eq!(day_03_part_1("xmul(2,4)%&mul(3,7)"), 29);
        assert_eq!(day_03_part_1(EXAMPLE_PART_1), 161);
    }

    #[test]
    fn test_day_03_part_2() {
        assert_eq!(day_03_part_2(EXAMPLE_PART_1), 161);
        assert_eq!(day_03_part_2(EXAMPLE_PART_2), 48);
    }
}
