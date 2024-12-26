/*
    Comments.
*/

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, one_of},
    combinator::map,
    multi::{count, separated_list0, separated_list1},
    sequence::tuple,
    IResult,
};

type Key = [u8; 5];
type Lock = [u8; 5];

#[derive(Debug)]
struct Problem {
    keys: Vec<Key>,
    locks: Vec<Lock>,
}

fn parse_key(input: &str) -> IResult<&str, Key> {
    map(
        tuple((
            tag("....."),
            line_ending,
            separated_list1(line_ending, count(one_of(".#"), 5)),
        )),
        |(_, _, key_pattern)| {
            let mut key = [0; 5];

            if key_pattern.len() <= 1 {
                return key;
            }

            assert_eq!(key_pattern.len(), 6);

            for line in &key_pattern[..5] {
                for (i, c) in line.iter().enumerate() {
                    if *c == '#' {
                        key[i] += 1;
                    }
                }
            }

            key
        },
    )(input)
}

fn parse_lock(input: &str) -> IResult<&str, Lock> {
    map(
        tuple((
            tag("#####"),
            line_ending,
            separated_list1(line_ending, count(one_of(".#"), 5)),
        )),
        |(_, _, lock_pattern)| {
            let mut lock = [0; 5];

            assert_eq!(lock_pattern.len(), 6);

            for line in &lock_pattern[..5] {
                for (i, c) in line.iter().enumerate() {
                    if *c == '#' {
                        lock[i] += 1;
                    }
                }
            }

            lock
        },
    )(input)
}

fn parse_input_data(input: &str) -> IResult<&str, Problem> {
    map(
        separated_list0(
            tuple((line_ending, line_ending)),
            alt((
                map(parse_key, |key| (0, key)),
                map(parse_lock, |lock| (1, lock)),
            )),
        ),
        |data| {
            let mut keys = Vec::new();
            let mut locks = Vec::new();

            for item in data {
                match item {
                    (0, key) => keys.push(key),
                    (1, lock) => locks.push(lock),
                    _ => panic!("Failed to parse input data"),
                }
            }

            Problem { keys, locks }
        },
    )(input)
}

pub fn day_25_part_1(data: &str) -> usize {
    let (_, problem) = parse_input_data(data).expect("Failed to parse input data");

    // Very simple algorithm, but the number of keys and locks is small.
    problem
        .keys
        .iter()
        .map(|key| {
            problem
                .locks
                .iter()
                .filter(|lock| {
                    lock[0] + key[0] <= 5
                        && lock[1] + key[1] <= 5
                        && lock[2] + key[2] <= 5
                        && lock[3] + key[3] <= 5
                        && lock[4] + key[4] <= 5
                })
                .count()
        })
        .sum()
}

pub fn day_25_part_2(data: &str) -> i64 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

    #[test]
    fn test_day_25_utilities() {
        let lock: Lock = [0, 5, 3, 4, 3];
        let key: Key = [5, 0, 2, 1, 3];
    }

    #[test]
    fn test_day_25_part_1() {
        assert_eq!(day_25_part_1(EXAMPLE), 3);
    }

    #[test]
    fn test_day_25_part_2() {
        assert_eq!(day_25_part_2(EXAMPLE), 42);
    }
}
