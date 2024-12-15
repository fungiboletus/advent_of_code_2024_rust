/*
    Not the most beautiful code today, but I don't enjoy
    working on such problems that much, so that's good enough.

    I didn't try to make it run fast, it runs relatively slowly
    and I didn't implement any kind of optimization.
*/

use ndarray::Array2;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    combinator::map,
    multi::{many1, separated_list1},
    sequence::tuple,
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Box,
    Wall,
    Robot,
    // Part 2
    BoxLeft,
    BoxRight,
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    alt((
        map(tag("^"), |_| Direction::Up),
        map(tag("v"), |_| Direction::Down),
        map(tag("<"), |_| Direction::Left),
        map(tag(">"), |_| Direction::Right),
    ))(input)
}

fn parse_directions(input: &str) -> IResult<&str, Vec<Direction>> {
    map(
        separated_list1(line_ending, many1(parse_direction)),
        |directions| directions.into_iter().flatten().collect(),
    )(input)
}

fn parse_cell(input: &str) -> IResult<&str, Cell> {
    alt((
        map(tag("#"), |_| Cell::Wall),
        map(tag("O"), |_| Cell::Box),
        map(tag("@"), |_| Cell::Robot),
        map(tag("."), |_| Cell::Empty),
    ))(input)
}

fn parse_map(input: &str) -> IResult<&str, Array2<Cell>> {
    map(separated_list1(line_ending, many1(parse_cell)), |grid| {
        let nb_rows = grid.len();
        let nb_cols = grid.first().map_or(0, |row| row.len());

        Array2::from_shape_fn((nb_rows, nb_cols), |(row, col)| grid[row][col])
    })(input)
}

fn parse_input_data(input: &str) -> IResult<&str, (Array2<Cell>, Vec<Direction>)> {
    map(
        tuple((parse_map, line_ending, line_ending, parse_directions)),
        |(grid, _, _, directions)| (grid, directions),
    )(input)
}

#[allow(dead_code)]
fn print_map(map: &Array2<Cell>, robot_position: (usize, usize)) {
    for (row, line) in map.outer_iter().enumerate() {
        for (col, cell) in line.iter().enumerate() {
            if (row, col) == robot_position {
                print!("@");
            } else {
                match cell {
                    Cell::Empty => print!("."),
                    Cell::Box => print!("O"),
                    Cell::Wall => print!("#"),
                    Cell::Robot => panic!("Robot cell should not be in the map"),
                    // Part 2
                    Cell::BoxLeft => print!("["),
                    Cell::BoxRight => print!("]"),
                }
            }
        }
        println!();
    }
}

// returns a boolean showing if the move has been successful and the new position
fn attempt_push(
    map: &mut Array2<Cell>,
    position: (usize, usize),
    direction: Direction,
) -> (bool, (usize, usize)) {
    let (row, col) = position;
    let ncols = map.ncols();
    let nrows = map.nrows();
    let (new_row, new_col) = match direction {
        Direction::Up => {
            if row == 0 {
                return (false, (row, col));
            }
            (row - 1, col)
        }
        Direction::Down => {
            if row == nrows - 1 {
                return (false, (row, col));
            }
            (row + 1, col)
        }
        Direction::Left => {
            if col == 0 {
                return (false, (row, col));
            }
            (row, col - 1)
        }
        Direction::Right => {
            if col == ncols - 1 {
                return (false, (row, col));
            }
            (row, col + 1)
        }
    };

    let cell = map[(new_row, new_col)];
    match (cell, direction) {
        (Cell::Wall, _) => (false, (row, col)),
        (Cell::Robot, _) => panic!("Robot cell should not be in the map"),
        (Cell::Empty, _) => {
            map.swap((row, col), (new_row, new_col));
            (true, (new_row, new_col))
        }
        (Cell::Box, _)
        | (Cell::BoxLeft, Direction::Left)
        | (Cell::BoxLeft, Direction::Right)
        | (Cell::BoxRight, Direction::Left)
        | (Cell::BoxRight, Direction::Right) => {
            let (pushed_successful, _) = attempt_push(map, (new_row, new_col), direction);
            if pushed_successful {
                map.swap((row, col), (new_row, new_col));
                return (true, (new_row, new_col));
            }
            (false, (row, col))
        }
        // Part 2
        (Cell::BoxLeft, Direction::Up) | (Cell::BoxRight, Direction::Up) => {
            if can_push_up(map, (row, col)) {
                push_up(map, (new_row, new_col));
                return (true, (new_row, new_col));
            }
            (false, (row, col))
        }
        (Cell::BoxLeft, Direction::Down) | (Cell::BoxRight, Direction::Down) => {
            if can_push_down(map, (row, col)) {
                push_down(map, (new_row, new_col));
                return (true, (new_row, new_col));
            }
            (false, (row, col))
        }
    }
}

