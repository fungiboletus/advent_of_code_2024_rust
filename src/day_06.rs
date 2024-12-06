/*
    Comments.
*/

use ndarray::Array2;
use nom::{
    character::complete::{line_ending, one_of},
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};

#[derive(Debug, PartialEq)]
enum Space {
    Empty,
    Obstructed,
    Start,

    Visited,
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn rotate(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn facing_position(
        &self,
        position: (usize, usize),
        map_size: (usize, usize),
    ) -> Option<(usize, usize)> {
        let (row, col) = position;
        let (nb_rows, nb_cols) = map_size;
        match (self, row, col) {
            (Direction::Up, 0, _) => None,
            (Direction::Up, _, _) => Some((row - 1, col)),
            (Direction::Right, _, _) if col == nb_cols - 1 => None,
            (Direction::Right, _, _) => Some((row, col + 1)),
            (Direction::Down, _, _) if row == nb_rows - 1 => None,
            (Direction::Down, _, _) => Some((row + 1, col)),
            (Direction::Left, _, 0) => None,
            (Direction::Left, _, _) => Some((row, col - 1)),
        }
    }
}

type Map = Array2<Space>;

#[allow(dead_code)]
trait MapExt {
    fn print(&self);
}

impl MapExt for Map {
    fn print(&self) {
        for row in self.rows() {
            for space in row {
                print!(
                    "{}",
                    match space {
                        Space::Empty => '.',
                        Space::Obstructed => '#',
                        Space::Start => '^',
                        Space::Visited => 'X',
                    }
                );
            }
            println!();
        }
    }
}

fn parse_input_data(data: &str) -> IResult<&str, Map> {
    map(separated_list1(line_ending, many1(one_of(".#^"))), |rows| {
        let nb_rows = rows.len();
        let nb_cols = rows.first().map_or(0, |row| row.len());

        Array2::from_shape_fn((nb_rows, nb_cols), |(row, col)| match rows[row][col] {
            '.' => Space::Empty,
            '#' => Space::Obstructed,
            '^' => Space::Start,
            _ => unreachable!(),
        })
    })(data)
}

pub fn day_06_part_1(data: &str) -> i64 {
    let (_, map) = parse_input_data(data).expect("Failed to parse input data");

    let mut map = map;
    let shape = map.shape();
    let shape = (shape[0], shape[1]);

    // find the start position
    let (start_position, _) = map
        .indexed_iter()
        .find(|(_, m)| **m == Space::Start)
        .expect("No start position found");

    let mut position = start_position;
    let mut direction = Direction::Up;

    map[position] = Space::Visited;

    while let Some(next_position) = direction.facing_position(position, shape) {
        if map[next_position] == Space::Obstructed {
            direction = direction.rotate();
        } else {
            position = next_position;
            map[position] = Space::Visited;
        }
    }

    // map.print();
    map.iter().filter(|space| **space == Space::Visited).count() as i64
}

pub fn day_06_part_2(data: &str) -> i64 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_day_06_part_1() {
        assert_eq!(day_06_part_1(EXAMPLE), 41);
    }

    #[test]
    fn test_day_06_part_2() {
        assert_eq!(day_06_part_2(EXAMPLE), 42);
    }
}
