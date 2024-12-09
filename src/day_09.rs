/*
    Comments.
*/

use nom::{
    character::{complete::satisfy, is_digit},
    combinator::map,
    multi::many1,
    IResult,
};

fn parse_input_data(data: &str) -> IResult<&str, Vec<u8>> {
    many1(map(satisfy(|c| is_digit(c as u8)), |c| c as u8 - b'0'))(data)
}

pub fn day_09_part_1(data: &str) -> i64 {
    let (_, numbers) = parse_input_data(data).expect("Failed to parse input data");
    //println!("{:?}", numbers);

    let sum = numbers.iter().map(|n| *n as usize).sum::<usize>();
    let mut memory: Vec<Option<usize>> = Vec::with_capacity(sum);
    let mut final_size = 0;

    // simple fill
    for (i, chunk) in numbers.chunks(2).enumerate() {
        let files_size = chunk[0] as usize;
        let free_space_size = *chunk.get(1).unwrap_or(&0) as usize;

        memory.extend(std::iter::repeat(Some(i)).take(files_size));
        memory.extend(std::iter::repeat(None).take(free_space_size));

        final_size += files_size;
    }
    assert_eq!(memory.len(), sum);
    //println!("{:?}", memory);

    let mut i = 0;
    let mut end = memory.len() - 1;
    while i < end {
        if memory[i].is_none() {
            while memory[end].is_none() {
                end -= 1;
                if end == i {
                    break;
                }
            }
            memory[i] = memory[end];
            memory[end] = None;
            //memory.swap(i, end);
            end -= 1;
        }
        i += 1;
    }
    // Ditch the None at the end of the memory
    memory.resize(final_size, None);

    // compute weird checksum
    memory
        .iter()
        .flatten()
        .enumerate()
        .map(|(i, v)| i * v)
        .sum::<usize>() as i64
}

pub fn day_09_part_2(data: &str) -> i64 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_SMALL: &str = "12345";
    const EXAMPLE_BIG: &str = "2333133121414131402";

    #[test]
    fn test_day_09_part_1() {
        assert_eq!(day_09_part_1(EXAMPLE_SMALL), 60);
        assert_eq!(day_09_part_1(EXAMPLE_BIG), 1928);
    }

    #[test]
    fn test_day_09_part_2() {
        assert_eq!(day_09_part_2(EXAMPLE_SMALL), 42);
    }
}
