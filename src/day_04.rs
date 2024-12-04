/*
    Day 04 is an easy day with a grid/matrix of characters.

    However, I hate working with indexes and I decided to use
    the ndarray and fancy operations to work with the grid.

    It took some time to read the documentation and find what to do.

    Second part was very easy and I did quick and dirty.

    Could have done part 1 like this to be honest.
*/
//use ndarray::prelude::*;
use ndarray::{Array2 /*,  Axis*/};
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
/*fn diag_with_offset(grid: &Array2<char>, offset: isize) -> String {
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

    // row by row
    if nb_rows >= min_string_size {
        for row in grid.rows() {
            strings.push(row.iter().collect::<String>());
        }
    }

    // col by col
    if nb_cols >= min_string_size {
        for col in grid.columns() {
            strings.push(col.iter().collect::<String>());
        }
    }

    // Quick shortcut if the grid is too small
    if nb_rows * nb_rows + nb_cols * nb_cols < min_string_size * min_string_size {
        return strings;
    }

    // The diagonals, in both directions
    // We rotate the grid by 90 degrees to reuse the same code
    let mut rotate_90_grid = grid.clone();
    rotate_90_grid.swap_axes(0, 1);
    rotate_90_grid.invert_axis(Axis(0));

    strings.push(diag_with_offset(grid, 0));
    strings.push(diag_with_offset(&rotate_90_grid, 0));
    for offset in 1..nb_rows.saturating_sub(min_string_size - 1) {
        strings.push(diag_with_offset(grid, offset as isize));
        strings.push(diag_with_offset(&rotate_90_grid, -(offset as isize)));
    }
    for offset in 1..nb_cols.saturating_sub(min_string_size - 1) {
        strings.push(diag_with_offset(grid, -(offset as isize)));
        strings.push(diag_with_offset(&rotate_90_grid, offset as isize));
    }

    strings
}*/

pub fn day_04_part_1(data: &str) -> i64 {
    let (_, grid) = parse_input_data(data).expect("Failed to parse input data");

    /*let strings = extract_strings(&grid, 4);

    strings
        .iter()
        // Small trick, we look at the reversed trick so we don't
        // have to do a lot of matrix rotations and flips and so on.
        .map(|string| string.matches("XMAS").count() + string.matches("SAMX").count())
        .sum::<usize>() as i64*/

    (grid
        .windows((1, 4))
        .into_iter()
        .filter(|w| {
            let a = w[(0, 0)];
            let b = w[(0, 1)];
            let c = w[(0, 2)];
            let d = w[(0, 3)];

            (a == 'X' && b == 'M' && c == 'A' && d == 'S')
                || (a == 'S' && b == 'A' && c == 'M' && d == 'X')
        })
        .count()
        + grid
            .windows((4, 1))
            .into_iter()
            .filter(|w| {
                let a = w[(0, 0)];
                let b = w[(1, 0)];
                let c = w[(2, 0)];
                let d = w[(3, 0)];

                (a == 'X' && b == 'M' && c == 'A' && d == 'S')
                    || (a == 'S' && b == 'A' && c == 'M' && d == 'X')
            })
            .count()
        + grid
            .windows((4, 4))
            .into_iter()
            .map(|w| {
                /*
                 * a . . b
                 * . c d .
                 * . e f .
                 * g . . h
                 */
                let a = w[(0, 0)];
                let b = w[(0, 3)];
                let c = w[(1, 1)];
                let d = w[(1, 2)];
                let e = w[(2, 1)];
                let f = w[(2, 2)];
                let g = w[(3, 0)];
                let h = w[(3, 3)];

                (((a == 'X' && c == 'M' && f == 'A' && h == 'S')
                    || (a == 'S' && c == 'A' && f == 'M' && h == 'X')) as usize)
                    + (((b == 'X' && d == 'M' && e == 'A' && g == 'S')
                        || (b == 'S' && d == 'A' && e == 'M' && g == 'X'))
                        as usize)
            })
            .sum::<usize>()) as i64
}

pub fn day_04_part_2(data: &str) -> i64 {
    let (_, grid) = parse_input_data(data).expect("Failed to parse input data");

    grid.windows((3, 3))
        .into_iter()
        .filter(|w| {
            let middle = w[(1, 1)];
            let top_left = w[(0, 0)];
            let top_right = w[(0, 2)];
            let bottom_left = w[(2, 0)];
            let bottom_right = w[(2, 2)];

            // Could be more optimised but well… it's boring.
            // Hopefully the compiler will notice since
            // we extracted the values into local variables first.
            (middle == 'A')
                && (/*
                M . S
                . A .
                M . S
                */(top_left == 'M'
            && top_right == 'S'
            && bottom_left == 'M'
            && bottom_right == 'S')
        /*
        S . M
        . A .
        S . M
         */
        || (top_left == 'S'
            && top_right == 'M'
            && bottom_left == 'S'
            && bottom_right == 'M')
        /*
        M . M
        . A .
        S . S
         */
        || (top_left == 'M'
            && top_right == 'M'
            && bottom_left == 'S'
            && bottom_right == 'S')
        /*
        S . S
        . A .
        M . M
         */
        || (top_left == 'S'
            && top_right == 'S'
            && bottom_left == 'M'
            && bottom_right == 'M'))
        })
        .count() as i64
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

    const EXAMPLE_PART_2: &str = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";

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
        assert_eq!(day_04_part_2(EXAMPLE_PART_2), 9);
        assert_eq!(day_04_part_2(EXAMPLE_BIG), 9);
    }
}
