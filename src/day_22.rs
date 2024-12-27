/*
    A pretty nice day.

    Part 1 is mostly about doing the right operations, which was relatively straightforward.
    I noticed that the operations were obvious bitwise operations.

    I tried to think about a way to optimise, but this looked like a very much non-optimisable
    problem. The problem was a classic hashing/psuedo-random number generation despite
    the very simple operations.

    Part 2 was a bit more work, and my first implementation was pretty slow.

    To optimise, I encoded the sequences to a number. I first used a 20 bits long number,
    using bitwise operators. The maximum range was 19 values, from -9 to 9 which required 5 bits each.
    So 20 bits total. It made arrays of 1 048 576 elements, which was a bit too much.
    I still used it but the HashMap were faster than arrays despite using a somewhat low number.

    I found on reddit later than using multiplications to not waste bits was much better,
    as it needed 17 bits, with a max value of 130 321. Using this encoding,
    an array was faster than a HashMap.

    Then I tried to make it parallel with rayon, but it ran much slower due to the need
    of mutexes to update the array of sums and the highest sum. As I was computing the highest
    sum on the fly. Using rayon, but with batches made the performances much faster,
    despite having to merge the arrays of sums and finding the highest sum at the end.

    I also noticed than a = vec![None, a.len()] could be faster than a a.fill(None).
*/

use nom::{character::complete::line_ending, multi::separated_list0, IResult};
use rayon::{
    iter::{IntoParallelRefIterator, ParallelIterator},
    slice::ParallelSlice,
};

#[inline]
fn mix(a: u32, b: u32) -> u32 {
    a ^ b
}

#[inline]
fn prune(number: u32) -> u32 {
    number & 0b111111111111111111111111
}

#[inline]
fn multiply_by_64(number: u32) -> u32 {
    number << 6
}

#[inline]
fn divide_by_32(number: u32) -> u32 {
    number >> 5
}

#[inline]
fn multiply_by_2048(number: u32) -> u32 {
    number << 11
}

fn compute_next_secret(secret: u32) -> u32 {
    let mut next = prune(mix(multiply_by_64(secret), secret));
    next = prune(mix(divide_by_32(next), next));
    next = prune(mix(multiply_by_2048(next), next));
    next
}

fn parse_input_data(data: &str) -> IResult<&str, Vec<u32>> {
    separated_list0(line_ending, nom::character::complete::u32)(data)
}

pub fn day_22_part_1(data: &str) -> i64 {
    let (_, data) = parse_input_data(data).expect("Failed to parse input data");

    data.par_iter()
        .map(|seed| {
            let mut secret = *seed;
            for _ in 0..2000 {
                secret = compute_next_secret(secret);
            }
            secret as i64
        })
        .sum()
}

#[inline]
fn sequence_to_number(a: i8, b: i8, c: i8, d: i8) -> u32 {
    /*static MASK: u32 = 0b11111;
    ((((a + 9) as u32) & MASK) << 15)
        | ((((b + 9) as u32) & MASK) << 10)
        | ((((c + 9) as u32) & MASK) << 5)
        | (((d + 9) as u32) & MASK)*/
    6859 * ((a + 9) as u32) + 361 * ((b + 9) as u32) + 19 * ((c + 9) as u32) + ((d + 9) as u32)
}

#[inline]
fn last_digit_base_10(number: u32) -> i8 {
    (number % 10) as i8
}

pub fn day_22_part_2(data: &str) -> i64 {
    let (_, data) = parse_input_data(data).expect("Failed to parse input data");

    let max_iter = 2000 - 3;
    let chunk_size = 64;
    //let array_size = 1 << 20;
    let array_size = 130321;

    *data
        .par_chunks(chunk_size)
        .map(|chunk| {
            let mut sums: Vec<u16> = vec![0; array_size];
            let mut map: Vec<Option<i8>> = vec![None; 0];

            chunk.iter().for_each(|seed| {
                map = vec![None; array_size];
                let secret_g3 = *seed;
                let secret_g2 = compute_next_secret(secret_g3);
                let secret_g1 = compute_next_secret(secret_g2);
                let mut secret_g0 = compute_next_secret(secret_g1);

                let last_digit_g3 = last_digit_base_10(secret_g3);
                let last_digit_g2 = last_digit_base_10(secret_g2);
                let last_digit_g1 = last_digit_base_10(secret_g1);
                let mut last_digit_g0 = last_digit_base_10(secret_g0);

                let mut diff_3_to_2 = last_digit_g2 - last_digit_g3;
                let mut diff_2_to_1 = last_digit_g1 - last_digit_g2;
                let mut diff_1_to_0 = last_digit_g0 - last_digit_g1;

                for _ in 0..max_iter {
                    let next = compute_next_secret(secret_g0);
                    let last_digit_next = last_digit_base_10(next);
                    let diff_0_to_next = last_digit_next - last_digit_g0;

                    let number =
                        sequence_to_number(diff_3_to_2, diff_2_to_1, diff_1_to_0, diff_0_to_next);

                    if map[number as usize].is_none() {
                        map[number as usize] = Some(last_digit_next);
                        let new_sum = sums[number as usize] + last_digit_next as u16;
                        sums[number as usize] = new_sum;
                    }

                    secret_g0 = next;

                    last_digit_g0 = last_digit_next;

                    diff_3_to_2 = diff_2_to_1;
                    diff_2_to_1 = diff_1_to_0;
                    diff_1_to_0 = diff_0_to_next;
                }
            });

            sums
        })
        .reduce(
            || vec![0; array_size],
            |mut acc, vec| {
                for (i, value) in vec.iter().enumerate() {
                    acc[i] += value;
                }
                acc
            },
        )
        .par_iter()
        .max()
        .unwrap_or(&0) as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PART_1: &str = "1
10
100
2024";

    const EXAMPLE_PART_2: &str = "1
2
3
2024";

    #[test]
    fn test_day_22_mix() {
        assert_eq!(mix(42, 15), 37);
    }

    #[test]
    fn test_day_22_prune() {
        assert_eq!(prune(100000000), 16113920);
    }

    #[test]
    fn test_day_22_sequence_to_index() {
        assert_eq!(sequence_to_number(9, 9, 9, 9), 130320);
    }

    #[test]
    fn test_day_22_compute_next_secret() {
        assert_eq!(compute_next_secret(123), 15887950);
        assert_eq!(compute_next_secret(15887950), 16495136);
        assert_eq!(compute_next_secret(16495136), 527345);
        assert_eq!(compute_next_secret(527345), 704524);
        assert_eq!(compute_next_secret(704524), 1553684);
        assert_eq!(compute_next_secret(1553684), 12683156);
        assert_eq!(compute_next_secret(12683156), 11100544);
        assert_eq!(compute_next_secret(11100544), 12249484);
        assert_eq!(compute_next_secret(12249484), 7753432);
        assert_eq!(compute_next_secret(7753432), 5908254);
    }

    #[test]
    fn test_day_22_part_1() {
        assert_eq!(day_22_part_1(EXAMPLE_PART_1), 37327623);
    }

    #[test]
    fn test_day_22_part_2() {
        assert_eq!(day_22_part_2(EXAMPLE_PART_2), 23);
    }
}
