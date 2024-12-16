/*
    Part 1 is a classic A* algorithm.

    To track the visited position from the direction, I made a 3D array
    with the third dimension being the direction.

    The BinaryHeap returns the greatest element, so I used the Reverse
    wrapper to make it return the smallest element. It's a bit verbose.

    The manhattan distance heuristic seems to be completely useless
    on this problem, but let's keep it for good measure.

    Part 2 is a bit more *cumbersome*. I ditch the A* algorithm to have a
    simpler algorithm since we don't really care about finding the best path
    quickly, we want to find all the best paths.
    Then I implemented a nasty backtracking algorithm to find all the paths
    that lead to the exit, and count the number of visited cells.
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

fn find_start_and_exit(map: &Array2<Cell>) -> ((usize, usize), (usize, usize)) {
    let mut start: Option<(usize, usize)> = None;
    let mut exit: Option<(usize, usize)> = None;

    for (position, &cell) in map.indexed_iter() {
        if cell == Cell::Start {
            start = Some(position);
        } else if cell == Cell::Exit {
            exit = Some(position);
        }
    }

    (
        start.expect("Failed to find start cell"),
        exit.expect("Failed to find exit cell"),
    )
}

/** Solved with a very classic A* algorithm. */
pub fn day_16_part_1(data: &str) -> i64 {
    let (_, map) = parse_input_data(data).expect("Failed to parse input data");
    let (start, exit) = find_start_and_exit(&map);

    #[allow(clippy::type_complexity)]
    let mut priority_queue: BinaryHeap<Reverse<(usize, usize, usize, usize, Direction)>> =
        BinaryHeap::new();

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
    let (_, map) = parse_input_data(data).expect("Failed to parse input data");

    let (start, exit) = find_start_and_exit(&map);

    let mut priority_queue: BinaryHeap<Reverse<(usize, usize, usize, Direction)>> =
        BinaryHeap::new();

    priority_queue.push(Reverse((1000, start.0, start.1, Direction::Up)));
    priority_queue.push(Reverse((1000, start.0, start.1, Direction::Down)));
    priority_queue.push(Reverse((1000, start.0, start.1, Direction::Left)));
    priority_queue.push(Reverse((0, start.0, start.1, Direction::Right)));

    let nrows = map.nrows();
    let ncols = map.ncols();
    let mut visited: Array3<Option<usize>> = Array3::from_elem((nrows, ncols, 4), None);

    let mut found_exit_cost: Option<usize> = None;

    while let Some(Reverse((g_score, row, col, current_direction))) = priority_queue.pop() {
        /*println!(
            "g_score: {}, row: {}, col: {}, direction: {:?}",
            g_score, row, col, current_direction
        );*/
        // Skip all the tentatives that are more expensive than the best path found
        // In practice this doesn't seem to be useful on the problems we have.
        if let Some(cost) = found_exit_cost {
            if g_score > cost {
                break;
            }
        }

        // if already visited with a lower cost
        if let Some(visited_g_score) = visited[[row, col, current_direction as usize]] {
            if visited_g_score <= g_score {
                continue;
            }
        }
        visited[[row, col, current_direction as usize]] = Some(g_score);

        // if we reach the exit, we are done
        if (row, col) == exit {
            found_exit_cost = Some(g_score);
            // Let's keep going to see if another path from another direction
            // is just as good. This doesn't seem to be the case on the problems we have.
            continue;
        }

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

                priority_queue.push(Reverse((new_g_score, new_row, new_col, direction)));
            }
        }
    }

    let final_score = found_exit_cost.expect("Failed to find exit");

    let mut pile: Vec<(usize, usize, Direction, usize)> = Vec::new();

    let (exit_row, exit_col) = exit;

    // I could have used a fancy slice with iters and so on, but I'm lazy
    if visited[[exit_row, exit_col, 0]] == found_exit_cost {
        pile.push((exit_row, exit_col, Direction::Up, final_score));
    }
    if visited[[exit_row, exit_col, 1]] == found_exit_cost {
        pile.push((exit_row, exit_col, Direction::Down, final_score));
    }
    if visited[[exit_row, exit_col, 2]] == found_exit_cost {
        pile.push((exit_row, exit_col, Direction::Left, final_score));
    }
    if visited[[exit_row, exit_col, 3]] == found_exit_cost {
        pile.push((exit_row, exit_col, Direction::Right, final_score));
    }

    let mut visited_again: Array2<bool> = Array2::from_elem((nrows, ncols), false);

    while let Some((row, col, current_direction, current_score)) = pile.pop() {
        /*println!(
            "row: {}, col: {}, direction: {:?}, score: {}",
            row, col, current_direction, current_score
        );*/
        visited_again[[row, col]] = true;
        if (row, col) == start {
            //println!("found start at {:?}", (row, col));
            continue;
        }

        for direction in [
            Direction::Down,
            Direction::Up,
            Direction::Left,
            Direction::Right,
        ] {
            for (drow, dcol) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let new_row = (row as i64 + drow) as usize;
                let new_col = (col as i64 + dcol) as usize;

                let expected_score = if direction != current_direction {
                    current_score - 1001
                } else {
                    current_score - 1
                };

                /*println!(
                    "candidate: {:?}, expected_score: {}, actual_score: {:?}",
                    (new_row, new_col),
                    expected_score,
                    visited[[new_row, new_col, direction as usize]]
                );*/

                if visited[[new_row, new_col, direction as usize]] == Some(expected_score) {
                    pile.push((new_row, new_col, direction, expected_score));
                }
            }
        }
    }

    visited_again.iter().filter(|&&visited| visited).count() as i64
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
        assert_eq!(day_16_part_2(EXAMPLE_A), 45);
        assert_eq!(day_16_part_2(EXAMPLE_B), 64);
    }
}
