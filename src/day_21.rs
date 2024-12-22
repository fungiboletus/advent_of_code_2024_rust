/*
    A challenging puzzle.

    I'm happy with the solution I slowly came up to. I didn't go sideway
    that much.

    However, I lost motivation and time to finish on day 21. I realised that I indeed had to test all possible paths and not just
    go with a short path on the num pad. It did sound cumbersome and it was
    already late.

    I had an algorithm that builds the shortest path without zigzagging,
    as I identified that zigzagging couldn't be optimal. I only
    went horizontal first or vertical first, but not both.

    Going horizontal first if possible worked on the example data, but not on the actual data. So… that was about it and I went to bed.

    The next day, I adapted the solution to return two paths instead of
    one, as I confirmed that my no-zigzagging algorithm was correct.

    Then I got bored and I followed this thread: https://old.reddit.com/r/adventofcode/comments/1hjx0x4/2024_day_21_quick_tutorial_to_solve_part_2_in/

    Part 2 was then solved like part 1. It's somewhat fast.

    I wouldn't have solved it without the guide. I knew it was something
    related to find patterns in the sequences and doing something clever,
    but I didn't have the time nor the motivation to do it.

    Hard puzzle, happy to have it done with the solution.
*/

use cached::proc_macro::cached;
use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, one_of},
    combinator::map,
    multi::{count, separated_list1},
    sequence::tuple,
    IResult,
};
use std::collections::HashMap;

/*
+---+---+---+
| 7 | 8 | 9 | 0
+---+---+---+
| 4 | 5 | 6 | 1
+---+---+---+
| 1 | 2 | 3 | 2
+---+---+---+
    | 0 | A | 3
    +---+---+
  0   1   2
*/
// row, col order
type PadPosition = (usize, usize);
static NUMPAD_0: PadPosition = (3, 1);
static NUMPAD_1: PadPosition = (2, 0);
static NUMPAD_2: PadPosition = (2, 1);
static NUMPAD_3: PadPosition = (2, 2);
static NUMPAD_4: PadPosition = (1, 0);
static NUMPAD_5: PadPosition = (1, 1);
static NUMPAD_6: PadPosition = (1, 2);
static NUMPAD_7: PadPosition = (0, 0);
static NUMPAD_8: PadPosition = (0, 1);
static NUMPAD_9: PadPosition = (0, 2);
static NUMPAD_A: PadPosition = (3, 2);

#[inline]
fn char_to_keypad_position(c: char) -> PadPosition {
    match c {
        '0' => NUMPAD_0,
        '1' => NUMPAD_1,
        '2' => NUMPAD_2,
        '3' => NUMPAD_3,
        '4' => NUMPAD_4,
        '5' => NUMPAD_5,
        '6' => NUMPAD_6,
        '7' => NUMPAD_7,
        '8' => NUMPAD_8,
        '9' => NUMPAD_9,
        'A' => NUMPAD_A,
        _ => panic!("Invalid character in sequence"),
    }
}

