/*
    Part 1 is about modulo.
*/

use nom::{
    bytes::complete::tag, character::complete::line_ending, combinator::map,
    multi::separated_list1, sequence::tuple, IResult,
};

#[derive(Debug)]
struct Velocity {
    col: i64,
    row: i64,
}

#[derive(Debug)]
struct Position {
    col: i64,
    row: i64,
}

#[derive(Debug)]
struct Robot {
    position: Position,
    velocity: Velocity,
}

fn parse_position(input: &str) -> IResult<&str, Position> {
    map(
        tuple((
            tag("p="),
            nom::character::complete::i64,
            tag(","),
            nom::character::complete::i64,
        )),
        |(_, col, _, row)| Position { col, row },
    )(input)
}

fn parse_velocity(input: &str) -> IResult<&str, Velocity> {
    map(
        tuple((
            tag("v="),
            nom::character::complete::i64,
            tag(","),
            nom::character::complete::i64,
        )),
        |(_, col, _, row)| Velocity { col, row },
    )(input)
}

fn parse_robot(input: &str) -> IResult<&str, Robot> {
    map(
        tuple((parse_position, tag(" "), parse_velocity)),
        |(position, _, velocity)| Robot { position, velocity },
    )(input)
}

fn parse_input_data(data: &str) -> IResult<&str, Vec<Robot>> {
    separated_list1(line_ending, parse_robot)(data)
}

pub fn day_14_part_1(data: &str) -> i64 {
    let (_, data) = parse_input_data(data).expect("Failed to parse input data");

    // Different area size between the example and the actual input
    let (wide, tall) = if data.len() > 100 {
        (101, 103)
    } else {
        (11, 7)
    };

    let time_span = 100;
    let col_split = wide / 2;
    let row_split = tall / 2;

    let mut counter_top_left = 0;
    let mut counter_top_right = 0;
    let mut counter_bottom_left = 0;
    let mut counter_bottom_right = 0;

    for Position { col, row } in data.iter().map(|robot| {
        let col = (((robot.position.col + robot.velocity.col * time_span) % wide) + wide) % wide;
        let row = (((robot.position.row + robot.velocity.row * time_span) % tall) + tall) % tall;
        Position { col, row }
    }) {
        #[allow(clippy::comparison_chain)]
        if col < col_split {
            if row < row_split {
                counter_top_left += 1;
            } else if row > row_split {
                counter_bottom_left += 1;
            }
        } else if col > col_split {
            if row < row_split {
                counter_top_right += 1;
            } else if row > row_split {
                counter_bottom_right += 1;
            }
        }
    }

    counter_top_left * counter_top_right * counter_bottom_left * counter_bottom_right
}

pub fn day_14_part_2(data: &str) -> i64 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_day_14_part_1() {
        assert_eq!(day_14_part_1(EXAMPLE), 12);
    }

    #[test]
    fn test_day_14_part_2() {
        assert_eq!(day_14_part_2(EXAMPLE), 42);
    }
}
