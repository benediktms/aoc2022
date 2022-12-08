use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn check_against_alphabet(
    item: &char,
    comparitor: &Vec<char>,
    additional_offset: Option<i32>,
) -> i32 {
    let mut sum = 0;
    let index = comparitor.iter().position(|char| char == item);
    let offset = match additional_offset {
        Some(num) => num as i32,
        None => 0,
    };

    match index {
        Some(value) => {
            let priority = (value as i32) + 1 + offset;
            sum += priority as i32
        }
        None => sum += 0,
    };

    sum
}

fn get_alphabets() -> (Vec<char>, Vec<char>) {
    let lowercase_range = b'a'..=b'z';
    let uppercase_range = b'A'..=b'Z';

    let lowercase_vec = lowercase_range
        .map(|i| (i as char).to_ascii_lowercase())
        .collect_vec();

    let uppercase_vec = uppercase_range
        .map(|i| (i as char).to_ascii_uppercase())
        .collect_vec();

    (lowercase_vec, uppercase_vec)
}

fn get_item_priorities(backpack: (&str, &str)) -> i32 {
    let (lowercase_vec, uppercase_vec) = get_alphabets();

    let first_compartment = backpack.0.chars().collect_vec();
    let second_compartment = backpack.1.chars().collect_vec();
    let mut is_duplicate = false;

    let mut sum = 0;

    for outer_item in &first_compartment {
        for inner_item in &second_compartment {
            is_duplicate = outer_item == inner_item;

            if is_duplicate {
                sum += check_against_alphabet(inner_item, &lowercase_vec, None);
                sum += check_against_alphabet(inner_item, &uppercase_vec, Some(26));

                break;
            }
        }

        if is_duplicate {
            break;
        }
    }

    sum
}

// probably this can be refactored in a way so that everything can be done in `get_item_priorities`
fn caculate_total_of_badges_in_groups(lines: Vec<String>) -> i32 {
    let (lowercase_vec, uppercase_vec) = get_alphabets();

    let mut i = 0;
    let mut score = 0;

    while i < lines.len() {
        let row_one = &lines[i].chars().collect_vec();
        i += 1;

        let row_two = &lines[i].chars().collect_vec();
        i += 1;

        let row_three = &lines[i].chars().collect_vec();
        i += 1;

        let mut is_duplicate = false;

        for item_one in row_one {
            for item_two in row_two {
                if item_one == item_two {
                    for item_three in row_three {
                        if item_three == item_one {
                            is_duplicate = true;
                        }

                        if is_duplicate {
                            score += check_against_alphabet(item_three, &lowercase_vec, None);
                            score += check_against_alphabet(item_three, &uppercase_vec, Some(26));

                            break;
                        }
                    }
                }

                if is_duplicate {
                    break;
                }
            }

            if is_duplicate {
                break;
            }
        }
    }

    score
}

fn main() -> anyhow::Result<()> {
    let path = Path::new("input/day3/input1.txt");
    let file = File::open(path)?;

    let buf_reader = BufReader::new(file);
    let mut lines = buf_reader.lines();
    let mut sum = 0;
    let mut string_lines: Vec<String> = Vec::new();

    for line in lines.by_ref() {
        let line = line?;

        // empty slice array only used to get the length
        let split = line.split(|i: char| i.is_ascii_alphabetic()).collect_vec();

        let backpack_compartments = line.split_at((split.len() - 1) / 2);

        sum += get_item_priorities(backpack_compartments);
        string_lines.push(line);
    }

    let sum_of_badges = caculate_total_of_badges_in_groups(string_lines);

    println!("sum: {}", sum);
    assert_eq!(8123, sum);

    println!("sum of badges: {}", sum_of_badges);
    assert_eq!(2620, sum_of_badges);

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
        path::Path,
    };

    use crate::{caculate_total_of_badges_in_groups, get_item_priorities};

    #[test]
    fn should_cacluate_the_correct_value_from_test_input() -> anyhow::Result<()> {
        let path = Path::new("input/day3/test.txt");
        let file = File::open(path)?;

        let buf_reader = BufReader::new(file);
        let lines = buf_reader.lines();
        let mut string_lines: Vec<String> = Vec::new();

        let mut sum = 0;

        for line in lines {
            let line = line?;
            let split: Vec<&str> = line.split(|i: char| i.is_ascii_alphabetic()).collect();

            let backpack = line.split_at((split.len() - 1) / 2);

            sum += get_item_priorities(backpack);
            string_lines.push(line);
        }

        let sum_of_badges = caculate_total_of_badges_in_groups(string_lines);

        println!("sum: {}", sum);
        println!("sum of badges: {}", sum_of_badges);

        assert_eq!(sum, 157);
        assert_eq!(sum_of_badges, 70);

        Ok(())
    }
}
