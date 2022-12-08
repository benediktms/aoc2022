use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use itertools::Itertools;

fn create_strings_from_range(tuple: (&str, &str)) -> (String, String) {
    let range = tuple.0.split("-").collect_tuple::<(&str, &str)>().unwrap();
    let start = range.0.parse::<i32>().unwrap();
    let end = range.1.parse::<i32>().unwrap();
    let range_one = start..=end;

    let range_one_str = range_one.map(|num| num.to_string()).collect_vec().join(",");
    println!("range one {:?}", range_one_str);

    let range = tuple.1.split("-").collect_tuple::<(&str, &str)>().unwrap();
    let start = range.0.parse::<i32>().unwrap();
    let end = range.1.parse::<i32>().unwrap();
    let range = start..=end;

    let range_two_str = range.map(|num| num.to_string()).collect_vec().join(",");
    println!("range two {:?}", range_two_str);

    (range_one_str, range_two_str)
}

fn count_subvectors(raw_string: &str) -> i32 {
    let pairs = raw_string
        .split(",")
        .collect_tuple::<(&str, &str)>()
        .unwrap();

    let (range_one, range_two) = create_strings_from_range(pairs);

    let one_contains_two = range_one.contains(range_two.as_str());
    println!("one contains two {:?}", one_contains_two);
    let two_contains_one = range_two.contains(range_one.as_str());
    println!("two contains one {:?}", two_contains_one);

    let mut sum = 0;

    if one_contains_two || two_contains_one {
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

    assert_eq!(true, sum < 490);
    println!("sum: {}", sum);

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
