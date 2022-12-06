use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn get_item_priorities(backpack: (&str, &str)) -> i32 {
    let lowercase_range = b'a'..=b'z';
    let uppercase_range = b'A'..=b'Z';

    let lowercase_vec: Vec<char> = lowercase_range
        .map(|i| (i as char).to_ascii_lowercase())
        .collect();

    let uppercase_vec: Vec<char> = uppercase_range
        .map(|i| (i as char).to_ascii_uppercase())
        .collect();

    let first_compartment: Vec<char> = backpack.0.chars().collect();
    let second_compartment: Vec<char> = backpack.1.chars().collect();
    let mut is_duplicate = false;

    let mut sum = 0;

    for outer_item in &first_compartment {
        for inner_item in &second_compartment {
            is_duplicate = outer_item == inner_item;

            if is_duplicate {
                let index = lowercase_vec.iter().position(|char| char == inner_item);
                match index {
                    Some(value) => {
                        // account for 0th index
                        let priority = value + 1;
                        sum += priority as i32
                    }
                    None => {
                        let index = lowercase_vec.iter().position(|char| char == inner_item);
                        match index {
                            Some(value) => {
                                // account for 0th index
                                let priority = (value as i32) + 1;
                                sum += priority
                            }
                            None => sum += 0,
                        };
                    }
                };

                let index = uppercase_vec.iter().position(|char| char == inner_item);
                match index {
                    Some(value) => {
                        // account for the first 26 characters from the lowercase verion and 0th index
                        let priority = value + 1 + 26;
                        sum += priority as i32
                    }
                    None => {
                        let index = uppercase_vec.iter().position(|char| char == inner_item);
                        match index {
                            Some(value) => {
                                // account for the first 26 characters from the lowercase verion and 0th index
                                let priority = (value as i32) + 26 + 1;
                                sum += priority
                            }
                            None => sum += 0,
                        };
                    }
                };

                break;
            }
        }

        if is_duplicate {
            break;
        }
    }

    sum
}

fn main() -> anyhow::Result<()> {
    let path = Path::new("input/day3/input1.txt");
    let file = File::open(path)?;

    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines();
    let mut sum = 0;

    for line in lines {
        let line = line?;
        let split: Vec<&str> = line.split(|i: char| i.is_ascii_alphabetic()).collect();

        let backpack_compartments = line.split_at((split.len() - 1) / 2);

        sum += get_item_priorities(backpack_compartments);
    }

    println!("sum: {}", sum);
    assert_eq!(8123, sum);

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
        path::Path,
    };

    use crate::get_item_priorities;

    #[test]
    fn should_cacluate_the_correct_value_from_test_input() -> anyhow::Result<()> {
        let path = Path::new("input/day3/test.txt");
        let file = File::open(path)?;

        let buf_reader = BufReader::new(file);
        let lines = buf_reader.lines();
        let mut sum = 0;

        for line in lines {
            let line = line?;
            let split: Vec<&str> = line.split(|i: char| i.is_ascii_alphabetic()).collect();

            let backpack = line.split_at((split.len() - 1) / 2);

            sum += get_item_priorities(backpack);
        }

        println!("score 1: {}", sum);
        assert_eq!(sum, 157);

        Ok(())
    }
}