type PadPaths = HashMap<(char, char), Vec<String>>;
#[cached]
fn build_keypad_paths() -> PadPaths {
    // We build with an algorithm because it's actually 90 possible paths
    static CHARS: &[char] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A'];
    let mut pad_paths = HashMap::new();

    for &char_start in CHARS {
        for &char_stop in CHARS {
            //if char_start != '0' || char_stop != '4' {
            //    continue;
            // }
            let position_start = char_to_keypad_position(char_start);
            let position_stop = char_to_keypad_position(char_stop);
            let (row_start, col_start) = position_start;
            let (row_stop, col_stop) = position_stop;

            let direction_row = if row_start < row_stop { 1 } else { -1 };
            let direction_col = if col_start < col_stop { 1 } else { -1 };
            let letter_row = if direction_row == 1 { 'v' } else { '^' };
            let letter_col = if direction_col == 1 { '>' } else { '<' };

            let mut route_paths = Vec::with_capacity(2);

            // horizontal first path
            if row_start != 3 || col_stop != 0 {
                let mut horizontal_first_path = String::new();
                let mut current_row = row_start;
                let mut current_col = col_start;
                while current_col != col_stop {
                    horizontal_first_path.push(letter_col);
                    current_col = (current_col as i64 + direction_col) as usize;
                }
                while current_row != row_stop {
                    horizontal_first_path.push(letter_row);
                    current_row = (current_row as i64 + direction_row) as usize;
                }
                horizontal_first_path.push('A');
                route_paths.push(horizontal_first_path);
            }

            // vertical first path
            if col_start != 0 || row_stop != 3 {
                let mut vertical_first_path = String::new();
                let mut current_row = row_start;
                let mut current_col = col_start;
                while current_row != row_stop {
                    vertical_first_path.push(letter_row);
                    current_row = (current_row as i64 + direction_row) as usize;
                }
                while current_col != col_stop {
                    vertical_first_path.push(letter_col);
                    current_col = (current_col as i64 + direction_col) as usize;
                }
                vertical_first_path.push('A');
                route_paths.push(vertical_first_path);
            }

            pad_paths.insert((char_start, char_stop), route_paths);
        }
    }

    //println!("{:?}", keypad);
    //println!("{:?}", paths);

    pad_paths
}

//let new_position = char_to_keypad_position(c);

/*
    +---+---+
    | ^ | A | 0
+---+---+---+
| < | v | > | 1
+---+---+---+
  0   1   2
*/
// row, col order
static DIRECTIONAL_PAD_UP: PadPosition = (0, 1);
static DIRECTIONAL_PAD_DOWN: PadPosition = (1, 1);
static DIRECTIONAL_PAD_LEFT: PadPosition = (1, 0);
static DIRECTIONAL_PAD_RIGHT: PadPosition = (1, 2);
static DIRECTIONAL_PAD_A: PadPosition = (0, 2);

#[inline]
fn char_to_directional_pad_position(c: char) -> PadPosition {
    match c {
        '^' => DIRECTIONAL_PAD_UP,
        'v' => DIRECTIONAL_PAD_DOWN,
        '<' => DIRECTIONAL_PAD_LEFT,
        '>' => DIRECTIONAL_PAD_RIGHT,
        'A' => DIRECTIONAL_PAD_A,
        _ => panic!("Invalid character in sequence"),
    }
}

#[cached]
fn build_directional_pad_paths() -> PadPaths {
    static CHARS: &[char] = &['^', 'v', '<', '>', 'A'];
    let mut pad_paths = HashMap::new();

    for &char_start in CHARS {
        for &char_stop in CHARS {
            let position_start = char_to_directional_pad_position(char_start);
            let position_stop = char_to_directional_pad_position(char_stop);
            let (row_start, col_start) = position_start;
            let (row_stop, col_stop) = position_stop;

            let direction_row = if row_start < row_stop { 1 } else { -1 };
            let direction_col = if col_start < col_stop { 1 } else { -1 };
            let letter_row = if direction_row == 1 { 'v' } else { '^' };
            let letter_col = if direction_col == 1 { '>' } else { '<' };

            let mut route_paths = Vec::with_capacity(2);

            // horizontal first path
            if row_start != 0 || col_stop != 0 {
                let mut horizontal_first_path = String::new();
                let mut current_row = row_start;
                let mut current_col = col_start;
                while current_col != col_stop {
                    horizontal_first_path.push(letter_col);
                    current_col = (current_col as i64 + direction_col) as usize;
                }
                while current_row != row_stop {
                    horizontal_first_path.push(letter_row);
                    current_row = (current_row as i64 + direction_row) as usize;
                }
                horizontal_first_path.push('A');
                route_paths.push(horizontal_first_path);
            }
            // vertical first path
            if col_start != 0 || row_stop != 0 {
                let mut vertical_first_path = String::new();
                let mut current_row = row_start;
                let mut current_col = col_start;
                while current_row != row_stop {
                    vertical_first_path.push(letter_row);
                    current_row = (current_row as i64 + direction_row) as usize;
                }
                while current_col != col_stop {
                    vertical_first_path.push(letter_col);
                    current_col = (current_col as i64 + direction_col) as usize;
                }
                vertical_first_path.push('A');
                route_paths.push(vertical_first_path);
            }
            pad_paths.insert((char_start, char_stop), route_paths);
        }
    }
    //println!("{:?}", keypad);
    //println!("{:?}", paths);
    pad_paths
}

