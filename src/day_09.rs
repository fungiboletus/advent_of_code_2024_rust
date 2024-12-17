/*
 Day 09 was a struggle because I'm sick and I have some fever.

 I had a small mistake in part 2, taking the free space with the smallest size that
 can fit the block instead of the free space on the leftest. I had mixed two variables.
 It took a while to identify the bug, and the amount of left out println is an indicator.

 By the way, I prefer println over a debugger for this kind of exercice.

 Also, it seems you do .rev() before and after a .map, it cancels out instead of
 doing the map in reverse and puting the iterator back in the right order after.

 Anyway, I'm happy with the result as it runs on ~~O(N)~~ O(NlogN).

 Switching from VecDeque to BinaryHeap could be tested on very large inputs.

*/

use std::collections::VecDeque;

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

fn find_first_free_space_index(
    current_index: usize,
    required_space: usize,
    free_space_per_size: &[VecDeque<usize>; 9],
) -> Option<usize> {
    if required_space == 0 {
        panic!("We should not have a required space of 0");
    }

    /*println!(
        "current_index: {}, required_space: {}",
        current_index, required_space
    );
    println!("{:?}", free_space_per_size);*/

    // returns the index of the first free space that can hold
    // the required space.
    free_space_per_size
        .iter()
        .enumerate()
        .filter(|(i, v)| i + 1 >= required_space && !v.is_empty())
        .map(|(i, v)| (i, v.front().unwrap()))
        .filter(|(_, index)| **index < current_index)
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(i, _)| i)
}

fn insert_sorted(vec_deque: &mut VecDeque<usize>, n: usize) {
    let pos = vec_deque.binary_search(&n).unwrap_or_else(|e| e);
    vec_deque.insert(pos, n);
}

fn fit_in_new_index(
    current_index: usize,
    required_space: usize,
    free_space_per_size: &mut [VecDeque<usize>; 9],
) -> Option<usize> {
    let first_free_space_index =
        find_first_free_space_index(current_index, required_space, free_space_per_size);
    if let Some(index) = first_free_space_index {
        let free_space_index = free_space_per_size[index].pop_front().unwrap();
        //println!("required_space: {}, index: {}", required_space, index);
        let space_left = index + 1 - required_space;
        if space_left > 0 {
            let space_left_size_index = space_left - 1;
            assert!(space_left_size_index < 10); // just in case, I have fever
            let left_space_left_start_index = free_space_index + required_space;
            // Insert the new free space in the list
            insert_sorted(
                &mut free_space_per_size[space_left_size_index],
                left_space_left_start_index,
            );
        }
        Some(free_space_index)
    } else {
        None
    }
}

pub fn day_09_part_2(data: &str) -> i64 {
    let (_, numbers) = parse_input_data(data).expect("Failed to parse input data");
    //println!("len: {}", numbers.len());

    // VecDeque<index_start>
    let mut free_space_per_size: [VecDeque<usize>; 9] = Default::default();
    // Vec<(id, index_start, size)>
    let mut used_blocs: Vec<(usize, usize, usize)> = Vec::with_capacity((numbers.len() + 1) / 2);
    let mut index = 0_usize;

    for (id, chunk) in numbers.chunks(2).enumerate() {
        let files_size = chunk[0] as usize;
        let free_space_size = *chunk.get(1).unwrap_or(&0) as usize;

        used_blocs.push((id, index, files_size));
        index += files_size;

        if free_space_size > 0 {
            assert!(free_space_size <= 9);
            free_space_per_size[free_space_size - 1].push_back(index);
            index += free_space_size;
        }
    }

    assert_eq!(used_blocs.len(), (numbers.len() + 1) / 2);

    //println!("{:?}", used_blocs);
    //println!("{:?}", free_space_per_size);

    let new_used_blocks: Vec<(usize, usize, usize)> = used_blocs
        .iter()
        .rev()
        .map(|block| {
            //println!("block: {:?}", block);
            //let new_index = find_first_free_space_index(*size, &free_space_per_size);
            //println!("free_space_per_size: {:?}", free_space_per_size);
            //println!("used_blocks: {:?}", new_used_blocks);
            let (id, current_index, size) = block;
            if let Some(new_index) =
                fit_in_new_index(*current_index, *size, &mut free_space_per_size)
            {
                //println!("new_index: {:?}", new_index);
                (*id, new_index, *size)
            } else {
                // We didn't move the block
                *block
            }
        })
        .collect();

    //println!("{:?}", new_used_blocks);

    let checksum = new_used_blocks
        .iter()
        //.map(|(id, index, size)| id * (*index..(*index + *size)).sum::<usize>())
        .map(|(id, index, size)| id * *size * (2 * *index + *size - 1) / 2)
        .sum::<usize>();

    //println!("{}", checksum);

    checksum as i64
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
        assert_eq!(day_09_part_2(EXAMPLE_SMALL), 132);
        assert_eq!(day_09_part_2(EXAMPLE_BIG), 2858);
        // test cases found on r/adventofcode
        assert_eq!(day_09_part_2("14113"), 16); // works
        assert_eq!(day_09_part_2("1010101010101010101010"), 385); // works
        assert_eq!(day_09_part_2("354631466260"), 1325); // works
        assert_eq!(day_09_part_2("252"), 5); // works
        assert_eq!(day_09_part_2("171010402"), 88); // works
        assert_eq!(day_09_part_2("597689906"), 1840); // dosen't work \o/
    }
}
