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

fn ascii_to_height(ascii: char) -> Option<u8> {
    match ascii {
        '0'..='9' => Some(ascii as u8 - 48),
        '.' => None,
        _ => panic!("Invalid character"),
    }
}

fn parse_input_data(data: &str) -> IResult<&str, Array2<Option<u8>>> {
    map(
        separated_list1(
            line_ending,
            many1(map(one_of("0123456789."), ascii_to_height)),
        ),
        |rows| {
            let nb_rows = rows.len();
            let nb_cols = rows.first().map_or(0, |row| row.len());

            Array2::from_shape_fn((nb_rows, nb_cols), |(row, col)| rows[row][col])
        },
    )(data)
}

pub fn day_10_part_1(data: &str) -> i64 {
    let (_, grid) = parse_input_data(data).expect("Failed to parse input data");
    let nrows = grid.nrows();
    let ncols = grid.ncols();

    let mut reachable_tops_counters = Array2::<usize>::zeros(grid.dim());

    // Start from the tops
    for top in grid
        .indexed_iter()
        .filter(|(_, height)| **height == Some(9))
    {
        let mut dfs_pile: Vec<((usize, usize), u8)> = vec![(top.0, 9)];
        let mut visited = Array2::<bool>::from_elem(grid.dim(), false);

        while let Some(((row, col), height)) = dfs_pile.pop() {
            if visited[(row, col)] {
                continue;
            }
            reachable_tops_counters[(row, col)] += 1;
            visited[(row, col)] = true;
            if height == 0 {
                continue;
            }
            let target_height = height - 1;
            if row > 0 && grid[(row - 1, col)] == Some(target_height) {
                dfs_pile.push(((row - 1, col), target_height));
            }
            if row < nrows - 1 && grid[(row + 1, col)] == Some(target_height) {
                dfs_pile.push(((row + 1, col), target_height));
            }
            if col > 0 && grid[(row, col - 1)] == Some(target_height) {
                dfs_pile.push(((row, col - 1), target_height));
            }
            if col < ncols - 1 && grid[(row, col + 1)] == Some(target_height) {
                dfs_pile.push(((row, col + 1), target_height));
            }
        }
    }

    grid.indexed_iter()
        .filter(|(_, height)| **height == Some(0))
        .map(|(pos, _)| reachable_tops_counters[pos])
        .sum::<usize>() as i64
}

pub fn day_10_part_2(data: &str) -> i64 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_A: &str = "0123
1234
8765
9876";
    const EXAMPLE_B: &str = "...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9";
    const EXAMPLE_C: &str = "..90..9
...1.98
...2..7
6543456
765.987
876....
987....";
    const EXAMPLE_D: &str = "10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01";
    const EXAMPLE_E: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_day_10_part_1() {
        assert_eq!(day_10_part_1(EXAMPLE_A), 1);
        assert_eq!(day_10_part_1(EXAMPLE_B), 2);
        assert_eq!(day_10_part_1(EXAMPLE_C), 4);
        assert_eq!(day_10_part_1(EXAMPLE_D), 3);
        assert_eq!(day_10_part_1(EXAMPLE_E), 36);
    }

    #[test]
    fn test_day_10_part_2() {
        assert_eq!(day_10_part_2(EXAMPLE_A), 42);
    }
}
