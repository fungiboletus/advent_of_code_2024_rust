/*
    Part 1 is a classic A* algorithm.

    To track the visited position from the direction, I made a 3D array
    with the third dimension being the direction.

    The BinaryHeap returns the greatest element, so I used the Reverse
    wrapper to make it return the smallest element. It's a bit verbose.

    The manhattan distance heuristic seems to be completely useless
    on this problem, but let's keep it for good measure.
*/

use std::{cmp::Reverse, collections::BinaryHeap};

use ndarray::{Array2, Array3};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Wall,
    Start,
    Exit,
}

fn parse_cell(input: &str) -> IResult<&str, Cell> {
    alt((
        map(tag("#"), |_| Cell::Wall),
        map(tag("S"), |_| Cell::Start),
        map(tag("E"), |_| Cell::Exit),
        map(tag("."), |_| Cell::Empty),
    ))(input)
}

fn parse_input_data(input: &str) -> IResult<&str, Array2<Cell>> {
    map(separated_list1(line_ending, many1(parse_cell)), |grid| {
        let nb_rows = grid.len();
        let nb_cols = grid.first().map_or(0, |row| row.len());

        Array2::from_shape_fn((nb_rows, nb_cols), |(row, col)| grid[row][col])
    })(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[inline]
fn manhattan_distance(a: (usize, usize), b: (usize, usize)) -> usize {
    ((a.0 as i64 - b.0 as i64).unsigned_abs() + (a.1 as i64 - b.1 as i64).unsigned_abs()) as usize
}

/** Solved with a very classic A* algorithm. */
pub fn day_16_part_1(data: &str) -> i64 {
    let (_, map) = parse_input_data(data).expect("Failed to parse input data");

    #[allow(clippy::type_complexity)]
    let mut priority_queue: BinaryHeap<Reverse<(usize, usize, usize, usize, Direction)>> =
        BinaryHeap::new();

    let start = map
        .indexed_iter()
        .find_map(|((row, col), &cell)| {
            if cell == Cell::Start {
                Some((row, col))
            } else {
                None
            }
        })
        .expect("Failed to find start cell");

    let exit = map
        .indexed_iter()
        .find_map(|((row, col), &cell)| {
            if cell == Cell::Exit {
                Some((row, col))
            } else {
                None
            }
        })
        .expect("Failed to find exit cell");

    let initial_cost = manhattan_distance(start, exit);
    priority_queue.push(Reverse((
        initial_cost,
        1000,
        start.0,
        start.1,
        Direction::Up,
    )));
    priority_queue.push(Reverse((
        initial_cost,
        1000,
        start.0,
        start.1,
        Direction::Down,
    )));
    priority_queue.push(Reverse((
        initial_cost,
        1000,
        start.0,
        start.1,
        Direction::Left,
    )));
    priority_queue.push(Reverse((
        initial_cost,
        0,
        start.0,
        start.1,
        Direction::Right,
    )));

    let nrows = map.nrows();
    let ncols = map.ncols();
    let mut visited: Array3<Option<usize>> = Array3::from_elem((nrows, ncols, 4), None);

    while let Some(Reverse((_f_score, g_score, row, col, current_direction))) = priority_queue.pop()
    {
        /*println!(
            "f_score: {}, g_score: {}, row: {}, col: {}, direction: {:?}",
            _f_score, g_score, row, col, current_direction
        );*/
        // if we reach the exit, we are done
        if (row, col) == exit {
            return g_score as i64;
        }

        // if already visited with a lower cost
        if let Some(visited_g_score) = visited[[row, col, current_direction as usize]] {
            if visited_g_score <= g_score {
                continue;
            }
        }
        visited[[row, col, current_direction as usize]] = Some(g_score);

        // check neighbors
        for (direction, (drow, dcol)) in [
            (Direction::Up, (-1, 0)),
            (Direction::Down, (1, 0)),
            (Direction::Left, (0, -1)),
            (Direction::Right, (0, 1)),
        ] {
            let new_row = (row as i64 + drow) as usize;
            let new_col = (col as i64 + dcol) as usize;

            if new_row < nrows && new_col < ncols && map[[new_row, new_col]] != Cell::Wall {
                let new_g_score = if current_direction != direction {
                    g_score + 1001
                } else {
                    g_score + 1
                };

                // the heuristic is the Manhattan distance
                let new_f_score = new_g_score + manhattan_distance((new_row, new_col), exit);

                priority_queue.push(Reverse((
                    new_f_score,
                    new_g_score,
                    new_row,
                    new_col,
                    direction,
                )));
            }
        }
    }

    // If not reachable, return -1
    -1
}

pub fn day_16_part_2(data: &str) -> i64 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_A: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const EXAMPLE_B: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn test_day_16_part_1() {
        assert_eq!(day_16_part_1(EXAMPLE_A), 7036);
        assert_eq!(day_16_part_1(EXAMPLE_B), 11048);
    }

    #[test]
    fn test_day_16_part_2() {
        assert_eq!(day_16_part_2(EXAMPLE_A), 42);
    }
}
