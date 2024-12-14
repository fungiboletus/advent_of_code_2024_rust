/*
    Part 1 is solved using maths. It's only about solving
    a simple system of linear equations.

    Part 2 was very easy for me today, it was just about
    making the non maths solutions too slow.
*/

use nom::{
    bytes::complete::tag, character::complete::line_ending, combinator::map,
    multi::separated_list1, sequence::tuple, IResult,
};

#[derive(Debug, Clone)]
struct Button {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Prize {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct PrizeProblem {
    button_a: Button,
    button_b: Button,
    prize: Prize,
}

impl PrizeProblem {
    fn clicks_per_button(&self) -> Option<(i64, i64)> {
        // I obviously didn't come up with linear solver algorithm
        // on my own in a morning.
        let x = self.prize.x;
        let y = self.prize.y;

        let a_x = self.button_a.x;
        let a_y = self.button_a.y;
        let b_x = self.button_b.x;
        let b_y = self.button_b.y;

        let a1 = a_x * b_y;
        //let b1 = b_x * b_y;
        let c1 = x * b_y;

        let a2 = a_y * b_x;
        //let b2 = b_y * b_x;
        let c2 = y * b_x;

        let a_final = a1 - a2;
        //let b_final = b1 - b2;
        //println!("{:?}", b_final);
        let c_final = c1 - c2;

        let clicks_a = c_final / a_final;
        let clicks_b = (x - a_x * clicks_a) / b_x;

        // check that we have the correct solution, since we
        // work with integers
        if a_x * clicks_a + b_x * clicks_b != x || a_y * clicks_a + b_y * clicks_b != y {
            None
        } else {
            Some((clicks_a, clicks_b))
        }
    }

    fn to_part_two(&self) -> Self {
        PrizeProblem {
            button_a: self.button_a.clone(),
            button_b: self.button_b.clone(),
            prize: Prize {
                x: self.prize.x + 10000000000000,
                y: self.prize.y + 10000000000000,
            },
        }
    }
}

fn parse_button_position(input: &str) -> IResult<&str, Button> {
    map(
        tuple((
            tag(" X+"),
            nom::character::complete::i64,
            tag(", Y+"),
            nom::character::complete::i64,
        )),
        |(_, x, _, y)| Button { x, y },
    )(input)
}

fn parse_prize_position(input: &str) -> IResult<&str, Prize> {
    map(
        tuple((
            tag(" X="),
            nom::character::complete::i64,
            tag(", Y="),
            nom::character::complete::i64,
        )),
        |(_, x, _, y)| Prize { x, y },
    )(input)
}

fn parse_prize_problem(input: &str) -> IResult<&str, PrizeProblem> {
    map(
        tuple((
            tag("Button A:"),
            parse_button_position,
            line_ending,
            tag("Button B:"),
            parse_button_position,
            line_ending,
            tag("Prize:"),
            parse_prize_position,
        )),
        |(_, button_a, _, _, button_b, _, _, prize)| PrizeProblem {
            button_a,
            button_b,
            prize,
        },
    )(input)
}

fn parse_input_data(data: &str) -> IResult<&str, Vec<PrizeProblem>> {
    separated_list1(tuple((line_ending, line_ending)), parse_prize_problem)(data)
}

pub fn day_13_part_1(data: &str) -> i64 {
    let (_, data) = parse_input_data(data).expect("Failed to parse input data");

    data.iter()
        .filter_map(|problem| problem.clicks_per_button())
        .map(|(clicks_a, clicks_b)| 3 * clicks_a + clicks_b)
        .sum()
}

pub fn day_13_part_2(data: &str) -> i64 {
    let (_, data) = parse_input_data(data).expect("Failed to parse input data");

    data.iter()
        .map(|problem| problem.to_part_two())
        .filter_map(|problem| problem.clicks_per_button())
        .map(|(clicks_a, clicks_b)| 3 * clicks_a + clicks_b)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_day_13_part_1() {
        assert_eq!(day_13_part_1(EXAMPLE), 480);
    }

    #[test]
    fn test_day_13_part_2() {
        assert_eq!(day_13_part_2(EXAMPLE), 875318608908);
    }
}
