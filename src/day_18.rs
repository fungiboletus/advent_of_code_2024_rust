/*
    Part 1 is a straightforward A* pathfinding problem.
    Simpler version than day 16 part 1, worked first try.
*/

use std::{cmp::Reverse, collections::BinaryHeap};

use ndarray::Array2;
use nom::{
    bytes::complete::tag, character::complete::line_ending, multi::separated_list1,
    sequence::separated_pair, IResult,
};

fn parse_input_data(data: &str) -> IResult<&str, Vec<(u64, u64)>> {
    separated_list1(
        line_ending,
        separated_pair(
            nom::character::complete::u64,
            tag(","),
            nom::character::complete::u64,
        ),
    )(data)
}

#[inline]
fn manhattan_distance(a: (usize, usize), b: (usize, usize)) -> usize {
    ((a.0 as i64 - b.0 as i64).unsigned_abs() + (a.1 as i64 - b.1 as i64).unsigned_abs()) as usize
}

pub fn day_18_part_1(data: &str) -> i64 {
    let (_, data) = parse_input_data(data).expect("Failed to parse input data");

    // The example works on a smaller grid than the input data
    let grid_size = if data.len() >= 1024 { (71, 71) } else { (7, 7) };
    let max_count = if data.len() >= 1024 { 1024 } else { 12 };

    let nrows = grid_size.0;
    let ncols = grid_size.1;

    let start = (0, 0);
    let exit = (nrows - 1, ncols - 1);

    let mut grid = Array2::<bool>::from_elem(grid_size, false);
    for &(x, y) in data[..max_count].iter() {
        grid[[x as usize, y as usize]] = true;
    }

    // Let's go with A*
    // cost (f_score), distance (g_score), (row, col)
    #[allow(clippy::type_complexity)]
    let mut priority_queue: BinaryHeap<Reverse<(usize, usize, (usize, usize))>> = BinaryHeap::new();
    let mut visited = Array2::<Option<usize>>::from_elem(grid_size, None);

    let initial_cost = manhattan_distance(start, exit);
    priority_queue.push(Reverse((initial_cost, 0, start)));

    while let Some(Reverse((_f_score, g_score, (row, col)))) = priority_queue.pop() {
        if (row, col) == exit {
            return g_score as i64;
        }

        if let Some(visited_g_score) = visited[[row, col]] {
            if visited_g_score <= g_score {
                continue;
            }
        }
        visited[[row, col]] = Some(g_score);

        // visit neighbors
        for (drow, dcol) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let new_row = row as i64 + drow;
            let new_col = col as i64 + dcol;

            if new_row < 0 || new_row >= nrows as i64 || new_col < 0 || new_col >= ncols as i64 {
                continue;
            }

            let new_row = new_row as usize;
            let new_col = new_col as usize;

            if grid[[new_row, new_col]] {
                continue;
            }

            let new_g_score = g_score + 1;
            let new_f_score = new_g_score + manhattan_distance((new_row, new_col), exit);

            priority_queue.push(Reverse((new_f_score, new_g_score, (new_row, new_col))));
        }
    }

    -1
}

pub fn day_18_part_2(data: &str) -> i64 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn test_day_18_part_1() {
        assert_eq!(day_18_part_1(EXAMPLE), 22);
    }

    #[test]
    fn test_day_18_part_2() {
        assert_eq!(day_18_part_2(EXAMPLE), 42);
    }
}
