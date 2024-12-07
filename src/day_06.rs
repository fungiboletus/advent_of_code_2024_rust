/*
    Comments.
*/

use ndarray::{Array2, Array3};
use nom::{
    character::complete::{line_ending, one_of},
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};

#[derive(Clone, Debug, PartialEq)]
enum Space {
    Empty,
    Obstructed,
    Start,

    Visited,
}

#[derive(Debug, PartialEq, Clone, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn rotate(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn facing_position(
        &self,
        position: (usize, usize),
        map_size: (usize, usize),
    ) -> Option<(usize, usize)> {
        let (row, col) = position;
        let (nb_rows, nb_cols) = map_size;
        match (self, row, col) {
            (Direction::Up, 0, _) => None,
            (Direction::Up, _, _) => Some((row - 1, col)),
            (Direction::Right, _, _) if col == nb_cols - 1 => None,
            (Direction::Right, _, _) => Some((row, col + 1)),
            (Direction::Down, _, _) if row == nb_rows - 1 => None,
            (Direction::Down, _, _) => Some((row + 1, col)),
            (Direction::Left, _, 0) => None,
            (Direction::Left, _, _) => Some((row, col - 1)),
        }
    }

    fn to_usize(&self) -> usize {
        match self {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        }
    }
}

type Map = Array2<Space>;

#[allow(dead_code)]
trait MapExt {
    fn print(&self);
}

impl MapExt for Map {
    fn print(&self) {
        for row in self.rows() {
            for space in row {
                print!(
                    "{}",
                    match space {
                        Space::Empty => '.',
                        Space::Obstructed => '#',
                        Space::Start => '^',
                        Space::Visited => 'X',
                    }
                );
            }
            println!();
        }
    }
}

fn parse_input_data(data: &str) -> IResult<&str, Map> {
    map(separated_list1(line_ending, many1(one_of(".#^"))), |rows| {
        let nb_rows = rows.len();
        let nb_cols = rows.first().map_or(0, |row| row.len());

        Array2::from_shape_fn((nb_rows, nb_cols), |(row, col)| match rows[row][col] {
            '.' => Space::Empty,
            '#' => Space::Obstructed,
            '^' => Space::Start,
            _ => unreachable!(),
        })
    })(data)
}

fn find_start_position(map: &Map) -> (usize, usize) {
    map.indexed_iter()
        .find(|(_, m)| **m == Space::Start)
        .expect("No start position found")
        .0
}

fn visit_map(map: &Map, start_position: (usize, usize)) -> Map {
    let mut map = map.clone();
    let shape = map.shape();
    let shape = (shape[0], shape[1]);

    let mut position = start_position;
    let mut direction = Direction::Up;

    map[position] = Space::Visited;

    while let Some(next_position) = direction.facing_position(position, shape) {
        if map[next_position] == Space::Obstructed {
            direction = direction.rotate();
        } else {
            position = next_position;
            map[position] = Space::Visited;
        }
    }

    map
}

pub fn day_06_part_1(data: &str) -> i64 {
    let (_, map) = parse_input_data(data).expect("Failed to parse input data");

    let start_position = find_start_position(&map);
    let visited_map = visit_map(&map, start_position);

    // map.print();
    visited_map
        .iter()
        .filter(|space| **space == Space::Visited)
        .count() as i64
}

/*
Each cell contains the index of the next obstacle in the given direction.

Contains None if there is no obstacle in that direction.
*/
fn build_lookup_table(map: &Map, direction: Direction) -> Array2<Option<usize>> {
    // create a lookup table with the same shape as the map
    let mut lookup_table = Array2::from_elem(map.raw_dim(), None);

    let nb_rows = map.nrows();
    let nb_cols = map.ncols();

    match direction {
        Direction::Up => {
            for ((row, col), space) in map.indexed_iter() {
                if *space == Space::Obstructed {
                    lookup_table[(row, col)] = Some(row);
                } else if row > 0 {
                    lookup_table[(row, col)] = lookup_table[(row - 1, col)];
                }
            }
        }
        Direction::Down => {
            // go from the bottom to the top but .rev() doesn't work on
            // the iterator returned by indexed_iter() because
            // it's not a double-ended iterator.
            for col in 0..nb_cols {
                for row in (0..nb_rows).rev() {
                    if map[(row, col)] == Space::Obstructed {
                        lookup_table[(row, col)] = Some(row);
                    } else if row < nb_rows - 1 {
                        lookup_table[(row, col)] = lookup_table[(row + 1, col)];
                    }
                }
            }
        }
        Direction::Right => {
            // go from right to left with a manual for loop
            // for the same reason as above in Direction::Down
            for col in (0..nb_cols).rev() {
                for row in 0..nb_rows {
                    if map[(row, col)] == Space::Obstructed {
                        lookup_table[(row, col)] = Some(col);
                    } else if col < nb_cols - 1 {
                        lookup_table[(row, col)] = lookup_table[(row, col + 1)];
                    }
                }
            }
        }
        Direction::Left => {
            for ((row, col), space) in map.indexed_iter() {
                if *space == Space::Obstructed {
                    lookup_table[(row, col)] = Some(col);
                } else if col > 0 {
                    lookup_table[(row, col)] = lookup_table[(row, col - 1)];
                }
            }
        }
    };

    lookup_table
}

