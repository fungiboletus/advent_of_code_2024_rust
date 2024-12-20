/*
    Part 1 was done with regex, for fun. I tried to do it with nom,
    and I wasted quite some time there, but I realised that it wasn't the right tool for the job.

    Part 2 required to go away with the regex and do the boring recursive exploration.
    Relatively straightforward, but it requires memoization.

    I found an optimisation I enjoyed on r/adventofcode, though the code is very verbose
    and it's not much faster.
*/

use cached::proc_macro::cached;
use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, one_of},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::tuple,
    IResult,
};
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

type Pattern = Vec<char>;

fn parse_pattern(data: &str) -> IResult<&str, Pattern> {
    many1(one_of("wubrg"))(data)
}

fn parse_patterns(data: &str) -> IResult<&str, Vec<Pattern>> {
    separated_list1(tag(", "), parse_pattern)(data)
}

fn parse_input_data(data: &str) -> IResult<&str, (Vec<Pattern>, Vec<Pattern>)> {
    map(
        tuple((
            parse_patterns,
            line_ending,
            line_ending,
            separated_list1(line_ending, parse_pattern),
        )),
        |(rules, _, _, patterns)| (rules, patterns),
    )(data)
}

pub fn day_19_part_1(data: &str) -> usize {
    let (_, (patterns, designs)) = parse_input_data(data).expect("Failed to parse input data");

    let mut regex_string = String::new();
    regex_string.push_str("^(");

    for pattern in patterns {
        for c in pattern {
            regex_string.push(c);
        }
        regex_string.push('|');
    }
    // Remove the last '|'
    regex_string.pop();
    regex_string.push_str(")*$");

    let regex = regex::Regex::new(&regex_string).expect("Failed to create regex");

    designs
        .par_iter()
        .filter(|d| {
            let string = d.iter().collect::<String>();
            regex.is_match(&string)
        })
        .count()
}

#[cached(key = "String", convert = r#"{ design.clone() }"#)]
fn count_number_of_possibilities(design: String, patterns: &[String]) -> usize {
    if design.is_empty() {
        return 1;
    }

    let mut count = 0;
    for pattern in patterns {
        if design.starts_with(pattern) {
            count += count_number_of_possibilities(design[pattern.len()..].to_string(), patterns);
        }
    }

    count
}

pub fn day_19_part_2(data: &str) -> usize {
    let (_, (patterns, designs)) = parse_input_data(data).expect("Failed to parse input data");

    // we prefer strings to vectors of chars
    let patterns = patterns
        .iter()
        .map(|p| p.iter().collect::<String>())
        .collect::<Vec<String>>();

    // simple version
    /*designs
    .iter()
    .map(|d| d.iter().collect::<String>())
    .map(|d| count_number_of_possibilities(d, &patterns))
    .sum()*/

    // found an optimisation that is pretty verbose but I like it
    // https://www.reddit.com/r/adventofcode/comments/1hhtrgj/comment/m2u6ubd/
    // Idea is to work on a sliding window of 8 characters

    let patterns_set = patterns.iter().collect::<std::collections::HashSet<_>>();
    designs
        .par_iter()
        .map(|d| {
            let len = d.len();
            let mut count = vec![0; len];

            for i in 1..=8 {
                if i > len {
                    break;
                }
                let string = d[0..i].iter().collect::<String>();
                count[i - 1] = count_number_of_possibilities(string.clone(), &patterns);
            }

            for i in 8..len {
                let mut count_at_i = 0;

                for j in 0..8 {
                    if patterns_set.contains(&d[i - j..i + 1].iter().collect::<String>()) {
                        count_at_i += count[i - j - 1];
                    }
                }

                count[i] = count_at_i;
            }

            count[len - 1]
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test_day_19_part_1() {
        assert_eq!(day_19_part_1(EXAMPLE), 6);
    }

    #[test]
    fn test_day_19_part_2() {
        assert_eq!(day_19_part_2(EXAMPLE), 16);
    }
}
