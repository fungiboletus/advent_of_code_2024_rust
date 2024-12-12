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

fn parse_input_data(data: &str) -> IResult<&str, Array2<char>> {
    map(
        separated_list1(line_ending, many1(one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ"))),
        |rows| {
            let nb_rows = rows.len();
            let nb_cols = rows.first().map_or(0, |row| row.len());

            Array2::from_shape_fn((nb_rows, nb_cols), |(row, col)| rows[row][col])
        },
    )(data)
}

pub fn day_12_part_1(data: &str) -> i64 {
    let (_, grid) = parse_input_data(data).expect("Failed to parse input data");
    let nrows = grid.nrows();
    let ncols = grid.ncols();

    let mut visited = Array2::<bool>::from_elem(grid.dim(), false);
    let mut total_price: usize = 0;

    for ((row, col), &region) in grid.indexed_iter() {
        if visited[(row, col)] {
            continue;
        }

        let mut area: usize = 0;
        let mut borders: usize = 0;
        let mut dfs_pile: Vec<(usize, usize)> = vec![(row, col)];

        while let Some((row, col)) = dfs_pile.pop() {
            if visited[(row, col)] {
                continue;
            }

            visited[(row, col)] = true;

            area += 1;

            if row > 0 {
                if grid[(row - 1, col)] != region {
                    borders += 1;
                } else {
                    dfs_pile.push((row - 1, col));
                }
            } else {
                borders += 1;
            }
            if row < nrows - 1 {
                if grid[(row + 1, col)] != region {
                    borders += 1;
                } else {
                    dfs_pile.push((row + 1, col));
                }
            } else {
                borders += 1;
            }
            if col > 0 {
                if grid[(row, col - 1)] != region {
                    borders += 1;
                } else {
                    dfs_pile.push((row, col - 1));
                }
            } else {
                borders += 1;
            }
            if col < ncols - 1 {
                if grid[(row, col + 1)] != region {
                    borders += 1;
                } else {
                    dfs_pile.push((row, col + 1));
                }
            } else {
                borders += 1;
            }
        }

        total_price += area * borders;
    }

    total_price as i64
}

pub fn day_12_part_2(data: &str) -> i64 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_A: &str = "AAAA
BBCD
BBCC
EEEC";

    const EXAMPLE_B: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

    const EXAMPLE_C: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    const EXAMPLE_D: &str = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

    const EXAMPLE_E: &str = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

    #[test]
    fn test_day_12_part_1() {
        assert_eq!(day_12_part_1(EXAMPLE_A), 140);
        assert_eq!(day_12_part_1(EXAMPLE_B), 772);
        assert_eq!(day_12_part_1(EXAMPLE_C), 1930);
    }

    #[test]
    fn test_day_12_part_2() {
        assert_eq!(day_12_part_2(EXAMPLE_A), 80);
        assert_eq!(day_12_part_2(EXAMPLE_B), 436);
        assert_eq!(day_12_part_2(EXAMPLE_C), 1206);
        assert_eq!(day_12_part_2(EXAMPLE_D), 236);
        assert_eq!(day_12_part_2(EXAMPLE_E), 368);
    }
}