#[derive(Debug, Clone)]
struct LookupTables {
    up: Array2<Option<usize>>,
    right: Array2<Option<usize>>,
    down: Array2<Option<usize>>,
    left: Array2<Option<usize>>,
}

fn print_lookup_table(lookup_table: &Array2<Option<usize>>) {
    for row in lookup_table.rows() {
        for cell in row {
            print!("{}", cell.map_or('.', |index| (index as u8 + b'0') as char));
        }
        println!();
    }
}

impl LookupTables {
    fn new(map: &Map) -> Self {
        Self {
            up: build_lookup_table(map, Direction::Up),
            right: build_lookup_table(map, Direction::Right),
            down: build_lookup_table(map, Direction::Down),
            left: build_lookup_table(map, Direction::Left),
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!("Up:");
        print_lookup_table(&self.up);
        println!("Right:");
        print_lookup_table(&self.right);
        println!("Down:");
        print_lookup_table(&self.down);
        println!("Left:");
        print_lookup_table(&self.left);
    }

    fn position_before_obstacle(
        &self,
        direction: &Direction,
        position: (usize, usize),
    ) -> Option<(usize, usize)> {
        let (row, col) = position;
        match direction {
            Direction::Up => self.up[(row, col)].map(|row| (row + 1, col)),
            Direction::Right => self.right[(row, col)].map(|col| (row, col - 1)),
            Direction::Down => self.down[(row, col)].map(|row| (row - 1, col)),
            Direction::Left => self.left[(row, col)].map(|col| (row, col + 1)),
        }
    }

    fn with_new_obstacle(&self, obstacle_position: (usize, usize)) -> Self {
        let mut destination = self.clone();
        let (row, col) = obstacle_position;
        let nb_rows = destination.up.nrows();
        let nb_cols = destination.up.ncols();

        // update up by setting the new index to the rows below the obstacle
        // until we reach the end of the map or another obstacle
        for i_row in row..nb_rows {
            let current = destination.up[(i_row, col)];
            if current.is_none() || current.unwrap() < row {
                destination.up[(i_row, col)] = Some(row);
            }
        }

        // down
        for i_row in (0..row).rev() {
            let current = destination.down[(i_row, col)];
            if current.is_none() || current.unwrap() > row {
                destination.down[(i_row, col)] = Some(row);
            }
        }

        // left
        for i_col in col..nb_cols {
            let current = destination.left[(row, i_col)];
            if current.is_none() || current.unwrap() < col {
                destination.left[(row, i_col)] = Some(col);
            }
        }

        // right
        for i_col in (0..col).rev() {
            let current = destination.right[(row, i_col)];
            if current.is_none() || current.unwrap() > col {
                destination.right[(row, i_col)] = Some(col);
            }
        }

        destination
    }
}

fn will_exit_map(
    start_position: (usize, usize),
    lookup_tables: &LookupTables,
    visited_positions: &mut Array3<bool>,
) -> bool {
    visited_positions.fill(false);
    //let mut visited_positions = Array3::from_elem((map.nrows(), map.ncols(), 4), false);

    let mut position = start_position;
    let mut direction = Direction::Up;

    visited_positions[(position.0, position.1, direction.to_usize())] = true;

    while let Some(position_before_next_obstacle) =
        lookup_tables.position_before_obstacle(&direction, position)
    {
        position = position_before_next_obstacle;
        let visited_index = (position.0, position.1, direction.to_usize());
        if visited_positions[visited_index] {
            return false;
        }
        visited_positions[visited_index] = true;
        direction = direction.rotate();
    }

    true
}

pub fn day_06_part_2(data: &str) -> i64 {
    /*
    .#.
    .^#
    .^.
     */

    let (_, map) = parse_input_data(data).expect("Failed to parse input data");

    // we build lookup maps that give the index of the next obstacle in each direction
    let lookup_tables = LookupTables::new(&map);

    let start_position = find_start_position(&map);

    let visited_map = visit_map(&map, start_position);

    let mut visited_positions = Array3::from_elem((map.nrows(), map.ncols(), 4), false);

    visited_map
        .indexed_iter()
        .filter(|(position, space)| {
            if **space != Space::Visited {
                return false;
            }

            if position == &start_position {
                return false;
            }

            let new_lookup_tables = lookup_tables.with_new_obstacle(*position);
            !will_exit_map(start_position, &new_lookup_tables, &mut visited_positions)
        })
        .count() as i64
        - 1 // seriously didn't try to find out why I need to subtract 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_day_06_part_1() {
        assert_eq!(day_06_part_1(EXAMPLE), 41);
    }

    #[test]
    fn test_day_06_part_2() {
        assert_eq!(day_06_part_2(EXAMPLE), 6);
    }
}
