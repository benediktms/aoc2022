#![feature(iter_next_chunk)]

use std::collections::{HashSet, VecDeque};

use anyhow::Context;
use util::read_file;

// https://www.youtube.com/watch?v=LKncwDMrQOg&ab_channel=RitualCoding
fn get_signal_marker_index(input: &str, increase_buffer: bool) -> i32 {
    let mut stream = input.chars();

    // start the index at either 4 or 14 because the first 4 elements have already been loaded
    let max_buffer_size = if increase_buffer { 14 } else { 4 };
    let mut marker_index = max_buffer_size;

    // use deque as a sliding window along the stream
    let mut buf = VecDeque::with_capacity(marker_index);
    // initialize the deque with the first 4 elements
    if increase_buffer {
        buf.extend(stream.next_chunk::<14>().unwrap().iter());
    } else {
        buf.extend(stream.next_chunk::<4>().unwrap().iter());
    }

    // run through the remainder of the stream
    for (idx, val) in stream.enumerate() {
        let store: HashSet<char> = HashSet::from_iter(buf.iter().cloned());

        // check for 4 unique characters in the current buffer window
        if store.len() == max_buffer_size {
            marker_index += idx;
            break;
        }

        // move buffer window along the stream
        buf.pop_front();
        buf.push_back(val);
    }

    marker_index as i32
}

fn main() -> anyhow::Result<()> {
    let input = read_file("input/day6/input.txt")
        .last()
        .context("Could not get string from input")
        .unwrap()
        .unwrap();

    let idx = get_signal_marker_index(&input, false);
    let idx_for_msg = get_signal_marker_index(&input, true);

    println!("marker index: {idx}");
    println!("marker message: {idx_for_msg}");
    assert_eq!(1794, idx);

    Ok(())
}

#[cfg(test)]
mod test {
    use util::read_file;

    use crate::get_signal_marker_index;

    #[test]
    fn should_get_correct_marker_for_file_1() -> anyhow::Result<()> {
        let input = read_file("input/day6/test1.txt").last().unwrap().unwrap();

        let index = get_signal_marker_index(&input, false);
        let idx_for_msg = get_signal_marker_index(&input, true);

        assert_eq!(7, index);
        assert_eq!(19, idx_for_msg);

        Ok(())
    }

    #[test]
    fn should_get_correct_marker_for_file_2() -> anyhow::Result<()> {
        let input = read_file("input/day6/test2.txt").last().unwrap().unwrap();

        let index = get_signal_marker_index(&input, false);
        let idx_for_msg = get_signal_marker_index(&input, true);

        assert_eq!(5, index);
        assert_eq!(23, idx_for_msg);

        Ok(())
    }

    #[test]
    fn should_get_correct_marker_for_file_3() -> anyhow::Result<()> {
        let input = read_file("input/day6/test3.txt").last().unwrap().unwrap();

        let index = get_signal_marker_index(&input, false);
        let idx_for_msg = get_signal_marker_index(&input, true);

        assert_eq!(6, index);
        assert_eq!(23, idx_for_msg);

        Ok(())
    }

    #[test]
    fn should_get_correct_marker_for_file_4() -> anyhow::Result<()> {
        let input = read_file("input/day6/test4.txt").last().unwrap().unwrap();

        let index = get_signal_marker_index(&input, false);
        let idx_for_msg = get_signal_marker_index(&input, true);

        assert_eq!(10, index);
        assert_eq!(29, idx_for_msg);

        Ok(())
    }

    #[test]
    fn should_get_correct_marker_for_file_5() -> anyhow::Result<()> {
        let input = read_file("input/day6/test5.txt").last().unwrap().unwrap();

        let index = get_signal_marker_index(&input, false);
        let idx_for_msg = get_signal_marker_index(&input, true);

        assert_eq!(11, index);
        assert_eq!(26, idx_for_msg);

        Ok(())
    }
}
