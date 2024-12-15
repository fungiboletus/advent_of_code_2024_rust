/*
    Part 1 is about modulo.

    Part 2 is about finding the first time when the robots
    display a christmas tree.

    I had a guess that filtering out the times where robots
    are overlapping would help a lot, and perhaps a visual
    filtering would be necessary. But it seems like the first
    day where there is no overlap is the answer. So an easy
    day.

    The part 2 solution could be optimised instead of iterating
    over time, I guess, but it's relatively fast.

    A day that didn't require fishing for clues.

    Thinking back about it, my solution works 96% of the time,
    and only because the initial positions used to build the problem
    didn't have overlapping robots.

    96% because of the birthday paradoxe.

    Probability of at least 2 overlapping robots in a group of
    500 robots over 101*103 positions is 196 497 / 196 498
    (birthday paradoxe computed with Wolfram Alpha).

    Then having this probability about 7800 times in a row is
    (196 497 / 196 498)^7800 ~= 0.96

    The probability to have it between 1 and 10000 times in a row
    is about 97%. Good enough.

*/

use ndarray::Array2;
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

fn move_robot(robot: &Robot, time_span: i64, wide: i64, tall: i64) -> Position {
    let col = (((robot.position.col + robot.velocity.col * time_span) % wide) + wide) % wide;
    let row = (((robot.position.row + robot.velocity.row * time_span) % tall) + tall) % tall;
    Position { col, row }
}

impl Robot {
    fn update_position(&mut self, time_span: i64, wide: i64, tall: i64) {
        let Position { col, row } = move_robot(self, time_span, wide, tall);
        self.position.col = col;
        self.position.row = row;
    }
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

    for Position { col, row } in data
        .iter()
        .map(|robot| move_robot(robot, time_span, wide, tall))
    {
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
    let (_, mut data) = parse_input_data(data).expect("Failed to parse input data");
    let wide = 101;
    let tall = 103;

    // assumption, no overlapping robots
    let mut time = 0;
    let mut positions = Array2::<bool>::from_elem((wide as usize, tall as usize), false);
    loop {
        time += 1;
        for robot in data.iter_mut() {
            robot.update_position(1, wide, tall);
        }

        positions.fill(false);
        let mut found_overlap = false;
        for robot in data.iter() {
            let position = (robot.position.col as usize, robot.position.row as usize);
            if positions[position] {
                found_overlap = true;
                break;
            } else {
                positions[position] = true;
            }
        }
        if !found_overlap {
            break;
        }
    }
    time
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
        assert_eq!(day_14_part_2(EXAMPLE), 1);
    }
}
