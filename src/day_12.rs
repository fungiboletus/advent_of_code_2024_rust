/*
    Enjoyed part 1, with a nice DFS like the previous day.

    Part 2 has been a annoying to me because I thought it was
    going to be a nice trick to count the straights but it wasn't.

    I didn't do it for days and consider skipping it because I didn't
    enjoy the problem. It did feel like a chore.

    But someone on r/adventofcode mentioned that you could do it with
    a Set and checking the existence of another similar border next
    to the current one when counting.

    That did sound more fun than most other suggested solutions, so
    I did that, and it wasn't too bad after all.
*/

use std::collections::HashSet;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Side {
    Top,
    Right,
    Bottom,
    Left,
}

fn day_12(data: &str, part_two: bool) -> i64 {
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
        //let mut borders: usize = 0;
        let mut borders_set: HashSet<(usize, usize, Side)> = HashSet::new();
        let mut dfs_pile: Vec<(usize, usize)> = vec![(row, col)];

        while let Some((row, col)) = dfs_pile.pop() {
            if visited[(row, col)] {
                continue;
            }

            visited[(row, col)] = true;

            area += 1;

            if row > 0 {
                if grid[(row - 1, col)] != region {
                    //borders += 1;
                    borders_set.insert((row, col, Side::Top));
                } else {
                    dfs_pile.push((row - 1, col));
                }
            } else {
                //borders += 1;
                borders_set.insert((row, col, Side::Top));
            }
            if row < nrows - 1 {
                if grid[(row + 1, col)] != region {
                    //borders += 1;
                    borders_set.insert((row, col, Side::Bottom));
                } else {
                    dfs_pile.push((row + 1, col));
                }
            } else {
                //borders += 1;
                borders_set.insert((row, col, Side::Bottom));
            }
            if col > 0 {
                if grid[(row, col - 1)] != region {
                    //borders += 1;
                    borders_set.insert((row, col, Side::Left));
                } else {
                    dfs_pile.push((row, col - 1));
                }
            } else {
                //borders += 1;
                borders_set.insert((row, col, Side::Left));
            }
            if col < ncols - 1 {
                if grid[(row, col + 1)] != region {
                    //borders += 1;
                    borders_set.insert((row, col, Side::Right));
                } else {
                    dfs_pile.push((row, col + 1));
                }
            } else {
                //borders += 1;
                borders_set.insert((row, col, Side::Right));
            }
        }

        //assert_eq!(borders_set.len(), borders);
        let cost_model = if part_two {
            borders_set
                .iter()
                .filter(|(row, col, side)| {
                    let border_to_check = match side {
                        Side::Top | Side::Bottom => {
                            if *col == 0 {
                                return true;
                            }
                            (*row, col - 1, *side)
                        }
                        Side::Left | Side::Right => {
                            if *row == 0 {
                                return true;
                            }
                            (row - 1, *col, *side)
                        }
                    };

                    !borders_set.contains(&border_to_check)
                })
                .count()
        } else {
            borders_set.len()
        };
        total_price += area * cost_model;
    }

    total_price as i64
}

pub fn day_12_part_1(data: &str) -> i64 {
    day_12(data, false)
}

pub fn day_12_part_2(data: &str) -> i64 {
    day_12(data, true)
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
