/*
    Comments.
*/

use std::{
    collections::HashMap,
    sync::{Arc, Mutex, RwLock},
};

use nom::{character::complete::line_ending, multi::separated_list0, IResult};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

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
    static MASK: u32 = 0b11111;
    ((((a + 9) as u32) & MASK) << 15)
        | ((((b + 9) as u32) & MASK) << 10)
        | ((((c + 9) as u32) & MASK) << 5)
        | (((d + 9) as u32) & MASK)
}

#[inline]
fn last_digit_base_10(number: u32) -> i8 {
    (number % 10) as i8
}

pub fn day_22_part_2(data: &str) -> i64 {
    let (_, data) = parse_input_data(data).expect("Failed to parse input data");

    //let mut sums: HashMap<u32, i64> = HashMap::new();
    //let mut highest_sum: i64 = 0;

    // make it thread safe
    //let sums: Arc<RwLock<HashMap<u32, i64>>> = Arc::new(RwLock::new(HashMap::new()));
    let sums: Arc<RwLock<Vec<i64>>> = Arc::new(RwLock::new(vec![0; 1 << 20]));
    let highest_sum: Arc<RwLock<i64>> = Arc::new(RwLock::new(0));

    data.par_iter().for_each(|seed| {
        let sums = Arc::clone(&sums);
        let highest_sum = Arc::clone(&highest_sum);
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

        let max_iter = 2000 - 3;

        let mut map: HashMap<u32, i8> = HashMap::new();
        //let mut sums: Vec<i8> = vec![0; 1 << 20];
        //let mut map: Vec<Option<i8>> = vec![None; 1 << 20];

        for i in 0..max_iter {
            let next = compute_next_secret(secret_g0);
            let last_digit_next = last_digit_base_10(next);
            let diff_0_to_next = last_digit_next - last_digit_g0;

            // let diff_2_to_1 = last_digit_base_10(secret_g1) - last_digit_base_10(secret_g2);
            // let diff_1_to_0 = last_digit_base_10(secret_g0) - last_digit_base_10(secret_g1);
            // let diff_0_to_next = last_digit_base_10(next) - last_digit_base_10(secret_g0);

            //println!(
            //    "{}: {} {} {} {} {}",
            //    i, next, secret_g0, secret_g1, secret_g2, secret_g3
            //);
            //println!(
            //    "{}: {} {} {} {}",
            //    i, diff_3_to_2, diff_2_to_1, diff_1_to_0, diff_0_to_next
            //);
            let number = sequence_to_number(diff_3_to_2, diff_2_to_1, diff_1_to_0, diff_0_to_next);

            // unsert if not exist in one line
            //map.entry(number).or_insert(last_digit_next);
            if let std::collections::hash_map::Entry::Vacant(e) = map.entry(number) {
                e.insert(last_digit_next);
                let new_sum_b: i64;
                {
                    let mut sums = sums.write().unwrap();
                    let new_sum = sums[number as usize] + last_digit_next as i64;
                    sums[number as usize] = new_sum;
                    new_sum_b = new_sum;
                }
                let mut should_update = false;
                {
                    let highest_sum_read = highest_sum.read().unwrap();
                    if new_sum_b > *highest_sum_read {
                        should_update = true;
                    }
                }
                if should_update {
                    let mut highest_sum = highest_sum.write().unwrap();
                    *highest_sum = new_sum_b;
                }

                /*sums.entry(number)
                .and_modify(|sum| {
                    let new_sum = *sum + last_digit_next as i64;
                    let mut should_update = false;
                    {
                        let highest_sum_read = highest_sum.read().unwrap();
                        if new_sum > *highest_sum_read {
                            should_update = true;
                        }
                    }
                    if should_update {
                        let mut highest_sum = highest_sum.write().unwrap();
                        *highest_sum = new_sum;
                    }
                    *sum = new_sum;
                })
                .or_insert(last_digit_next as i64);*/
            }
            /*if map[number as usize].is_none() {
                map[number as usize] = Some(last_digit_next);
            }*/

            //secret_g3 = secret_g2;
            //secret_g2 = secret_g1;
            //secret_g1 = secret_g0;
            secret_g0 = next;

            //last_digit_g3 = last_digit_g2;
            //last_digit_g2 = last_digit_g1;
            //last_digit_g1 = last_digit_g0;
            last_digit_g0 = last_digit_next;

            diff_3_to_2 = diff_2_to_1;
            diff_2_to_1 = diff_1_to_0;
            diff_1_to_0 = diff_0_to_next;

            //println!("{}: {}", i, number);
        }
        //println!("debug: {}", sequence_to_number(9, 9, 9, 9));
        //println!("map: {:?}", map);
    });

    //println!("sums: {:?}", sums);
    //println!("highest_sum: {}", highest_sum);
    //highest_sum.lock().unwrap().clone()
    let highest_sum = highest_sum.read().unwrap();
    *highest_sum
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