fn rec_compute_minimum_pad_sequence(
    sequence_left: &str,
    previous_letter: char,
    current_path: &String,
    results: &mut Vec<String>,
    paths: &PadPaths,
) {
    if sequence_left.is_empty() {
        results.push(current_path.clone());
        return;
    }

    let next_letter = sequence_left.chars().next().unwrap();
    let next_path = &paths[&(previous_letter, next_letter)];
    for path in next_path {
        let mut new_path = current_path.to_string();
        new_path.push_str(path);
        rec_compute_minimum_pad_sequence(
            &sequence_left[1..],
            next_letter,
            &new_path,
            results,
            paths,
        );
    }
}

fn compute_minimum_pad_sequences(sequence: &str, paths: &PadPaths) -> Vec<String> {
    let start_letter: char = 'A';

    let mut results = Vec::new();
    rec_compute_minimum_pad_sequence(sequence, start_letter, &"".to_string(), &mut results, paths);
    results
}

fn parse_input_data(data: &str) -> IResult<&str, Vec<(String, usize)>> {
    separated_list1(
        line_ending,
        map(
            tuple((count(one_of("0123456789"), 3), tag("A"))),
            |(digits, _)| {
                let a = digits[0];
                let b = digits[1];
                let c = digits[2];
                let mut string = String::with_capacity(4);
                string.push(a);
                string.push(b);
                string.push(c);
                string.push('A');
                let value = ((a as u8 - b'0') as usize) * 100
                    + ((b as u8 - b'0') as usize) * 10
                    + ((c as u8 - b'0') as usize);
                (string, value)
            },
        ),
    )(data)
}

// Complete naive solution:

// #[inline]
// fn manhattan_distance(a: (usize, usize), b: (usize, usize)) -> usize {
//     a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
// }

/*fn compute_minimum_keypad_sequence_length(sequence: &str) -> usize {
    //let mut current_position = NUMPAD_A;
    let mut current_letter: char = 'A';

    let paths = build_keypad_paths();

    let mut length = 0;
    for new_letter in sequence.chars() {
        //let new_position = char_to_keypad_position(c);
        //let distance = manhattan_distance(current_position, new_position);
        //length += distance + 1;
        //current_position = new_position;
        let path = &paths[&(current_letter, new_letter)];
        length += path.len();
        current_letter = new_letter;
    }
    length
}*/

#[cached(key = "(String, usize)", convert = "{ (keys.clone(), depth) }")]
fn shortest_sequence(keys: String, depth: usize, paths: &PadPaths) -> usize {
    if depth == 0 {
        return keys.len();
    }

    // split per A, split inclusive is A MUST
    let sub_keys: Vec<&str> = keys.split_inclusive('A').collect();
    let mut total = 0;
    for sub_key in sub_keys {
        let sequences = compute_minimum_pad_sequences(sub_key, paths);
        let mut min_length = usize::MAX;
        for sequence in sequences {
            let length = shortest_sequence(sequence, depth - 1, paths);
            if length < min_length {
                min_length = length;
            }
        }
        total += min_length;
    }
    total
}

