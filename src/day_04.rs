/*
    Comments.
*/
use ndarray::prelude::*;
use ndarray::{Array2, Axis};
use nom::{
    character::complete::{line_ending, one_of},
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};

fn parse_input_data(data: &str) -> IResult<&str, Array2<char>> {
    map(
        separated_list1(line_ending, many1(one_of("XMAS.0123456789BCDEF"))),
        |rows| {
            let nb_rows = rows.len();
            let nb_cols = rows.first().map_or(0, |row| row.len());

            Array2::from_shape_fn((nb_rows, nb_cols), |(row, col)| rows[row][col])
        },
    )(data)
}

// Infortunately, the ndarray in rust doesn't take a offset/k parameter
// like with numpy. so we have to play with slices.
fn diag_with_offset(grid: &Array2<char>, offset: isize) -> String {
    match offset {
        0 => grid.diag().iter().collect::<String>(),
        _ if offset > 0 => grid
            .slice(s![offset.., ..])
            .diag()
            .iter()
            .collect::<String>(),
        _ => grid
            .slice(s![.., -offset..])
            .diag()
            .iter()
            .collect::<String>(),
    }
}

fn extract_strings(grid: &Array2<char>, min_string_size: usize) -> Vec<String> {
    let nb_rows = grid.nrows();
    let nb_cols = grid.ncols();

    let max_strings = 3 * (nb_rows + nb_cols) - 2;
    let mut strings = Vec::with_capacity(max_strings);
    println!("max_strings: {}", max_strings);

    // row by row
    for row in grid.rows() {
        println!("row: {:?}", row);
        let row_string = row.iter().collect::<String>();
        println!("row_string: {}", row_string);
        strings.push(row_string);
    }

    // col by col
    for col in grid.columns() {
        println!("col: {:?}", col);
        let col_string = col.iter().collect::<String>();
        println!("col_string: {}", col_string);
        strings.push(col_string);
    }

    let mut rotate_90_grid = grid.clone();
    rotate_90_grid.swap_axes(0, 1);
    rotate_90_grid.invert_axis(Axis(0));

    strings.push(diag_with_offset(grid, 0));
    strings.push(diag_with_offset(&rotate_90_grid, 0));
    for offset in 1..nb_rows {
        strings.push(diag_with_offset(grid, offset as isize));
        strings.push(diag_with_offset(&rotate_90_grid, -(offset as isize)));
    }
    for offset in 1..nb_cols {
        strings.push(diag_with_offset(grid, -(offset as isize)));
        strings.push(diag_with_offset(&rotate_90_grid, offset as isize));
    }

    strings
}

pub fn day_04_part_1(data: &str) -> i64 {
    let (_, grid) = parse_input_data(data).expect("Failed to parse input data");
    println!("normal:\n{:?}", grid);

    /*println!("diag 0: {:?}", diag_with_offset(&grid, 0));
    println!("diag 1: {:?}", diag_with_offset(&grid, 1));
    println!("diag -1: {:?}", diag_with_offset(&grid, -1));
    println!("diag 2: {:?}", diag_with_offset(&grid, 2));
    println!("diag -2: {:?}", diag_with_offset(&grid, -2));
    println!("diag 3: {:?}", diag_with_offset(&grid, 3));
    println!("diag -3: {:?}", diag_with_offset(&grid, -3));*/

    let strings = extract_strings(&grid, 4);
    println!("strings: {:?}", strings);

    strings
        .iter()
        .map(|string| string.matches("XMAS").count() + string.matches("SAMX").count())
        .sum::<usize>() as i64

    // diagonal of grid
    /*let diagonal = grid.diag();
    println!("diagonal:\n{:?}", diagonal);

    // diagonal offset 1
    let tmp = grid.slice(s![1.., ..]);
    let diagonal_offset_1 = tmp.diag();
    println!("diagonal offset 1:\n{:?}", diagonal_offset_1);

    // diagonal offset -1
    let tmp = grid.slice(s![.., 1..]);
    let diagonal_offset_minus_1 = tmp.diag();
    println!("diagonal offset -1:\n{:?}", diagonal_offset_minus_1);

    // diagonal offset -2
    let tmp = grid.slice(s![.., 2..]);
    let diagonal_offset_minus_2 = tmp.diag();
    println!("diagonal offset -2:\n{:?}", diagonal_offset_minus_2);

    // transposed grid
    let transposed_grid = grid.t();
    println!("transposed:\n{:?}", transposed_grid);

    let mut horizontal_flip_grid = grid.clone();
    horizontal_flip_grid.invert_axis(Axis(1));
    println!("horizontal flip:\n{:?}", horizontal_flip_grid);

    //let mut vertical_flip_grid = grid.clone();
    //vertical_flip_grid.invert_axis(Axis(0));
    //println!("vertical flip:\n{:?}", vertical_flip_grid);

    // rotate 90 degrees
    let mut rotate_90_grid = grid.clone();
    rotate_90_grid.swap_axes(0, 1);
    rotate_90_grid.invert_axis(Axis(0));
    println!("rotate 90:\n{:?}", rotate_90_grid);

    // horizontal flip rotate 90
    let mut horizontal_flip_rotate_90_grid = rotate_90_grid.clone();
    horizontal_flip_rotate_90_grid.invert_axis(Axis(1));
    println!(
        "horizontal flip rotate 90:\n{:?}",
        horizontal_flip_rotate_90_grid
    );*/
}

pub fn day_04_part_2(data: &str) -> i64 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_SMALL: &str = "..X...
.SAMX.
.A..A.
XMAS.S
.X....";

    const EXAMPLE_BIG: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_day_04_part_1() {
        assert_eq!(day_04_part_1("01\n23\n45\n67"), 0);
        assert_eq!(day_04_part_1("0123\n4567"), 0);
        assert_eq!(day_04_part_1("012\n345\n678"), 0);
        assert_eq!(day_04_part_1("0123\n4567\n89AB\nCDEF"), 0);
        assert_eq!(day_04_part_1(EXAMPLE_SMALL), 4);
        assert_eq!(day_04_part_1(EXAMPLE_BIG), 18);
    }

    #[test]
    fn test_day_04_part_2() {
        assert_eq!(day_04_part_2(EXAMPLE_BIG), 42);
    }
}
