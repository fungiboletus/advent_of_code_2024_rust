/*
    Part 1 is a straightforward A* pathfinding problem.
    Simpler version than day 16 part 1, worked first try.

    Part 2 is a binary search using part 1. Perhaps not the
    most efficient way to solve it, but it works.
*/

use std::{cmp::Reverse, collections::BinaryHeap};

use ndarray::prelude::*;
use ndarray::{Array2, Array3};
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

// Returns the number of steps to reach the exit, or None if there is no path
fn has_path(
    grid: &ArrayView2<bool>,
    start: (usize, usize),
    exit: (usize, usize),
    visited: &mut Array2<Option<usize>>,
) -> Option<usize> {
    let grid_size = grid.dim();
    let nrows = grid_size.0;
    let ncols = grid_size.1;

    // Let's go with A*
    // cost (f_score), distance (g_score), (row, col)
    #[allow(clippy::type_complexity)]
    let mut priority_queue: BinaryHeap<Reverse<(usize, usize, (usize, usize))>> = BinaryHeap::new();
    //let mut visited = Array2::<Option<usize>>::from_elem(grid_size, None);

    let initial_cost = manhattan_distance(start, exit);
    priority_queue.push(Reverse((initial_cost, 0, start)));

    while let Some(Reverse((_f_score, g_score, (row, col)))) = priority_queue.pop() {
        if (row, col) == exit {
            return Some(g_score);
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

    None
}

pub fn day_18_part_1(data: &str) -> usize {
    let (_, data) = parse_input_data(data).expect("Failed to parse input data");

    // The example works on a smaller grid than the input data
    let grid_size = if data.len() >= 1024 { (71, 71) } else { (7, 7) };
    let max_count = if data.len() >= 1024 { 1024 } else { 12 };

    let nrows = grid_size.0;
    let ncols = grid_size.1;

    let start = (0, 0);
    let exit = (nrows - 1, ncols - 1);

    let mut grid = Array2::<bool>::from_elem(grid_size, false);
    for &(row, col) in data[..max_count].iter() {
        grid[[row as usize, col as usize]] = true;
    }
    let mut visited = Array2::<Option<usize>>::from_elem(grid_size, None);
    has_path(&grid.view(), start, exit, &mut visited).expect("No path found")
}

pub fn day_18_part_2(data: &str) -> String {
    let (_, data) = parse_input_data(data).expect("Failed to parse input data");
    let ntimes = data.len();
    let grid_size = if ntimes >= 1024 {
        (71, 71, ntimes)
    } else {
        (7, 7, ntimes)
    };

    let nrows = grid_size.0;
    let ncols = grid_size.1;

    let start = (0, 0);
    let exit = (nrows - 1, ncols - 1);

    // Instead of a grid, we use a 3D cube and the third dimension is time
    let mut cube = Array3::<bool>::from_elem(grid_size, false);
    for (time, &(row, col)) in data.iter().enumerate() {
        /*for t in time..ntimes {
            cube[[row as usize, col as usize, t]] = true;
        }*/
        cube.slice_mut(s![row as usize, col as usize, time..])
            .fill(true);
    }

    // We then do a binary search to find the first time where there is no path.

    // We know that the part 1 solution is a valid path, so we don't
    // need to check below those.
    let mut time_low = if ntimes >= 1024 { 1024 } else { 12 };
    // We assume that by the end of the input data, there is no path
    let mut time_high = ntimes;

    let mut visited = Array2::<Option<usize>>::from_elem((nrows, ncols), None);

    while time_low < time_high {
        let time_pivot = time_low + (time_high - time_low) / 2;
        let grid: ArrayView2<bool> = cube.index_axis(Axis(2), time_pivot);
        visited.fill(None);
        if has_path(&grid, start, exit, &mut visited).is_some() {
            time_low = time_pivot + 1;
        } else {
            time_high = time_pivot;
        }
    }

    let (row, col) = data[time_low];
    format!("{},{}", row, col)
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
        assert_eq!(day_18_part_2(EXAMPLE), "6,1");
    }
}