fn can_push_up(map: &Array2<Cell>, position: (usize, usize)) -> bool {
    let (row, col) = position;
    // If already at the top
    if row == 0 {
        return false;
    }
    let cell = map[(row - 1, col)];
    match cell {
        Cell::Wall => false,
        Cell::Empty => true,
        Cell::BoxLeft => can_push_up(map, (row - 1, col)) && can_push_up(map, (row - 1, col + 1)),
        Cell::BoxRight => can_push_up(map, (row - 1, col - 1)) && can_push_up(map, (row - 1, col)),
        Cell::Robot => panic!("Robot cell should not be in the map"),
        Cell::Box => panic!("This function requires a widened map"),
    }
}

fn push_up(map: &mut Array2<Cell>, position: (usize, usize)) {
    let mut visited = Array2::from_elem(map.dim(), false);
    push_up_rec(map, position, &mut visited);
}

fn push_up_rec(map: &mut Array2<Cell>, position: (usize, usize), visited: &mut Array2<bool>) {
    if visited[position] {
        return;
    }
    visited[position] = true;
    let cell = map[position];
    let (row, col) = position;
    // also do the other side
    match cell {
        Cell::BoxLeft => {
            push_up_rec(map, (row, col + 1), visited);
        }
        Cell::BoxRight => {
            push_up_rec(map, (row, col - 1), visited);
        }
        _ => (),
    }
    // get cell above
    let cell = map[(row - 1, col)];
    match cell {
        Cell::BoxLeft => {
            push_up_rec(map, (row - 1, col), visited);
            push_up_rec(map, (row - 1, col + 1), visited);
        }
        Cell::BoxRight => {
            push_up_rec(map, (row - 1, col - 1), visited);
            push_up_rec(map, (row - 1, col), visited);
        }
        _ => (),
    }
    map.swap((row, col), (row - 1, col));
}

fn can_push_down(map: &Array2<Cell>, position: (usize, usize)) -> bool {
    let (row, col) = position;
    let nrows = map.nrows();
    // If already at the bottom
    if row == nrows - 1 {
        return false;
    }
    let cell = map[(row + 1, col)];
    match cell {
        Cell::Wall => false,
        Cell::Empty => true,
        Cell::BoxLeft => {
            can_push_down(map, (row + 1, col)) && can_push_down(map, (row + 1, col + 1))
        }
        Cell::BoxRight => {
            can_push_down(map, (row + 1, col - 1)) && can_push_down(map, (row + 1, col))
        }
        Cell::Robot => panic!("Robot cell should not be in the map"),
        Cell::Box => panic!("This function requires a widened map"),
    }
}

fn push_down(map: &mut Array2<Cell>, position: (usize, usize)) {
    let mut visited = Array2::from_elem(map.dim(), false);
    push_down_rec(map, position, &mut visited);
}

