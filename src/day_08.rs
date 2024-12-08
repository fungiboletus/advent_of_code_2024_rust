/*
    Part 1 is somewhat simple, though I definitely don't enjoy
    working with indices on matrix.

    I used itertools' permutations utility which is neat.
    I should use it more often.
*/

use itertools::Itertools;
use ndarray::Array2;
use nom::{
    branch::alt,
    character::{
        complete::{line_ending, satisfy},
        is_alphanumeric,
    },
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};

fn ascii_to_key(ascii: char) -> Option<usize> {
    //let ascii = ascii as u8;
    match ascii {
        'A'..='Z' => Some(ascii as usize - 65),
        'a'..='z' => Some(ascii as usize - 97 + 26),
        '0'..='9' => Some(ascii as usize - 48 + 52),
        '.' => None,
        _ => panic!("Invalid character"),
    }
}

#[allow(dead_code)]
fn print_presence_map(map: &Array2<bool>) {
    for row in map.outer_iter() {
        for presence in row {
            print!("{}", if *presence { '#' } else { '.' });
        }
        println!();
    }
}

fn parse_input_data(data: &str) -> IResult<&str, Array2<char>> {
    map(
        separated_list1(
            line_ending,
            many1(alt((
                satisfy(|c| is_alphanumeric(c as u8)),
                nom::character::complete::char('.'),
            ))),
        ),
        |rows| {
            let nb_rows = rows.len();
            let nb_cols = rows.first().map_or(0, |row| row.len());

            Array2::from_shape_fn((nb_rows, nb_cols), |(row, col)| rows[row][col])
        },
    )(data)
}

pub fn day_08_part_1(data: &str) -> i64 {
    let (_, grid) = parse_input_data(data).expect("Failed to parse input data");

    const NB_KEYS: usize = 62;
    let mut antennas_per_key: Vec<Vec<(usize, usize)>> = vec![Vec::new(); NB_KEYS];

    for (position, value) in grid.indexed_iter() {
        if let Some(key) = ascii_to_key(*value) {
            antennas_per_key[key].push(position)
        }
    }

    let mut antinodes_presence_map = Array2::from_elem(grid.dim(), false);
    let nb_cols = grid.ncols() as i64;
    let nb_rows = grid.nrows() as i64;

    for antennas_group in antennas_per_key.iter() {
        if antennas_group.is_empty() {
            continue;
        }

        //println!("--------------");
        //println!("{:?}", antennas_group);

        for window in antennas_group.iter().permutations(2) {
            let (col_a, row_a) = window[0];
            let (col_b, row_b) = window[1];
            //println!("{:?} {:?}", window[0], window[1]);
            let col_a = *col_a as i64;
            let row_a = *row_a as i64;
            let col_b = *col_b as i64;
            let row_b = *row_b as i64;

            let diff_col = col_b - col_a;
            let diff_row = row_b - row_a;

            let antipode = (col_a - diff_col, row_a - diff_row);
            //let antipode_two = (col_b + diff_col, row_b + diff_row);

            //println!("{:?} {:?}", antipode_one, antipode_two);
            if antipode.0 >= 0 && antipode.1 >= 0 && antipode.0 < nb_cols && antipode.1 < nb_rows {
                antinodes_presence_map[(antipode.0 as usize, antipode.1 as usize)] = true;
            }
            /*if antipode_two.0 >= 0
                && antipode_two.1 >= 0
                && antipode_two.0 < grid.ncols() as i64
                && antipode_two.1 < grid.nrows() as i64
            {
                antinodes_presence_map[(antipode_two.0 as usize, antipode_two.1 as usize)] = true;
            }*/
        }
        //print_presence_map(&antinodes_presence_map);
    }

    //println!("{:?}", antennas_per_key);

    antinodes_presence_map
        .iter()
        .filter(|&&presence| presence)
        .count() as i64
}

pub fn day_08_part_2(data: &str) -> i64 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_day_08_part_1() {
        assert_eq!(
            day_08_part_1(
                "..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
.........."
            ),
            2
        );
        assert_eq!(
            day_08_part_1(
                "..........
..........
..........
....a.....
........a.
.....a....
..........
..........
..........
.........."
            ),
            4
        );
        assert_eq!(day_08_part_1(EXAMPLE), 14);
    }

    #[test]
    fn test_day_08_part_2() {
        assert_eq!(day_08_part_2(EXAMPLE), 42);
    }
}