fn compute_day_21(data: &str, depth: usize) -> usize {
    let (_, data) = parse_input_data(data).expect("Failed to parse input data");
    let keypad_paths = build_keypad_paths();
    let directional_pad_paths = build_directional_pad_paths();
    data.iter()
        .map(|(sequence, value)| {
            let mut shortest = usize::MAX;
            let keypad_sequences = compute_minimum_pad_sequences(sequence, &keypad_paths);
            for keypad_sequence in keypad_sequences {
                let sequence_length =
                    shortest_sequence(keypad_sequence, depth, &directional_pad_paths);
                if sequence_length < shortest {
                    shortest = sequence_length;
                }
            }
            shortest * *value
        })
        .sum()
}
pub fn day_21_part_1(data: &str) -> usize {
    compute_day_21(data, 2)
}

pub fn day_21_part_2(data: &str) -> usize {
    compute_day_21(data, 25)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "029A
980A
179A
456A
379A";

    /*#[test]
    fn test_compute_minimum_keypad_sequence_length() {
        assert_eq!(compute_minimum_keypad_sequence_length("0"), 2);
        assert_eq!(compute_minimum_keypad_sequence_length("02"), 4);
        assert_eq!(compute_minimum_keypad_sequence_length("029"), 8);
        assert_eq!(compute_minimum_keypad_sequence_length("029A"), 12);
    }*/

    /*#[test]
    fn test_compute_minimum_keypad_sequence() {
        let paths = build_keypad_paths();
        assert_eq!(compute_minimum_pad_sequences("0", &paths), "<A");
        assert_eq!(compute_minimum_pad_sequences("02", &paths), "<A^A");
        assert_eq!(compute_minimum_pad_sequences("029", &paths), "<A^A>^^A");
        assert_eq!(
            compute_minimum_pad_sequences("029A", &paths),
            "<A^A>^^AvvvA"
        );
    }*/

    #[test]
    fn test_compute_minimum_directional_pad_sequence() {
        let paths = build_directional_pad_paths();
        // assert_eq!(compute_minimum_pad_sequence("0", &paths), "<A");
        // assert_eq!(compute_minimum_pad_sequence("02", &paths), "<A^A");
        // assert_eq!(compute_minimum_pad_sequence("029", &paths), "<A^A>^^A");
        /*assert_eq!(
            compute_minimum_pad_sequences("<A^A>^^AvvvA", &paths),
            //"v<<A>>^A<A>AvA<^AA>A<vAAA>^A"
            "v<<A>>^A<A>AvA<^AA>Av<AAA>^A" // small change but should be fine ?
        );
        assert_eq!(
            compute_minimum_pad_sequences("v<<A>>^A<A>AvA<^AA>A<vAAA>^A", &paths),
            //"<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A",
            "v<A<AA>>^AvAA<^A>Av<<A>>^AvA^Av<A>^Av<<A>^A>AAvA^Av<<A>A>^AAAvA<^A>A"
        );*/
        assert_eq!(
            compute_minimum_pad_sequences("<A", &paths),
            vec!["v<<A>>^A"]
        );
    }

    /*#[test]
    fn test_day_21_recursivity() {
        let sequence = "029A";
        let keypad_paths = build_keypad_paths();
        let directional_pad_paths = build_directional_pad_paths();

        let keypad_sequence = compute_minimum_pad_sequences(sequence, &keypad_paths);
        let first_dir_pad_sequence =
            compute_minimum_pad_sequences(&keypad_sequence, &directional_pad_paths);
        let second_dir_pad_sequence =
            compute_minimum_pad_sequences(&first_dir_pad_sequence, &directional_pad_paths);
        println!("second_dir_pad_sequence: {}", second_dir_pad_sequence);
        assert_eq!(
            second_dir_pad_sequence,
            "v<A<AA>>^AvAA<^A>Av<<A>>^AvA^Av<A>^Av<<A>^A>AAvA^Av<A<A>>^AAAvA<^A>A",
            // "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A"
        );
    }*/

    #[test]
    fn test_day_21_part_1() {
        assert_eq!(day_21_part_1(EXAMPLE), 126384);
    }

    #[test]
    fn test_day_21_part_2() {
        assert_eq!(day_21_part_2(EXAMPLE), 154115708116294);
    }
}
