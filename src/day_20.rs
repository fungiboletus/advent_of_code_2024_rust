/*
    I did implement part 1 with a A* algorithm and a fancy
    system to break walls and find the best shortcuts.

    I did run in about 1.5s, which was too slow for a part 1.

    I read a bit more about the problem and found that it wasn't
    a maze problem, as there is only a single path from start to
    exit.

    So I reimplemented the solution with a faster algorithm
    that runs very quickly.

    The idea is to compute the accumulative path length, and
    then check every wall if I can find a shortcut by crossing
    the wall from top to bottom, bottom to top, left to right
    or right to left.

    Part 2 was a lot more annoying and I struggled a lot with
    quite a few one off errors. Especially because the example
    wasn't the same grid size than the actual input data.

    I had two main issues:
        - I forgot that the start of a shortcut
    can be below or on the right of the end of the shortcut.
        - I was visiting too far as I forgot that going
        -20 to +20 on both axis did give a manhattan distance
        above 20, which is kinda an obvious statement in
        the problem description.

    Overall, part 2 wasn't that hard, but I really struggle
    working with indices and offsets late at night.

    I optimised slightly after a night of sleep, it could be
    much more optimised by using a better datastructure to
    prevent checking so many cells in the 2D matrix, but
    this is good enough for now and I'm done with this puzzle.
*/
use ndarray::{Array2, ArrayView2};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

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

#[allow(clippy::type_complexity)]
fn compute_path_lengths(
    map: &ArrayView2<Cell>,
    start: (usize, usize),
    end: (usize, usize),
) -> (Array2<Option<usize>>, Vec<((usize, usize), usize)>) {
    let map_size = map.dim();
    let (nrows, ncols) = map_size;

    let mut current = start;
    let mut previous = start;
    let mut current_length = 0;

    let mut path_lengths = Array2::from_elem(map_size, None);
    let mut lol = Vec::new();

    while current != end {
        path_lengths[current] = Some(current_length);
        lol.push((current, current_length));
        current_length += 1;

        let (row, col) = current;
        // find the next non wall cell
        for (drow, dcol) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let new_row = row as i64 + drow;
            let new_col = col as i64 + dcol;

            if new_row < 0 || new_row >= nrows as i64 || new_col < 0 || new_col >= ncols as i64 {
                continue;
            }

            let new_row = new_row as usize;
            let new_col = new_col as usize;

            if map[[new_row, new_col]] != Cell::Wall && (new_row, new_col) != previous {
                previous = current;
                current = (new_row, new_col);
                break;
            }
        }
    }

    path_lengths[end] = Some(current_length);
    lol.push((end, current_length));
    (path_lengths, lol)
}

fn compute_part_1(data: &str, threshold: usize) -> usize {
    let (_, map) = parse_input_data(data).expect("Failed to parse input data");
    let (start, exit) = find_start_and_exit(&map);
    let map_view = map.view();
    let (path_lengths, _) = compute_path_lengths(&map_view, start, exit);

    //let non_cheating_length = path_lengths[exit].expect("Failed to find a path without cheats");

    path_lengths
        .windows((1, 3))
        .into_iter()
        .filter(|w| {
            let a = w[(0, 0)];
            let b = w[(0, 1)];
            let c = w[(0, 2)];

            if a.is_none() || b.is_some() || c.is_none() {
                return false;
            }

            a.unwrap().abs_diff(c.unwrap()) > threshold
        })
        .count()
        + path_lengths
            .windows((3, 1))
            .into_iter()
            .filter(|w| {
                let a = w[(0, 0)];
                let b = w[(1, 0)];
                let c = w[(2, 0)];

                if a.is_none() || b.is_some() || c.is_none() {
                    return false;
                }

                a.unwrap().abs_diff(c.unwrap()) > threshold
            })
            .count()
}
pub fn day_20_part_1(data: &str) -> usize {
    compute_part_1(data, 100)
}

#[inline]
fn manhattan_distance(a: (usize, usize), b: (usize, usize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn compute_part_2(data: &str, threshold: usize) -> usize {
    let (_, map) = parse_input_data(data).expect("Failed to parse input data");
    let (start, exit) = find_start_and_exit(&map);
    let map_view = map.view();
    let (path_lengths, perfect_path) = compute_path_lengths(&map_view, start, exit);
    let map_size = map.dim();
    let (nrows, ncols) = map_size;
    let window_size = 21_isize;
    let neg_window_size = -21_isize;

    perfect_path
        .par_iter()
        .map(|(position_start, window_start)| {
            /*let window_start = if let Some(window_start) = window_start {
                *window_start
            } else {
                return 0;
            };*/

            let position_start = *position_start;
            let window_start = *window_start;

            let (start_row, start_col) = position_start;

            let mut count = 0;

            for drow in neg_window_size..window_size {
                for dcol in neg_window_size..window_size {
                    if drow.abs() + dcol.abs() >= window_size {
                        continue;
                    }
                    let end_row: isize = start_row as isize + drow;
                    let end_col: isize = start_col as isize + dcol;

                    if end_row < 0
                        || end_col < 0
                        || end_row >= nrows as isize
                        || end_col >= ncols as isize
                    {
                        continue;
                    }

                    let end_row = end_row as usize;
                    let end_col = end_col as usize;

                    let position_end = (end_row, end_col);

                    if position_start == position_end {
                        continue;
                    }

                    // // max distance is 20
                    // if manhattan_distance(position_start, position_end) > 20 {
                    //     continue;
                    // }

                    let window_end = path_lengths[position_end];

                    if let Some(window_end) = window_end {
                        // Don't go back
                        if window_end > window_start {
                            let distance_with_shortcut = window_end - window_start;
                            let manhattan_distance =
                                manhattan_distance(position_start, position_end);

                            let saved = distance_with_shortcut - manhattan_distance;

                            if saved >= threshold {
                                count += 1;
                            }
                        }
                    }
                }
            }

            count
        })
        .sum()
}

pub fn day_20_part_2(data: &str) -> usize {
    compute_part_2(data, 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn test_day_20_part_1() {
        assert_eq!(compute_part_1(EXAMPLE, 0), 44);
        assert_eq!(compute_part_1(EXAMPLE, 20), 5);
        assert_eq!(day_20_part_1(EXAMPLE), 0);
    }

    #[test]
    fn test_day_20_part_2() {
        assert_eq!(compute_part_2(EXAMPLE, 80), 0);
        assert_eq!(compute_part_2(EXAMPLE, 76), 3);
        assert_eq!(compute_part_2(EXAMPLE, 74), 7);
        assert_eq!(compute_part_2(EXAMPLE, 72), 29);
        assert_eq!(compute_part_2(EXAMPLE, 70), 41);
        assert_eq!(compute_part_2(EXAMPLE, 50), 285);
        assert_eq!(day_20_part_2(EXAMPLE), 0);

        // 961364
    }
}
