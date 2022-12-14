#![feature(iter_next_chunk)]

use std::collections::{HashSet, VecDeque};

use anyhow::Context;
use util::read_file;

fn get_signal_marker_index(input: String) -> i32 {
    let mut stream = input.chars();

    // use deque as a sliding window along the stream
    let mut buf = VecDeque::with_capacity(4);
    // initialize the deque with the first 4 elements
    buf.extend(stream.next_chunk::<4>().unwrap().iter());

    // start the index at 4 because the first 4 elements have already been loaded
    let mut marker_index = 4;

    // run through the remainder of the stream
    for (idx, val) in stream.enumerate() {
        let store: HashSet<char> = HashSet::from_iter(buf.iter().cloned());

        // check for 4 unique characters in the current buffer window
        if store.len() == 4 {
            marker_index += idx as i32;
            break;
        }

        // move buffer window along the stream
        buf.pop_front();
        buf.push_back(val);
    }

    marker_index
}

fn main() -> anyhow::Result<()> {
    let input = read_file("input/day6/input.txt")
        .last()
        .context("Could not get string from input")
        .unwrap()
        .unwrap();

    let idx = get_signal_marker_index(input);

    println!("marker index: {idx}");
    assert_eq!(1794, idx);

    Ok(())
}

#[cfg(test)]
mod test {
    use anyhow::Context;
    use util::read_file;

    use crate::get_signal_marker_index;

    #[test]
    fn should_get_correct_marker_for_file_1() -> anyhow::Result<()> {
        let input = read_file("input/day6/test1.txt");
        let mut index = 0;

        for line in input {
            let line = line.context("Could not read line")?;
            index = get_signal_marker_index(line);
        }

        assert_eq!(7, index);

        Ok(())
    }

    #[test]
    fn should_get_correct_marker_for_file_2() -> anyhow::Result<()> {
        let input = read_file("input/day6/test2.txt");
        let mut index = 0;

        for line in input {
            let line = line.context("Could not read line")?;
            index = get_signal_marker_index(line);
        }

        assert_eq!(5, index);

        Ok(())
    }

    #[test]
    fn should_get_correct_marker_for_file_3() -> anyhow::Result<()> {
        let input = read_file("input/day6/test3.txt");
        let mut index = 0;

        for line in input {
            let line = line.context("Could not read line")?;
            index = get_signal_marker_index(line);
        }

        assert_eq!(6, index);

        Ok(())
    }

    #[test]
    fn should_get_correct_marker_for_file_4() -> anyhow::Result<()> {
        let input = read_file("input/day6/test4.txt");
        let mut index = 0;

        for line in input {
            let line = line.context("Could not read line")?;
            index = get_signal_marker_index(line);
        }

        assert_eq!(10, index);

        Ok(())
    }

    #[test]
    fn should_get_correct_marker_for_file_5() -> anyhow::Result<()> {
        let input = read_file("input/day6/test5.txt");
        let mut index = 0;

        for line in input {
            let line = line.context("Could not read line")?;
            index = get_signal_marker_index(line);
        }

        assert_eq!(11, index);

        Ok(())
    }
}