fn push_down_rec(map: &mut Array2<Cell>, position: (usize, usize), visited: &mut Array2<bool>) {
    if visited[position] {
        return;
    }
    visited[position] = true;
    let cell = map[position];
    let (row, col) = position;
    // also do the other side
    match cell {
        Cell::BoxLeft => {
            push_down_rec(map, (row, col + 1), visited);
        }
        Cell::BoxRight => {
            push_down_rec(map, (row, col - 1), visited);
        }
        _ => (),
    }
    // get cell below
    let cell = map[(row + 1, col)];
    match cell {
        Cell::BoxLeft => {
            push_down_rec(map, (row + 1, col), visited);
            push_down_rec(map, (row + 1, col + 1), visited);
        }
        Cell::BoxRight => {
            push_down_rec(map, (row + 1, col - 1), visited);
            push_down_rec(map, (row + 1, col), visited);
        }
        _ => (),
    }
    map.swap((row, col), (row + 1, col));
}

fn extract_robot(map: &mut Array2<Cell>) -> (usize, usize) {
    let robot_position = map
        .indexed_iter()
        .find_map(|(position, &cell)| {
            if cell == Cell::Robot {
                Some(position)
            } else {
                None
            }
        })
        .expect("Robot not found");

    map[robot_position] = Cell::Empty;

    robot_position
}

pub fn day_15_part_1(data: &str) -> i64 {
    let (_, (mut map, directions)) = parse_input_data(data).expect("Failed to parse input data");

    // find the position of the robot
    let mut robot_position = extract_robot(&mut map);

    //print_map(&map, robot_position);

    for direction in directions {
        let (moved, new_position) = attempt_push(&mut map, robot_position, direction);
        if moved {
            robot_position = new_position;
        }
        //print_map(&map, robot_position);
    }

    map.indexed_iter()
        .filter(|(_, &cell)| cell == Cell::Box)
        .map(|((row, col), _)| 100 * row + col)
        .sum::<usize>() as i64
}

fn widen_map(map: &Array2<Cell>) -> Array2<Cell> {
    let nrows = map.nrows();
    let ncols = map.ncols();
    Array2::from_shape_fn((nrows, ncols * 2), |(row, col)| {
        let cell = map[(row, col / 2)];

        let is_left = col % 2 == 0;
        /*if cell == Cell::Robot {
            println!("Robot !!!!");
            println!("{} {}", col, is_left);
        }*/

        match (cell, is_left) {
            (Cell::Robot, true) => Cell::Robot,
            (Cell::Robot, false) => Cell::Empty,
            (Cell::Box, true) => Cell::BoxLeft,
            (Cell::Box, false) => Cell::BoxRight,
            _ => cell,
        }
    })
}

pub fn day_15_part_2(data: &str) -> i64 {
    let (_, (map, directions)) = parse_input_data(data).expect("Failed to parse input data");

    let mut map = widen_map(&map);
    let mut robot_position = extract_robot(&mut map);

    //print_map(&map, robot_position);
    for direction in directions {
        //println!("{:?}", direction);
        let (moved, new_position) = attempt_push(&mut map, robot_position, direction);
        if moved {
            robot_position = new_position;
        }
        //print_map(&map, robot_position);
    }

    map.indexed_iter()
        .filter(|(_, &cell)| cell == Cell::BoxLeft)
        .map(|((row, col), _)| 100 * row + col)
        .sum::<usize>() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_EXAMPLE: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    const BIG_EXAMPLE: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    const EXAMPLE_PART_TWO: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

    #[test]
    fn test_day_15_part_1() {
        assert_eq!(day_15_part_1(SMALL_EXAMPLE), 2028);
        assert_eq!(day_15_part_1(BIG_EXAMPLE), 10092);
    }

    #[test]
    fn test_day_15_part_2() {
        assert_eq!(day_15_part_2(EXAMPLE_PART_TWO), 618);
        assert_eq!(day_15_part_2(BIG_EXAMPLE), 9021);
    }
}
