/*
    Comments.
*/
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    fmt::Binary,
};

use ndarray::{Array2, Array3, ArrayView2};
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

#[derive(Debug, PartialEq, Eq)]
struct SearchPath {
    f_score: usize,
    g_score: usize,
    position: (usize, usize),
    cheats_left: usize,
    cheats_positions: Option<Vec<(usize, usize)>>,
}

impl Ord for SearchPath {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.f_score.cmp(&other.f_score)
    }
}
impl PartialOrd for SearchPath {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn find_shortest_path(
    map: &ArrayView2<Cell>,
    start: (usize, usize),
    exit: (usize, usize),
    allowed_number_of_cheats: usize,
    forbidden_positions: &HashSet<Vec<(usize, usize)>>,
) -> Option<(usize, Option<Vec<(usize, usize)>>)> {
    let grid_size = map.dim();
    let nrows = grid_size.0;
    let ncols = grid_size.1;

    let mut priority_queue: BinaryHeap<Reverse<SearchPath>> = BinaryHeap::new();
    let initial_cost = manhattan_distance(start, exit);
    priority_queue.push(Reverse(SearchPath {
        f_score: initial_cost,
        g_score: 0,
        position: start,
        cheats_left: allowed_number_of_cheats,
        cheats_positions: None,
    }));

    let mut visited: Array3<Option<usize>> =
        Array3::from_elem((nrows, ncols, allowed_number_of_cheats + 1), None);

    while let Some(Reverse(SearchPath {
        f_score: _,
        g_score,
        position,
        cheats_left,
        cheats_positions,
    })) = priority_queue.pop()
    {
        if position == exit {
            return Some((g_score, cheats_positions));
        }

        let (row, col) = position;

        if let Some(visited_g_score) = visited[[row, col, cheats_left]] {
            if visited_g_score <= g_score {
                continue;
            }
        }

        visited[[row, col, cheats_left]] = Some(g_score);

        for (drow, dcol) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let new_row = row as i64 + drow;
            let new_col = col as i64 + dcol;

            if new_row < 0 || new_row >= nrows as i64 || new_col < 0 || new_col >= ncols as i64 {
                continue;
            }

            let new_row = new_row as usize;
            let new_col = new_col as usize;

            let is_wall = map[[new_row, new_col]] == Cell::Wall;

            if cheats_left < 2 && is_wall {
                continue;
            }

            let will_cheat = (is_wall && cheats_left == 2) || cheats_left == 1;

            let cheats_positions = if will_cheat {
                if let Some(cheats_positions) = cheats_positions.clone() {
                    let mut new_cheats_positions = cheats_positions.clone();
                    new_cheats_positions.push((new_row, new_col));
                    Some(new_cheats_positions)
                } else {
                    Some(vec![(new_row, new_col)])
                }
            } else {
                cheats_positions.clone()
            };

            let cheats_left = if will_cheat {
                //println!("Cheating at ({}, {})", new_row, new_col);
                cheats_left - 1
            } else {
                cheats_left
            };

            // Forbidden cheats
            if will_cheat && cheats_left == 0 {
                if let Some(cheats_positions) = &cheats_positions {
                    if forbidden_positions.contains(cheats_positions) {
                        continue;
                    }
                }
            }

            let new_g_score = g_score + 1;
            let new_f_score = new_g_score + manhattan_distance((new_row, new_col), exit);

            priority_queue.push(Reverse(SearchPath {
                f_score: new_f_score,
                g_score: new_g_score,
                position: (new_row, new_col),
                cheats_left,
                cheats_positions,
            }));
        }
    }

    None
}

fn compute_part_1(data: &str, threshold: usize) -> usize {
    let (_, map) = parse_input_data(data).expect("Failed to parse input data");
    let (start, exit) = find_start_and_exit(&map);
    let map_view = map.view();
    let mut forbidden_positions = HashSet::new();
    let (path_size_without_cheats, lol) =
        find_shortest_path(&map_view, start, exit, 0, &forbidden_positions)
            .expect("Failed to find a path without cheats");
    //println!("Path size without cheats: {}", path_size_without_cheats);
    assert!(lol.is_none());

    let mut number_of_shortcuts = 0;

    loop {
        let (path_size_with_cheats, cheats) =
            find_shortest_path(&map_view, start, exit, 2, &forbidden_positions)
                .expect("Failed to find a path with cheats, weird weird weird");
        //println!("Path size with cheats: {}", path_size_with_cheats);
        //println!("Cheats: {:?}", cheats);

        if let Some(cheats) = cheats {
            forbidden_positions.insert(cheats);
        } else {
            break;
        }

        let diff = path_size_without_cheats - path_size_with_cheats;
        //println!("saving: {}", diff);
        if diff > threshold {
            number_of_shortcuts += 1;
        } else {
            //println!("No more worsening");
            break;
        }
    }

    number_of_shortcuts
}
pub fn day_20_part_1(data: &str) -> usize {
    compute_part_1(data, 99)
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
    }

    #[test]
    fn test_day_20_part_2() {
        assert_eq!(day_20_part_2(EXAMPLE), 42);
    }
}
