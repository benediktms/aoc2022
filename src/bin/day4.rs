use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use itertools::Itertools;

fn create_vectors(tuple: (&str, &str)) -> (Vec<i32>, Vec<i32>) {
    let range = tuple.0.split("-").collect_tuple::<(&str, &str)>().unwrap();
    let start = range.0.parse::<i32>().unwrap();
    let end = range.1.parse::<i32>().unwrap();
    let range = start..=end;

    let vector_one = range.collect_vec();

    let range = tuple.1.split("-").collect_tuple::<(&str, &str)>().unwrap();
    let start = range.0.parse::<i32>().unwrap();
    let end = range.1.parse::<i32>().unwrap();
    let range = start..=end;

    let vector_two = range.collect_vec();

    (vector_one, vector_two)
}

// https://stackoverflow.com/questions/47043167/does-rust-contain-a-way-to-directly-check-whether-or-not-one-vector-is-a-substr
fn is_sub<T: PartialEq>(mut haystack: &[T], needle: &[T]) -> bool {
    if needle.len() == 0 {
        return true;
    }
    while !haystack.is_empty() {
        if haystack.starts_with(needle) {
            return true;
        }
        haystack = &haystack[1..];
    }
    false
}

fn count_subvectors(raw_string: &str) -> i32 {
    let mut sum = 0;

    let pairs = raw_string
        .split(",")
        .collect_tuple::<(&str, &str)>()
        .unwrap();

    let (vec_one, vec_two) = create_vectors(pairs);

    let (outer, inner) = if vec_one.len() > vec_two.len() {
        (vec_one, vec_two)
    } else {
        (vec_two, vec_one)
    };

    if is_sub(&outer, &inner) {
        sum += 1;
    }

    sum
}

fn main() -> anyhow::Result<()> {
    let path = Path::new("input/day4/input.txt");
    let file = File::open(path)?;

    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines();
    let mut sum = 0;

    for line in lines {
        let line = line?;

        sum += count_subvectors(&line);
    }

    println!("sum: {}", sum);
    assert_eq!(462, sum);

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
        path::Path,
    };

    use crate::count_subvectors;

    #[test]
    fn should_correctly_identify_fully_contained_ranges() -> anyhow::Result<()> {
        let path = Path::new("input/day4/test.txt");
        let file = File::open(path)?;

        let buf_reader = BufReader::new(file);
        let lines = buf_reader.lines();
        let mut sum = 0;

        for line in lines {
            let line = line?;

            sum += count_subvectors(&line);
        }

        assert_eq!(2, sum);

        Ok(())
    }
}
