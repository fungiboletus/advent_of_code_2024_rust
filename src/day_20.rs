/*
    Comments.
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

fn compute_path_lengths(
    map: &ArrayView2<Cell>,
    start: (usize, usize),
    end: (usize, usize),
) -> Array2<Option<usize>> {
    let map_size = map.dim();
    let (nrows, ncols) = map_size;

    let mut current = start;
    let mut previous = start;
    let mut current_length = 0;

    let mut path_lengths = Array2::from_elem(map_size, None);

    while current != end {
        path_lengths[current] = Some(current_length);
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
    path_lengths
}

fn compute_part_1(data: &str, threshold: usize) -> usize {
    let (_, map) = parse_input_data(data).expect("Failed to parse input data");
    let (start, exit) = find_start_and_exit(&map);
    let map_view = map.view();
    let path_lengths = compute_path_lengths(&map_view, start, exit);

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

pub fn day_20_part_2(data: &str) -> i64 {
    42
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
    }

    #[test]
    fn test_day_20_part_2() {
        assert_eq!(day_20_part_2(EXAMPLE), 42);
    }
}
